use std::{f32::consts::TAU, time::Duration};

use avian2d::{math::PI, prelude::*};
use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_enoki::{
    EnokiPlugin, ParticleEffectHandle, ParticleSpawner,
    prelude::{OneShot, ParticleSpawnerState},
};
use rand::{Rng, thread_rng};

use crate::{Game, GameState, Player, game_canvas::GameCanvas, loading::ParticleAssets};

const _MAX_SCORE: u32 = 100;
const NUM_LIVES: u32 = 3;
const PIXEL_SCALE: f32 = 25.;

const SHIP_THRUST_MAGNITUDE: f32 = 100.;
const SHIP_MAX_VELOCITY: f32 = 750.;
const SHIP_ROTATION_SPEED: f32 = 0.5 * 2. * PI;
const SHIP_INVINCIBLE_TIME: f32 = 5.0;
const SHIP_BLINK_RATE: f32 = 50.0;

const BLASTER_COOLDOWN: f32 = 0.1;
const BULLET_TTL: f32 = 0.5;
const BULLET_RADIUS: f32 = 2.0;
const BULLET_SPEED: f32 = 1000.;

const BEEF_NUM_VERTS: u8 = 10;
const BEEF_RADIUS: f32 = 50.;
// percent of radius
const BEEF_RADIUS_VARIANCE: f32 = 0.25;

const SHIP_LAYER_MASK: u32 = 0b0001;
const BULLET_LAYER_MASK: u32 = 0b0010;
const BEEF_LAYER_MASK: u32 = 0b0100;

pub fn plugin(app: &mut App) {
    app.add_plugins((PhysicsPlugins::default(), EnokiPlugin))
        .insert_resource(Gravity::ZERO)
        .init_gizmo_group::<ShipGizmoGroup>()
        .insert_resource(BlasterCooldown(Timer::new(
            Duration::from_secs_f32(BLASTER_COOLDOWN),
            TimerMode::Once,
        )))
        .insert_resource(Lives(3))
        .insert_resource(Score(0))
        .add_sub_state::<BeefBlastoidsState>()
        .add_sub_state::<RunningState>()
        .add_systems(
            OnEnter(BeefBlastoidsState::Running),
            (spawn_beef, reset_game),
        )
        .add_systems(OnEnter(RunningState::SpawnShip), spawn_ship)
        .add_systems(
            Update,
            tick_invincibility.run_if(in_state(RunningState::ShipInvincible)),
        )
        .add_systems(OnEnter(RunningState::ShipDestroyed), destroy_ship)
        .add_systems(
            Update,
            handle_ship_particles.run_if(in_state(RunningState::ShipDestroyed)),
        )
        // .add_systems(
        //     FixedUpdate,
        //     (
        //         handle_screen_wrap,
        //         handle_collisions,
        //         check_bullets_ttl,
        //         tick_blaster_cooldown,
        //     )
        //         .chain()
        //         .run_if(in_state(GameState::Playing(Game::BeefBlastoids))),
        // )
        .add_systems(
            Update,
            (
                handle_screen_wrap,
                handle_collisions,
                check_bullets_ttl,
                tick_blaster_cooldown,
            )
                .chain()
                .run_if(in_state(GameState::Playing(Game::BeefBlastoids))),
        )
        .add_observer(apply_rotation)
        .add_observer(apply_thrust)
        .add_observer(shoot_blaster);
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing(Game::BeefBlastoids))]
#[states(scoped_entities)]
pub(crate) enum BeefBlastoidsState {
    #[default]
    Running,
    _GameOver,
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(BeefBlastoidsState = BeefBlastoidsState::Running)]
#[states(scoped_entities)]
pub(crate) enum RunningState {
    #[default]
    SpawnShip,
    ShipInvincible,
    Normal,
    ShipDestroyed,
}

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
struct Lives(u32);

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
struct Score(u32);

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct BlasterCooldown(Timer);

#[derive(InputAction)]
#[action_output(bool)]
struct Thrust;

#[derive(InputAction)]
#[action_output(f32)]
struct Rotate;

#[derive(InputAction)]
#[action_output(bool)]
struct Shoot;

#[derive(Component)]
struct ScreenWrap;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Bullet(f32);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Beef;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ShipExplosion;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Invincible {
    invincibility_timer: Timer,
    blink_rate: f32,
}

fn reset_game(mut score: ResMut<Score>, mut lives: ResMut<Lives>) {
    **score = 0;
    **lives = NUM_LIVES;
}

fn ship_gizmo_and_verts(visible: bool) -> (GizmoAsset, [Vec2; 3]) {
    let mut ship = GizmoAsset::default();

    let verts = [
        Vec2::new(-0.5, -0.5) * PIXEL_SCALE,
        Vec2::new(0., 1.0) * PIXEL_SCALE,
        Vec2::new(0.5, -0.5) * PIXEL_SCALE,
    ];

    ship.primitive_2d(
        &Triangle2d::new(verts[0], verts[1], verts[2]),
        Isometry2d::from_xy(0.0, 0.0),
        Color::srgba(1.0, 1.0, 1.0, if visible { 1.0 } else { 0.0 }),
    );

    (ship, verts)
}

fn spawn_ship(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    canvas: Single<Entity, With<GameCanvas>>,
    mut next_state: ResMut<NextState<RunningState>>,
) {
    info!("spawning ship");

    let (ship, verts) = ship_gizmo_and_verts(true);

    commands.spawn((
        Player,
        Name::new("Ship"),
        ScreenWrap,
        Invincible {
            invincibility_timer: Timer::from_seconds(SHIP_INVINCIBLE_TIME, TimerMode::Once),
            blink_rate: SHIP_BLINK_RATE,
        },
        Visibility::Hidden,
        actions!(Player[
            (
                Action::<Thrust>::new(),
                bindings![
                    (KeyCode::KeyW),
                    (KeyCode::ArrowUp)
                ]
            ),
            (
                Action::<Rotate>::new(),
                bindings![
                    (KeyCode::KeyA),
                    (KeyCode::KeyD, Negate::all()),
                    (KeyCode::ArrowLeft),
                    (KeyCode::ArrowRight,Negate::all()),

                ]
            ),
            (
                Action::<Shoot>::new(),
                Press::new(1.0),
                bindings![
                    (KeyCode::Space),
                ]
            ),
        ]),
        Gizmo {
            handle: gizmo_assets.add(ship),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::triangle(verts[0], verts[1], verts[2]),
        CollisionMargin(0.1),
        SweptCcd::default(),
        CollisionLayers::new(SHIP_LAYER_MASK, BEEF_LAYER_MASK | BULLET_LAYER_MASK),
        CollisionEventsEnabled,
        ChildOf(*canvas),
        StateScoped(BeefBlastoidsState::Running),
    ));

    next_state.set(RunningState::ShipInvincible);
}

fn spawn_beef(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    canvas_query: Single<(Entity, &GameCanvas)>,
) {
    let num_beef = 1;

    let (canvas_entity, canvas) = *canvas_query;

    let mut rng = thread_rng();

    for _ in 0..num_beef {
        let beef = generate_beef(BEEF_RADIUS);

        let mut collider_indices = vec![];

        for i in 0..(beef.vertices.len() - 2) {
            collider_indices.push([i as u32, i as u32 + 1]);
        }
        collider_indices.push([beef.vertices.len() as u32 - 1, 0]);

        let mut gizmo = GizmoAsset::default();

        gizmo.primitive_2d(
            &beef,
            Isometry2d::from_translation(Vec2::ZERO),
            Color::srgba(1.0, 1.0, 1.0, 1.0),
        );

        commands.spawn((
            Beef,
            Name::new("Beef"),
            ScreenWrap,
            Gizmo {
                handle: gizmo_assets.add(gizmo),
                ..default()
            },
            RigidBody::Kinematic,
            SweptCcd::default(),
            LinearVelocity(vec2(rng.gen_range(-10.0..10.), rng.gen_range(-10.0..10.))),
            AngularVelocity(rng.gen_range((-TAU / 10.)..(TAU / 10.))),
            Transform::from_xyz(
                rng.gen_range((-canvas.width() / 2.)..(canvas.width() / 2.)),
                rng.gen_range((-canvas.height() / 2.)..(canvas.height() / 2.)),
                0.,
            ),
            Collider::convex_decomposition_with_config(
                beef.vertices.to_vec(),
                collider_indices,
                &VhacdParameters {
                    concavity: 1.0,
                    alpha: 0.05,
                    beta: 0.05,
                    resolution: 256,
                    plane_downsampling: 4,
                    convex_hull_downsampling: 4,
                    convex_hull_approximation: true,
                    max_convex_hulls: 1024,
                    ..Default::default()
                },
            ),
            CollisionMargin(0.1),
            DebugRender::default().with_collider_color(Color::from(RED)),
            CollisionLayers::new(BEEF_LAYER_MASK, SHIP_LAYER_MASK | BULLET_LAYER_MASK),
            ChildOf(canvas_entity),
            StateScoped(BeefBlastoidsState::Running),
        ));
    }
}

fn shoot_blaster(
    _trigger: Trigger<Fired<Shoot>>,
    mut commands: Commands,
    mut cooldown: ResMut<BlasterCooldown>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    canvas: Single<Entity, With<GameCanvas>>,
    time: Res<Time>,
    ship: Single<(&Position, &Rotation), With<Player>>,
) {
    // info!("shoot: {}", trigger.value);

    if cooldown.0.finished() {
        let (ship_position, ship_rotation) = *ship;

        commands.spawn((
            Bullet(time.elapsed_secs()),
            ScreenWrap,
            Name::new("Bullet"),
            Mesh2d(meshes.add(Circle::new(BULLET_RADIUS))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
            RigidBody::Kinematic,
            SweptCcd::default(),
            Collider::circle(BULLET_RADIUS),
            CollisionMargin(0.1),
            LinearVelocity(BULLET_SPEED * (ship_rotation * Vec2::Y)),
            Position(**ship_position),
            CollisionLayers::new(BULLET_LAYER_MASK, BEEF_LAYER_MASK),
            CollisionEventsEnabled,
            StateScoped(BeefBlastoidsState::Running),
            ChildOf(*canvas),
        ));

        cooldown.0.reset();
    }
}

fn handle_screen_wrap(
    wrapping_transforms: Query<&mut Transform, (With<ScreenWrap>, Without<GameCanvas>)>,
    canvas: Single<&GameCanvas>,
) {
    // TODO put this into a resource during startup

    for mut transform in wrapping_transforms {
        let t = &mut transform.translation;

        if t.x < -canvas.width() / 2. || t.x > canvas.width() / 2. {
            t.x *= -1.;
        }

        if t.y < -canvas.height() / 2. || t.y > canvas.height() / 2. {
            t.y *= -1.;
        }
    }
}

fn apply_thrust(
    _trigger: Trigger<Fired<Thrust>>,
    time: Res<Time>,
    query: Single<(&mut LinearVelocity, &Rotation), With<Player>>,
) {
    // info!("thrust: {}", trigger.value);

    let (mut velocity, rotation) = query.into_inner();
    let local_up = rotation * Vec2::Y;
    let delta_velocity = local_up * SHIP_THRUST_MAGNITUDE * time.delta_secs();

    velocity.0 += delta_velocity;
    velocity.0 = velocity.clamp(
        Vec2::splat(-SHIP_MAX_VELOCITY),
        Vec2::splat(SHIP_MAX_VELOCITY),
    );
}

fn apply_rotation(
    trigger: Trigger<Fired<Rotate>>,
    mut rotation: Single<&mut Rotation, With<Player>>,
    time: Res<Time>,
) {
    // info!("rotation: {}", trigger.value);

    **rotation = rotation.add_angle(trigger.value * SHIP_ROTATION_SPEED * time.delta_secs());
}

fn tick_blaster_cooldown(mut cooldown: ResMut<BlasterCooldown>, time: Res<Time>) {
    cooldown.0.tick(Duration::from_secs_f32(time.delta_secs()));
}

fn check_bullets_ttl(mut commands: Commands, bullets: Query<(Entity, &Bullet)>, time: Res<Time>) {
    let current_time = time.elapsed_secs();

    for (entity, bullet) in bullets {
        if current_time - bullet.0 > BULLET_TTL {
            commands.entity(entity).despawn();
        }
    }
}

fn generate_beef(radius: f32) -> BoxedPolygon {
    let mut rng = thread_rng();

    BoxedPolygon::new((0..BEEF_NUM_VERTS).map(|i| {
        let theta = (i as f32) / (BEEF_NUM_VERTS as f32) * TAU;

        let radius = radius * (1. + rng.gen_range(-BEEF_RADIUS_VARIANCE..BEEF_RADIUS_VARIANCE));

        let x = f32::cos(theta) * radius;
        let y = f32::sin(theta) * radius;

        Vec2::new(x, y)
    }))
}

fn handle_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    bullets_query: Query<&Bullet>,
    player_query: Single<(Entity, &Player, Option<&Invincible>)>,
    beef_query: Query<(Entity, &Beef)>,
    mut next_state: ResMut<NextState<RunningState>>,
) {
    let player_entity = player_query.0;
    let player_invincible = player_query.2.is_some();

    for collision in collision_event_reader.read() {
        info!("{collision:?}");

        let this_collider_entity = collision.0;
        let other_collider_entity = collision.1;

        if this_collider_entity == player_entity && beef_query.contains(other_collider_entity) {
            info!("Ship hit a beef!");

            if !player_invincible {
                // Destroy ship
                next_state.set(RunningState::ShipDestroyed);
            }
        } else if bullets_query.contains(this_collider_entity)
            && beef_query.contains(other_collider_entity)
        {
            info!("Bullet hit a beef!");
        }
    }
}

fn destroy_ship(
    mut commands: Commands,
    particle_assets: Res<ParticleAssets>,
    mut lives: ResMut<Lives>,
    mut next_state_bb: ResMut<NextState<BeefBlastoidsState>>,
    mut next_state_running: ResMut<NextState<RunningState>>,
    ship: Single<(Entity, &Transform), With<Player>>,
    canvas: Single<Entity, With<GameCanvas>>,
) {
    let (ship_entity, ship_transform) = ship.into_inner();

    // spawn explosion particles
    commands.spawn((
        ShipExplosion,
        ParticleSpawner::default(),
        ParticleEffectHandle(particle_assets.beef_blastoids_explosion.clone()),
        OneShot::Deactivate,
        Transform::from_translation(ship_transform.translation),
        ChildOf(*canvas),
        Name::new("Ship Explosion"),
        StateScoped(RunningState::ShipDestroyed),
    ));

    if **lives == 1 {
        next_state_bb.set(BeefBlastoidsState::_GameOver);
    } else {
        **lives -= 1;
        // next_state_running.set(RunningState::SpawnShip);
    }

    commands.entity(ship_entity).despawn();
}

fn handle_ship_particles(
    mut next_state: ResMut<NextState<RunningState>>,
    explosion_state: Single<&ParticleSpawnerState, With<ShipExplosion>>,
) {
    if !explosion_state.active {
        next_state.set(RunningState::SpawnShip);
    }
}

fn tick_invincibility(
    mut commands: Commands,
    ship: Single<(Entity, &mut Invincible, &mut Gizmo), With<Player>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<RunningState>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let (entity, mut invincible, mut gizmo_component) = ship.into_inner();

    invincible.invincibility_timer.tick(time.delta());

    let blink_visibility = (time.elapsed_secs() * SHIP_BLINK_RATE).sin() > 0.0;
    // if let Some(gizmo_asset) = gizmo_assets.get_mut(&handle) {
    let (ship_gizmo_asset, _) =
        ship_gizmo_and_verts(if invincible.invincibility_timer.finished() {
            commands.entity(entity).remove::<Invincible>();
            next_state.set(RunningState::Normal);

            true
        } else {
            blink_visibility
        });

    gizmo_component.handle = gizmo_assets.add(ship_gizmo_asset);
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct ShipGizmoGroup;
