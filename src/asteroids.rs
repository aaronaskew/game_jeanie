use avian2d::{math::PI, prelude::*};
use bevy::{
    color::palettes::css::WHITE, dev_tools::states::log_transitions, prelude::*,
    window::PrimaryWindow,
};
use bevy_enhanced_input::prelude::*;

use crate::{Game, GameState, Player};

const _MAX_SCORE: u32 = 1;
const _NUM_LIVES: u32 = 3;
const PIXEL_SCALE: f32 = 25.;
const THRUST_MAGNITUDE: f32 = 100.;
const ROTATION_SPEED: f32 = 0.5 * 2. * PI;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing(Game::Asteroids))]
#[states(scoped_entities)]
pub(crate) enum AsteroidsState {
    #[default]
    Running,
    _GameOver,
}

#[derive(Resource, Deref)]
struct _Lives(u32);

pub(crate) struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity::ZERO)
            .add_sub_state::<AsteroidsState>()
            .add_systems(Update, log_transitions::<AsteroidsState>)
            .add_systems(OnEnter(AsteroidsState::Running), spawn_ship)
            .add_systems(
                FixedUpdate,
                handle_screen_wrap.run_if(in_state(GameState::Playing(Game::Asteroids))),
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
    transforms: Query<&mut Transform, With<ScreenWrap>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let width = window.resolution.width();
    let height = window.resolution.height();

    for mut transform in transforms {
        let t = &mut transform.translation;

        if t.x < -width / 2.0 || t.x > width / 2.0 {
            t.x *= -1.0;
        }

        if t.y < -height / 2.0 || t.y > height / 2.0 {
            t.y *= -1.0;
        }
    }
}

fn spawn_ship(mut commands: Commands, mut gizmo_assets: ResMut<Assets<GizmoAsset>>) {
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
        // ExternalForce::ZERO.with_persistence(false),
        Collider::triangle(verts[0], verts[1], verts[2]),
        StateScoped(AsteroidsState::Running),
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

    let new_velocity = local_up * THRUST_MAGNITUDE * time.delta_secs();

    velocity.0 += new_velocity;
}

fn apply_rotation(
    trigger: Trigger<Fired<Rotate>>,
    rotation: Single<&mut Rotation, With<Player>>,
    time: Res<Time>,
) {
    info!("rotation: {}", trigger.value);

    let mut rotation = rotation.into_inner();

    *rotation = rotation.add_angle(trigger.value * ROTATION_SPEED * time.delta_secs());
}

fn teleport(trigger: Trigger<Fired<Teleport>>) {
    info!("teleport: {}", trigger.value);
}
