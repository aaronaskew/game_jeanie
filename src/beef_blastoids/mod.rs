use std::{f32::consts::TAU, time::Duration};

use avian2d::{math::PI, prelude::*};
use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_enoki::{EnokiPlugin, ParticleEffectHandle, ParticleSpawner, prelude::OneShot};
use rand::{Rng, thread_rng};

mod ui;

use crate::{Game, GameState, Player, game_canvas::GameCanvas, loading::ParticleAssets};

const MAX_SCORE: u32 = 10000;
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

const INITIAL_NUM_BEEF: u32 = 0;
const BEEF_NUM_VERTS: u8 = 10;
const BEEF_RADIUS: f32 = 50.;
// percent of radius
const BEEF_RADIUS_VARIANCE: f32 = 0.25;
const BEEF_SCORE_VALUE: u32 = 100;

const SHIP_LAYER_MASK: u32 = 0b0001;
const BULLET_LAYER_MASK: u32 = 0b0010;
const BEEF_LAYER_MASK: u32 = 0b0100;

pub fn plugin(app: &mut App) {
    app.add_plugins(ui::plugin)
        .add_plugins((PhysicsPlugins::default(), EnokiPlugin))
        .insert_resource(Gravity::ZERO)
        .init_gizmo_group::<ShipGizmoGroup>()
        .insert_resource(BlasterCooldown(Timer::new(
            Duration::from_secs_f32(BLASTER_COOLDOWN),
            TimerMode::Once,
        )))
        .insert_resource(Lives(3))
        .insert_resource(Score(0))
        .insert_resource(NumBeef(INITIAL_NUM_BEEF))
        .add_sub_state::<BeefBlastoidsState>()
        .add_sub_state::<RunningState>()
        .add_systems(OnEnter(BeefBlastoidsState::Running), reset_game)
        .add_systems(OnEnter(RunningState::NextLevel), next_level)
        .add_systems(OnEnter(RunningState::SpawnBeef), spawn_beef)
        .add_systems(OnEnter(RunningState::SpawnShip), spawn_ship)
        .add_systems(
            Update,
            tick_invincibility.run_if(in_state(RunningState::ShipInvincible)),
        )
        .add_systems(OnEnter(RunningState::ShipDestroyed), destroy_ship)
        .add_systems(
            Update,
            respawn_timer.run_if(in_state(RunningState::ShipDestroyed)),
        )
        .add_systems(
            Update,
            destroy_beef.run_if(in_state(BeefBlastoidsState::Running)),
        )
        .add_systems(
            Update,
            (
                handle_screen_wrap,
                handle_collisions,
                check_bullets_ttl,
                tick_blaster_cooldown,
                game_over_check,
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
    GameOver,
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(BeefBlastoidsState = BeefBlastoidsState::Running)]
#[states(scoped_entities)]
pub(crate) enum RunningState {
    #[default]
    NextLevel,
    SpawnBeef,
    SpawnShip,
    ShipInvincible,
    ShipDestroyed,
    Normal,
}

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
struct Lives(u32);

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
struct Score(u32);

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
struct NumBeef(u32);

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

#[derive(Component, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component)]
enum BeefSize {
    Large,
    Medium,
    Small,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct DestroyBeef;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Invincible {
    invincibility_timer: Timer,
    blink_rate: f32,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct ShipGizmoGroup;

#[derive(Bundle)]
struct BeefBundle {
    beef: Beef,
    beef_size: BeefSize,
    name: Name,
    screen_wrap: ScreenWrap,
    gizmo: Gizmo,
    rb: RigidBody,
    sweptccd: SweptCcd,
    l_vel: LinearVelocity,
    a_vel: AngularVelocity,
    transform: Transform,
    collider: Collider,
    collision_margin: CollisionMargin,
    debug_render: DebugRender,
    collision_layers: CollisionLayers,
    child_of: ChildOf,
    state_scoped: StateScoped<BeefBlastoidsState>,
}

fn reset_game(mut score: ResMut<Score>, mut lives: ResMut<Lives>, mut num_beef: ResMut<NumBeef>) {
    **score = 0;
    **lives = NUM_LIVES;
    **num_beef = INITIAL_NUM_BEEF;
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
        CollidingEntities::default(),
        ChildOf(*canvas),
        StateScoped(BeefBlastoidsState::Running),
    ));

    next_state.set(RunningState::ShipInvincible);
}

fn generate_beef_polygon(radius: f32) -> BoxedPolygon {
    let mut rng = thread_rng();

    BoxedPolygon::new((0..BEEF_NUM_VERTS).map(|i| {
        let theta = (i as f32) / (BEEF_NUM_VERTS as f32) * TAU;

        let radius = radius * (1. + rng.gen_range(-BEEF_RADIUS_VARIANCE..BEEF_RADIUS_VARIANCE));

        let x = f32::cos(theta) * radius;
        let y = f32::sin(theta) * radius;

        Vec2::new(x, y)
    }))
}

fn generate_beef_bundle(
    beef_size: BeefSize,
    canvas_entity: Entity,
    gizmo_assets: &mut ResMut<Assets<GizmoAsset>>,
    translation: Vec3,
    linear_velocity: Vec2,
    angular_velocity: f32,
) -> BeefBundle {
    let beef = generate_beef_polygon(match beef_size {
        BeefSize::Large => BEEF_RADIUS,
        BeefSize::Medium => BEEF_RADIUS * 0.5,
        BeefSize::Small => BEEF_RADIUS * 0.25,
    });

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

    BeefBundle {
        beef: Beef,
        beef_size,
        name: Name::new("Beef"),
        screen_wrap: ScreenWrap,
        gizmo: Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        },
        rb: RigidBody::Kinematic,
        sweptccd: SweptCcd::default(),
        l_vel: LinearVelocity(linear_velocity),
        a_vel: AngularVelocity(angular_velocity),
        transform: Transform::from_translation(translation),
        collider: Collider::convex_decomposition_with_config(
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
        collision_margin: CollisionMargin(0.1),
        debug_render: DebugRender::default().with_collider_color(Color::from(RED)),
        collision_layers: CollisionLayers::new(
            BEEF_LAYER_MASK,
            SHIP_LAYER_MASK | BULLET_LAYER_MASK,
        ),
        child_of: ChildOf(canvas_entity),
        state_scoped: StateScoped(BeefBlastoidsState::Running),
    }
}

fn spawn_beef(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    canvas_query: Single<(Entity, &GameCanvas)>,
    num_beef: Res<NumBeef>,
    mut next_state: ResMut<NextState<RunningState>>,
) {
    let (canvas_entity, canvas) = *canvas_query;

    let mut rng = thread_rng();

    for _ in 0..**num_beef {
        let translation = Vec3::new(
            rng.gen_range((-canvas.width() / 2.)..(canvas.width() / 2.)),
            rng.gen_range((-canvas.height() / 2.)..(canvas.height() / 2.)),
            0.,
        );

        let linear_velocity = vec2(rng.gen_range(-10.0..10.), rng.gen_range(-10.0..10.));

        let angular_velocity = rng.gen_range((-TAU / 10.)..(TAU / 10.));

        commands.spawn(generate_beef_bundle(
            BeefSize::Large,
            canvas_entity,
            &mut gizmo_assets,
            translation,
            linear_velocity,
            angular_velocity,
        ));
    }

    next_state.set(RunningState::SpawnShip);
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
            CollidingEntities::default(),
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

fn next_level(
    mut num_beef: ResMut<NumBeef>,
    mut next_state: ResMut<NextState<RunningState>>,
    mut commands: Commands,
    ship: Option<Single<Entity, With<Player>>>,
) {
    **num_beef += 2;

    next_state.set(RunningState::SpawnBeef);

    if let Some(ship) = ship {
        commands.entity(*ship).despawn();
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

fn handle_collisions(
    bullets_query: Query<(Entity, &CollidingEntities), With<Bullet>>,
    player_query: Single<(Entity, &Player, Option<&Invincible>, &CollidingEntities)>,
    beef_query: Query<(Entity, &Beef)>,
    mut next_state: ResMut<NextState<RunningState>>,
    mut commands: Commands,
) {
    let player_invincible = player_query.2.is_some();
    let player_colliding_entities = player_query.3;

    if !player_colliding_entities.is_empty()
        && player_colliding_entities
            .iter()
            .any(|entity| beef_query.contains(*entity))
        && !player_invincible
    {
        // Destroy ship
        next_state.set(RunningState::ShipDestroyed);
    }

    for (bullet_entity, bullet_colliding_entities) in bullets_query {
        if !bullet_colliding_entities.is_empty() {
            for entity in bullet_colliding_entities.iter() {
                if beef_query.contains(*entity) {
                    commands.entity(*entity).insert(DestroyBeef);
                }
            }

            commands.entity(bullet_entity).despawn();
        }
    }
}

fn destroy_beef(
    destroy_beef_query: Query<
        (
            Entity,
            &Transform,
            &LinearVelocity,
            &AngularVelocity,
            &BeefSize,
        ),
        With<DestroyBeef>,
    >,
    all_beef_query: Query<&Beef>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<RunningState>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    particle_assets: Res<ParticleAssets>,
    canvas_query: Single<(Entity, &GameCanvas)>,
    mut score: ResMut<Score>,
) {
    let (canvas_entity, canvas) = *canvas_query;

    // Check if this was the last beef
    if all_beef_query.iter().len() == 1
        && let Some(last_beef) = destroy_beef_query.iter().next()
        && *last_beef.4 == BeefSize::Small
    {
        next_state.set(RunningState::NextLevel);
    }

    for (entity, transform, linear_velocity, angular_velocity, beef_size) in destroy_beef_query {
        let radius = if *beef_size == BeefSize::Large {
            BEEF_RADIUS * 0.5
        } else {
            BEEF_RADIUS * 0.25
        };

        let perpendicular_1 = Vec2::new(-linear_velocity.y, linear_velocity.x)
            .normalize()
            .extend(0.);
        let perpendicular_2 = Vec2::new(linear_velocity.y, -linear_velocity.x)
            .normalize()
            .extend(0.);

        let canvas_min = Vec2::new(-canvas.width() / 2.0, -canvas.height() / 2.0).extend(0.);
        let canvas_max = Vec2::new(canvas.width() / 2.0, canvas.height() / 2.0).extend(0.);

        let translation_1 =
            (transform.translation + (perpendicular_1 * radius)).clamp(canvas_min, canvas_max);
        let translation_2 =
            (transform.translation + (perpendicular_2 * radius)).clamp(canvas_min, canvas_max);

        let lin_vel1 = linear_velocity.0 + 5.0 * linear_velocity.0 * perpendicular_1.truncate();
        let lin_vel2 = linear_velocity.0 + 5.0 * linear_velocity.0 * perpendicular_2.truncate();

        // TODO: Magic number
        let ang_vel1 = angular_velocity.0 * 1.2;
        let ang_vel2 = -ang_vel1;

        if matches!(beef_size, BeefSize::Large | BeefSize::Medium) {
            commands.spawn(generate_beef_bundle(
                match beef_size {
                    BeefSize::Large => BeefSize::Medium,
                    BeefSize::Medium => BeefSize::Small,
                    BeefSize::Small => panic!(),
                },
                canvas_entity,
                &mut gizmo_assets,
                translation_1,
                lin_vel1,
                ang_vel1,
            ));

            commands.spawn(generate_beef_bundle(
                match beef_size {
                    BeefSize::Large => BeefSize::Medium,
                    BeefSize::Medium => BeefSize::Small,
                    BeefSize::Small => panic!(),
                },
                canvas_entity,
                &mut gizmo_assets,
                translation_2,
                lin_vel2,
                ang_vel2,
            ));
        }

        **score += BEEF_SCORE_VALUE;

        commands.spawn((
            ParticleSpawner::default(),
            ParticleEffectHandle(particle_assets.beef_blastoids_beef_explosion.clone()),
            OneShot::Despawn,
            Transform::from_translation(transform.translation),
            ChildOf(canvas_entity),
            Name::new("Ship Explosion"),
            StateScoped(RunningState::ShipDestroyed),
        ));

        commands.entity(entity).despawn();
    }
}

fn destroy_ship(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    ship: Single<(Entity, &Transform), With<Player>>,
    particle_assets: Res<ParticleAssets>,
    canvas: Single<Entity, With<GameCanvas>>,
) {
    let (ship_entity, ship_transform) = ship.into_inner();

    // spawn explosion particles
    commands.spawn((
        ParticleSpawner::default(),
        ParticleEffectHandle(particle_assets.beef_blastoids_ship_explosion.clone()),
        OneShot::Despawn,
        Transform::from_translation(ship_transform.translation),
        ChildOf(*canvas),
        Name::new("Ship Explosion"),
        StateScoped(RunningState::ShipDestroyed),
    ));

    **lives -= 1;

    commands.entity(ship_entity).despawn();
}

fn respawn_timer(
    mut next_state: ResMut<NextState<RunningState>>,
    mut respawn_timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    if respawn_timer.is_some() {
        respawn_timer.as_mut().unwrap().tick(time.delta());

        if respawn_timer.as_ref().unwrap().finished() {
            next_state.set(RunningState::SpawnShip);
            *respawn_timer = None;
        }
    } else {
        *respawn_timer = Some(Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once));
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

fn game_over_check(
    score: Res<Score>,
    lives: Res<Lives>,
    mut next_state: ResMut<NextState<BeefBlastoidsState>>,
) {
    if **score >= MAX_SCORE {
        next_state.set(BeefBlastoidsState::GameOver);
    }

    if **lives == 0 {
        next_state.set(BeefBlastoidsState::GameOver);
    }
}
