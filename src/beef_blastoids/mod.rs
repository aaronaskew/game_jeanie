use std::{f32::consts::TAU, ops::DerefMut, time::Duration};

use avian2d::{math::PI, prelude::*};
use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_enoki::{EnokiPlugin, ParticleEffectHandle, ParticleSpawner, prelude::OneShot};
use rand::{Rng, thread_rng};

mod ui;

use crate::{
    Game, GameOutcomes, GameResult, GameState, Player, RootNode, TvScreenActive, TvScreenSystems,
    game_canvas::GameCanvas,
    game_jeanie::{ActiveCheatCode, CheatCode, GameJeanieState},
    loading::ParticleAssets,
};

const PIXEL_SCALE: f32 = 25.;
const SHIP_LAYER_MASK: u32 = 0b0001;
const BULLET_LAYER_MASK: u32 = 0b0010;
const BEEF_LAYER_MASK: u32 = 0b0100;

pub fn plugin(app: &mut App) {
    app.add_plugins(ui::plugin)
        .add_plugins((PhysicsPlugins::default(), EnokiPlugin))
        .insert_resource(Gravity::ZERO)
        .add_computed_state::<BeefBlastoidsSetupGameState>()
        .init_gizmo_group::<ShipGizmoGroup>()
        .init_resource::<BeefBlastoidsGlobals>()
        .insert_resource(Lives(3))
        .insert_resource(Score(0))
        .add_sub_state::<BeefBlastoidsState>()
        .add_sub_state::<RunningState>()
        .add_systems(
            OnEnter(BeefBlastoidsState::SetupGlobals),
            setup_beef_blastoids_globals,
        )
        .add_systems(
            OnEnter(BeefBlastoidsSetupGameState),
            (initialize_resources, reset_game).chain(),
        )
        .add_systems(
            OnEnter(BeefBlastoidsSetupGameState),
            (|mut next_state: ResMut<NextState<RunningState>>| {
                next_state.set(RunningState::NextLevel)
            })
            .after(TvScreenSystems)
            .run_if(in_state(RunningState::CanvasInit)),
        )
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
                .run_if(in_state(BeefBlastoidsState::Running)),
        )
        .add_observer(apply_rotation)
        .add_observer(apply_thrust)
        .add_observer(shoot_blaster)
        .add_systems(OnEnter(BeefBlastoidsState::GameOver), game_over)
        .add_systems(
            Update,
            check_game_over_timer.run_if(in_state(BeefBlastoidsState::GameOver)),
        );
}

fn initialize_resources(mut commands: Commands, beef_blastoids_globals: Res<BeefBlastoidsGlobals>) {
    commands.insert_resource(BlasterCooldown(Timer::new(
        Duration::from_secs_f32(beef_blastoids_globals.BLASTER_COOLDOWN),
        TimerMode::Once,
    )));

    commands.insert_resource(NumBeef(beef_blastoids_globals.INITIAL_NUM_BEEF));
}

#[derive(Debug, Hash, Eq, Clone, PartialEq)]
struct BeefBlastoidsSetupGameState;

impl ComputedStates for BeefBlastoidsSetupGameState {
    type SourceStates = (BeefBlastoidsState, Option<TvScreenActive>);

    fn compute((beef_blastoids_state, tv_screen_active_state): Self::SourceStates) -> Option<Self> {
        match (beef_blastoids_state, tv_screen_active_state) {
            (BeefBlastoidsState::Running, Some(_)) => Some(Self),
            _ => None,
        }
    }
}

#[derive(Resource, Reflect, Debug, Default, Clone)]
#[reflect(Resource)]
#[allow(non_snake_case)]
pub struct BeefBlastoidsGlobals {
    pub DESCRIPTION: String,
    pub MAX_SCORE: u32,
    pub NUM_LIVES: u32,
    pub SHIP_THRUST_MAGNITUDE: f32,
    pub SHIP_MAX_VELOCITY: f32,
    pub SHIP_ROTATION_SPEED: f32,
    pub SHIP_INVINCIBLE_TIME: f32,
    pub SHIP_BLINK_RATE: f32,
    pub BLASTER_COOLDOWN: f32,
    pub BULLET_TTL: f32,
    pub BULLETS_DESPAWN: bool,
    pub BULLET_RADIUS: f32,
    pub BULLET_SPEED: f32,
    pub INITIAL_NUM_BEEF: u32,
    pub INITIAL_BEEF_SPEED: f32,
    pub BEEF_NUM_VERTS: u8,
    pub BEEF_RADIUS: f32,
    pub BEEF_RADIUS_VARIANCE: f32,
    pub BEEF_SCORE_VALUE: u32,
}

fn setup_beef_blastoids_globals(
    active_cheat_code: Res<ActiveCheatCode>,
    mut beef_blastoids_globals: ResMut<BeefBlastoidsGlobals>,
    mut next_state: ResMut<NextState<BeefBlastoidsState>>,
    game_jeanie_state: Res<State<GameJeanieState>>,
) -> Result {
    match **game_jeanie_state {
        GameJeanieState::Inactive => {
            *beef_blastoids_globals.deref_mut() = get_beef_blastoids_globals(&CheatCode::DEFAULT);
        }
        GameJeanieState::Active => {
            let game = active_cheat_code
                .game
                .as_ref()
                .ok_or("No game set in active cheat code")?;

            if *game != Game::BeefBlastoids {
                return Err(format!("active cheat code game set to {:?}", *game).into());
            }

            *beef_blastoids_globals.deref_mut() =
                if let Some(cheat_code) = active_cheat_code.cheat_code.as_ref() {
                    get_beef_blastoids_globals(cheat_code)
                } else {
                    get_beef_blastoids_globals(&CheatCode::DEFAULT)
                };
        }
    }

    #[cfg(debug_assertions)]
    {
        // *beef_blastoids_globals.deref_mut() = get_beef_blastoids_globals(&CheatCode::XXPHIHCS);
        // *beef_blastoids_globals.deref_mut() = get_beef_blastoids_globals(&CheatCode::PCLFZZOG);
    }

    info!("BeefBlastoidsGlobals: {:#?}", beef_blastoids_globals);

    next_state.set(BeefBlastoidsState::Running);

    Ok(())
}

fn get_beef_blastoids_globals(cheat_code: &CheatCode) -> BeefBlastoidsGlobals {
    match cheat_code {
        CheatCode::XXPHIHCS => BeefBlastoidsGlobals {
            DESCRIPTION: "High Ship Thrust, high beef speed".into(),
            MAX_SCORE: 10000,
            NUM_LIVES: 3,
            SHIP_THRUST_MAGNITUDE: 1000.,
            SHIP_MAX_VELOCITY: 7500.,
            SHIP_ROTATION_SPEED: 0.5 * 2. * PI,
            SHIP_INVINCIBLE_TIME: 5.0,
            SHIP_BLINK_RATE: 50.0,
            BLASTER_COOLDOWN: 0.1,
            BULLET_TTL: 0.5,
            BULLETS_DESPAWN: true,
            BULLET_RADIUS: 2.0,
            BULLET_SPEED: 1000.,
            INITIAL_NUM_BEEF: 0,
            INITIAL_BEEF_SPEED: 500.0,
            BEEF_NUM_VERTS: 10,
            BEEF_RADIUS: 50.,
            BEEF_RADIUS_VARIANCE: 0.25,
            BEEF_SCORE_VALUE: 100,
        },
        CheatCode::PCLFZZOG => BeefBlastoidsGlobals {
            DESCRIPTION: "Giant bullets".into(),
            MAX_SCORE: 10000,
            NUM_LIVES: 3,
            SHIP_THRUST_MAGNITUDE: 100.,
            SHIP_MAX_VELOCITY: 750.,
            SHIP_ROTATION_SPEED: 0.5 * 2. * PI,
            SHIP_INVINCIBLE_TIME: 5.0,
            SHIP_BLINK_RATE: 50.0,
            BLASTER_COOLDOWN: 0.1,
            BULLET_TTL: 0.5,
            BULLETS_DESPAWN: true,
            BULLET_RADIUS: 20.0,
            BULLET_SPEED: 1000.,
            INITIAL_NUM_BEEF: 0,
            INITIAL_BEEF_SPEED: 10.0,
            BEEF_NUM_VERTS: 10,
            BEEF_RADIUS: 50.,
            BEEF_RADIUS_VARIANCE: 0.25,
            BEEF_SCORE_VALUE: 100,
        },
        _ => BeefBlastoidsGlobals {
            DESCRIPTION: "Default".into(),
            MAX_SCORE: 10000,
            NUM_LIVES: 3,
            SHIP_THRUST_MAGNITUDE: 100.,
            SHIP_MAX_VELOCITY: 750.,
            SHIP_ROTATION_SPEED: 0.5 * 2. * PI,
            SHIP_INVINCIBLE_TIME: 5.0,
            SHIP_BLINK_RATE: 50.0,
            BLASTER_COOLDOWN: 0.1,
            BULLET_TTL: 0.5,
            BULLETS_DESPAWN: true,
            BULLET_RADIUS: 2.0,
            BULLET_SPEED: 1000.,
            INITIAL_BEEF_SPEED: 100.0,
            INITIAL_NUM_BEEF: 50,
            BEEF_NUM_VERTS: 10,
            BEEF_RADIUS: 50.,
            BEEF_RADIUS_VARIANCE: 0.25,
            BEEF_SCORE_VALUE: 100,
        },
    }
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing(Game::BeefBlastoids))]
#[states(scoped_entities)]
pub(crate) enum BeefBlastoidsState {
    #[default]
    SetupGlobals,
    Running,
    GameOver,
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(BeefBlastoidsState = BeefBlastoidsState::Running)]
#[states(scoped_entities)]
pub(crate) enum RunningState {
    #[default]
    CanvasInit,
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

fn reset_game(
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut num_beef: ResMut<NumBeef>,
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    **score = 0;
    **lives = beef_blastoids_globals.NUM_LIVES;
    **num_beef = beef_blastoids_globals.INITIAL_NUM_BEEF;
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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    info!("spawning ship");

    let (ship, verts) = ship_gizmo_and_verts(true);

    commands.spawn((
        Player,
        Name::new("Ship"),
        ScreenWrap,
        Invincible {
            invincibility_timer: Timer::from_seconds(
                beef_blastoids_globals.SHIP_INVINCIBLE_TIME,
                TimerMode::Once,
            ),
            blink_rate: beef_blastoids_globals.SHIP_BLINK_RATE,
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

fn generate_beef_polygon(
    radius: f32,
    beef_blastoids_globals: &Res<BeefBlastoidsGlobals>,
) -> BoxedPolygon {
    let mut rng = thread_rng();

    BoxedPolygon::new((0..beef_blastoids_globals.BEEF_NUM_VERTS).map(|i| {
        let theta = (i as f32) / (beef_blastoids_globals.BEEF_NUM_VERTS as f32) * TAU;

        let radius = radius
            * (1.
                + rng.gen_range(
                    -beef_blastoids_globals.BEEF_RADIUS_VARIANCE
                        ..beef_blastoids_globals.BEEF_RADIUS_VARIANCE,
                ));

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
    beef_blastoids_globals: &Res<BeefBlastoidsGlobals>,
) -> BeefBundle {
    let beef = generate_beef_polygon(
        match beef_size {
            BeefSize::Large => beef_blastoids_globals.BEEF_RADIUS,
            BeefSize::Medium => beef_blastoids_globals.BEEF_RADIUS * 0.5,
            BeefSize::Small => beef_blastoids_globals.BEEF_RADIUS * 0.25,
        },
        beef_blastoids_globals,
    );

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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    let (canvas_entity, canvas) = *canvas_query;

    let mut rng = thread_rng();

    for _ in 0..**num_beef {
        let translation = Vec3::new(
            rng.gen_range((-canvas.width() / 2.)..(canvas.width() / 2.)),
            rng.gen_range((-canvas.height() / 2.)..(canvas.height() / 2.)),
            0.,
        );

        let linear_velocity = vec2(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.))
            * beef_blastoids_globals.INITIAL_BEEF_SPEED;

        let angular_velocity = rng.gen_range((-TAU / 10.)..(TAU / 10.));

        commands.spawn(generate_beef_bundle(
            BeefSize::Large,
            canvas_entity,
            &mut gizmo_assets,
            translation,
            linear_velocity,
            angular_velocity,
            &beef_blastoids_globals,
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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    // info!("shoot: {}", trigger.value);

    if cooldown.0.finished() {
        let (ship_position, ship_rotation) = *ship;

        commands.spawn((
            Bullet(time.elapsed_secs()),
            ScreenWrap,
            Name::new("Bullet"),
            Mesh2d(meshes.add(Circle::new(beef_blastoids_globals.BULLET_RADIUS))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
            RigidBody::Kinematic,
            SweptCcd::default(),
            Collider::circle(beef_blastoids_globals.BULLET_RADIUS),
            CollisionMargin(0.1),
            LinearVelocity(beef_blastoids_globals.BULLET_SPEED * (ship_rotation * Vec2::Y)),
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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    // info!("thrust: {}", trigger.value);

    let (mut velocity, rotation) = query.into_inner();
    let local_up = rotation * Vec2::Y;
    let delta_velocity =
        local_up * beef_blastoids_globals.SHIP_THRUST_MAGNITUDE * time.delta_secs();

    velocity.0 += delta_velocity;
    velocity.0 = velocity.clamp(
        Vec2::splat(-beef_blastoids_globals.SHIP_MAX_VELOCITY),
        Vec2::splat(beef_blastoids_globals.SHIP_MAX_VELOCITY),
    );
}

fn apply_rotation(
    trigger: Trigger<Fired<Rotate>>,
    mut rotation: Single<&mut Rotation, With<Player>>,
    time: Res<Time>,
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    // info!("rotation: {}", trigger.value);

    **rotation = rotation
        .add_angle(trigger.value * beef_blastoids_globals.SHIP_ROTATION_SPEED * time.delta_secs());
}

fn tick_blaster_cooldown(mut cooldown: ResMut<BlasterCooldown>, time: Res<Time>) {
    cooldown.0.tick(Duration::from_secs_f32(time.delta_secs()));
}

fn check_bullets_ttl(
    mut commands: Commands,
    bullets: Query<(Entity, &Bullet)>,
    time: Res<Time>,
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    let current_time = time.elapsed_secs();

    for (entity, bullet) in bullets {
        if current_time - bullet.0 > beef_blastoids_globals.BULLET_TTL {
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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
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
            if beef_blastoids_globals.BULLETS_DESPAWN {
                commands.entity(bullet_entity).despawn();
            }
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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
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
            beef_blastoids_globals.BEEF_RADIUS * 0.5
        } else {
            beef_blastoids_globals.BEEF_RADIUS * 0.25
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
                &beef_blastoids_globals,
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
                &beef_blastoids_globals,
            ));
        }

        **score += beef_blastoids_globals.BEEF_SCORE_VALUE;

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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    let (entity, mut invincible, mut gizmo_component) = ship.into_inner();

    invincible.invincibility_timer.tick(time.delta());

    let blink_visibility =
        (time.elapsed_secs() * beef_blastoids_globals.SHIP_BLINK_RATE).sin() > 0.0;
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
    beef_blastoids_globals: Res<BeefBlastoidsGlobals>,
) {
    if **score >= beef_blastoids_globals.MAX_SCORE {
        next_state.set(BeefBlastoidsState::GameOver);
    }

    if **lives == 0 {
        next_state.set(BeefBlastoidsState::GameOver);
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct GameOverTimer(Timer);

fn game_over(
    mut commands: Commands,
    lives: Res<Lives>,
    root_node: Single<Entity, With<RootNode>>,
    mut outcomes: ResMut<GameOutcomes>,
) {
    let won = lives.0 > 0;

    if won {
        outcomes.beef_blastoids.wins += 1;
    } else {
        outcomes.beef_blastoids.losses += 1;
    }
    
    let message_text = Text::new(if won { "You win!" } else { "You lose!" });

    commands.spawn((
        StateScoped(BeefBlastoidsState::GameOver),
        ChildOf(*root_node),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![message_text],
    ));

    commands.insert_resource(GameOverTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn check_game_over_timer(
    mut timer: ResMut<GameOverTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_state.set(GameState::ChooseGame);
        commands.remove_resource::<GameOverTimer>();
    }
}
