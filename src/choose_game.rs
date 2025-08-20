use crate::{Game, GameState, loading::TextureAssets};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin).add_systems(
        OnEnter(GameState::ChooseGame),
        (setup_choose_game_panel, setup_menu),
    );
}

fn setup_choose_game_panel(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        Name::new("Choose Game Background"),
        Sprite {
            image: texture_assets.panel2.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., -1.0),
        StateScoped(GameState::ChooseGame),
    ));
}

fn setup_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Name::new("Pung"),
            StateScoped(GameState::ChooseGame),
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(200.0, 50.0)))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Srgba::new(1.0, 1.0, 1.0, 0.0))),
            ),
            Transform {
                translation: Vec3::new(-143.3, 108.5, 0.0),
                rotation: Quat::from_rotation_z(0.3),
                scale: Vec3::new(1.2, 1.1, 1.0),
            },
        ))
        .observe(
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing(Game::Pung));
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             mesh_materials: Query<&MeshMaterial2d<ColorMaterial>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                if let Ok(mesh_material) = mesh_materials.get(trigger.target())
                    && let Some(material) = materials.get_mut(mesh_material.id())
                {
                    material.color.set_alpha(0.3);
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>,
             mesh_materials: Query<&MeshMaterial2d<ColorMaterial>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                if let Ok(mesh_material) = mesh_materials.get(trigger.target())
                    && let Some(material) = materials.get_mut(mesh_material.id())
                {
                    material.color.set_alpha(0.0);
                }
            },
        );

    commands
        .spawn((
            Name::new("Beef Blastoids"),
            StateScoped(GameState::ChooseGame),
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(200.0, 50.0)))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Srgba::new(1.0, 1.0, 1.0, 0.0))),
            ),
            Transform {
                translation: Vec3::new(-57.6, 23.5, 0.0),
                rotation: Quat::from_rotation_z(0.25),
                scale: Vec3::new(1.3, 1.1, 1.0),
            },
        ))
        .observe(
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing(Game::BeefBlastoids));
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             mesh_materials: Query<&MeshMaterial2d<ColorMaterial>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                if let Ok(mesh_material) = mesh_materials.get(trigger.target())
                    && let Some(material) = materials.get_mut(mesh_material.id())
                {
                    material.color.set_alpha(0.3);
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>,
             mesh_materials: Query<&MeshMaterial2d<ColorMaterial>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                if let Ok(mesh_material) = mesh_materials.get(trigger.target())
                    && let Some(material) = materials.get_mut(mesh_material.id())
                {
                    material.color.set_alpha(0.0);
                }
            },
        );

    commands
        .spawn((
            Name::new("Race Place"),
            StateScoped(GameState::ChooseGame),
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(200.0, 50.0)))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Srgba::new(1.0, 1.0, 1.0, 0.0))),
            ),
            Transform {
                translation: Vec3::new(23.2, -216.6, 0.0),
                rotation: Quat::from_rotation_z(0.05),
                scale: Vec3::new(1.8, 1.5, 1.0),
            },
        ))
        .observe(
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing(Game::RacePlace));
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             mesh_materials: Query<&MeshMaterial2d<ColorMaterial>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                if let Ok(mesh_material) = mesh_materials.get(trigger.target())
                    && let Some(material) = materials.get_mut(mesh_material.id())
                {
                    material.color.set_alpha(0.3);
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>,
             mesh_materials: Query<&MeshMaterial2d<ColorMaterial>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                if let Ok(mesh_material) = mesh_materials.get(trigger.target())
                    && let Some(material) = materials.get_mut(mesh_material.id())
                {
                    material.color.set_alpha(0.0);
                }
            },
        );
}
