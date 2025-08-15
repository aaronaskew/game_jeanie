use std::{f32::consts::TAU, time::Duration};

use avian2d::{math::PI, prelude::*};
use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use rand::{Rng, thread_rng};

use crate::{Game, GameState, Player, game_canvas::GameCanvas};

const _MAX_SCORE: u32 = 1;
const _NUM_LIVES: u32 = 3;
const PIXEL_SCALE: f32 = 25.;

const SHIP_THRUST_MAGNITUDE: f32 = 100.;
const SHIP_MAX_VELOCITY: f32 = 750.;
const SHIP_ROTATION_SPEED: f32 = 0.5 * 2. * PI;

const BLASTER_COOLDOWN: f32 = 0.1;
const BULLET_TTL: f32 = 1.0;
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
    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO)
        .insert_resource(BlasterCooldown(Timer::new(
            Duration::from_secs_f32(BLASTER_COOLDOWN),
            TimerMode::Once,
        )))
        .add_sub_state::<BeefBlastoidsState>()
        .add_systems(
            OnEnter(BeefBlastoidsState::Running),
            (spawn_ship, spawn_beef),
        )
        .add_systems(
            FixedUpdate,
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

#[derive(Resource, Deref)]
struct _Lives(u32);

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

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct BlasterCooldown(Timer);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Bullet(f32);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Beef;

fn spawn_ship(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    canvas: Single<Entity, With<GameCanvas>>,
) {
    let mut ship = GizmoAsset::default();

    let verts = [
        Vec2::new(-0.5, -0.5) * PIXEL_SCALE,
        Vec2::new(0., 1.0) * PIXEL_SCALE,
        Vec2::new(0.5, -0.5) * PIXEL_SCALE,
    ];

    ship.primitive_2d(
        &Triangle2d::new(verts[0], verts[1], verts[2]),
        Isometry2d::from_xy(0.0, 0.0),
        WHITE,
    );

    info!("spawning ship");

    commands.spawn((
        Player,
        Name::new("Ship"),
        ScreenWrap,
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
        CollisionLayers::new(SHIP_LAYER_MASK, BEEF_LAYER_MASK | BULLET_LAYER_MASK),
        CollisionEventsEnabled,
        ChildOf(*canvas),
        StateScoped(BeefBlastoidsState::Running),
    ));
}

fn spawn_beef(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    canvas_query: Single<(Entity, &GameCanvas)>,
) {
    let num_beef = 10;

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
            Color::srgba(0.0, 1.0, 0.0, 0.5),
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
            Collider::circle(BULLET_RADIUS),
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
    // mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    bullets_query: Query<&Bullet>,
    player_query: Single<(Entity, &Player)>,
    beef_query: Query<(Entity, &Beef)>,
) {
    let player_entity = player_query.0;

    for collision in collision_event_reader.read() {
        info!("{collision:?}");

        let this_collider_entity = collision.0;
        let other_collider_entity = collision.1;

        if this_collider_entity == player_entity && beef_query.contains(other_collider_entity) {
            info!("Player hit a beef!");
        }

        if bullets_query.contains(this_collider_entity)
            && beef_query.contains(other_collider_entity)
        {
            info!("Bullet hit a beef!");
        }
    }
}
