use std::f32::consts::TAU;

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
const THRUST_MAGNITUDE: f32 = 100.;
const ROTATION_SPEED: f32 = 0.5 * 2. * PI;

const SHIP_LAYER_MASK: u32 = 0b0001;
const BULLET_LAYER_MASK: u32 = 0b0010;
const BEEF_LAYER_MASK: u32 = 0b0100;

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

pub(crate) struct BeefBlastoidsPlugin;

impl Plugin for BeefBlastoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity::ZERO)
            .add_sub_state::<BeefBlastoidsState>()
            .add_systems(
                OnEnter(BeefBlastoidsState::Running),
                (spawn_ship, spawn_beef),
            )
            .add_systems(
                FixedUpdate,
                (handle_screen_wrap, handle_collisions)
                    .run_if(in_state(GameState::Playing(Game::BeefBlastoids))),
            )
            .add_observer(apply_rotation)
            .add_observer(apply_thrust)
            .add_observer(teleport);
    }
}

#[derive(InputAction)]
#[action_output(bool)]
struct Thrust;

#[derive(InputAction)]
#[action_output(f32)]
struct Rotate;

#[derive(InputAction)]
#[action_output(bool)]
struct Teleport;

#[derive(Component)]
struct ScreenWrap;

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
                Action::<Teleport>::new(),
                bindings![
                    (KeyCode::KeyS),
                    (KeyCode::ArrowDown)
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
        ChildOf(*canvas),
        StateScoped(BeefBlastoidsState::Running),
    ));
}

fn apply_thrust(
    trigger: Trigger<Fired<Thrust>>,
    time: Res<Time>,
    query: Single<(&mut LinearVelocity, &Rotation), With<Player>>,
) {
    info!("thrust: {}", trigger.value);

    let (mut velocity, rotation) = query.into_inner();

    let local_up = rotation * Vec2::Y;

    let delta_velocity = local_up * THRUST_MAGNITUDE * time.delta_secs();

    velocity.0 += delta_velocity;
}

fn apply_rotation(
    trigger: Trigger<Fired<Rotate>>,
    mut rotation: Single<&mut Rotation, With<Player>>,
    time: Res<Time>,
) {
    info!("rotation: {}", trigger.value);

    **rotation = rotation.add_angle(trigger.value * ROTATION_SPEED * time.delta_secs());
}

fn teleport(trigger: Trigger<Fired<Teleport>>) {
    info!("teleport: {}", trigger.value);
}

fn generate_beef() -> BoxedPolygon {
    let num_verts = 10;
    let radius = 50.;
    // percent of radius
    let radius_variance = 0.25;

    let mut rng = thread_rng();

    BoxedPolygon::new((0..num_verts).map(|i| {
        let theta = (i as f32) / (num_verts as f32) * TAU;

        let radius = radius * (1. + rng.gen_range(-radius_variance..radius_variance));

        let x = f32::cos(theta) * radius;
        let y = f32::sin(theta) * radius;

        // x += rng.gen_range(-vert_variance..vert_variance);
        // y += rng.gen_range(-vert_variance..vert_variance);

        // info!("theta:{theta} x:{x} y:{y}");

        Vec2::new(x, y)
    }))
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Beef;

fn spawn_beef(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    canvas_query: Single<(Entity, &GameCanvas)>,
) {
    let num_beef = 10;

    let (canvas_entity, canvas) = *canvas_query;

    let mut rng = thread_rng();

    for _ in 0..num_beef {
        let beef = generate_beef();

        dbg!(&beef.vertices);

        let mut collider_indices = vec![];

        for i in 0..(beef.vertices.len() - 2) {
            collider_indices.push([i as u32, i as u32 + 1]);
        }
        collider_indices.push([beef.vertices.len() as u32 - 1, 0]);

        dbg!(&collider_indices);

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
            RigidBody::Dynamic,
            LinearVelocity(vec2(rng.gen_range(-10.0..10.), rng.gen_range(-10.0..10.))),
            AngularVelocity(rng.gen_range((-TAU / 10.)..(TAU / 10.))),
            Transform::from_xyz(
                rng.gen_range((-canvas.width() / 2.)..(canvas.width() / 2.)),
                rng.gen_range((-canvas.height() / 2.)..(canvas.height() / 2.)),
                0.,
            ),
            // Collider::from(beef),

            // Collider::con
            // Collider::convex_decomposition(beef.vertices.to_vec(), collider_indices),
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
            // Collider::polyline(beef.vertices.to_vec(), Some(collider_indices)),
            CollisionLayers::new(BEEF_LAYER_MASK, SHIP_LAYER_MASK | BULLET_LAYER_MASK),
            ChildOf(canvas_entity),
            StateScoped(BeefBlastoidsState::Running),
        ));
    }
}

fn handle_collisions(mut collision_event_reader: EventReader<CollisionStarted>) {
    for collision_started in collision_event_reader.read() {
        info!("collision started: {collision_started:?}");
    }
}
