use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers,
    sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::{
    backends::raycast::{RaycastBackendSettings, RaycastPickable},
    PickableBundle,
};
use bevy_pancam::*;
use std::collections::HashMap;

use crate::word_table::Tile;

#[derive(Component)]
pub struct Table(pub crate::word_table::Table);

#[derive(Component)]
pub struct TilesInventory {
    pub screen_rect: Rect,
}
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct CameraUI;

pub(super) fn setup(mut commands: Commands) {
    commands.insert_resource(RaycastBackendSettings {
        require_markers: true,
    });
    // 2d world camera
    commands.spawn((Camera2dBundle::default(), RaycastPickable, MainCamera));

    let table = crate::word_table::Table {
        tiles: HashMap::from([
            (
                (0, 0).into(),
                Tile {
                    team: 0,
                    character: 'h',
                },
            ),
            (
                (0, 1).into(),
                Tile {
                    team: 0,
                    character: 'e',
                },
            ),
            (
                (0, 2).into(),
                Tile {
                    team: 0,
                    character: 'y',
                },
            ),
            (
                (1, 2).into(),
                Tile {
                    team: 0,
                    character: 'o',
                },
            ),
            (
                (2, 2).into(),
                Tile {
                    team: 0,
                    character: 'u',
                },
            ),
            (
                (4, 2).into(),
                Tile {
                    team: 0,
                    character: 'a',
                },
            ),
        ]),
    };
    commands.spawn(Table(table));
}

pub(super) fn create_inventory(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2d inventory camera
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(1),
        CameraUI,
    ));

    let inventory_background = commands
        .spawn((
            // As noted above, we are adding children here but we don't need to add an event
            // listener. Events on children will bubble up to the parent!
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::from_translation((Vec3::X + Vec3::Y) * 100f32)
                    .with_scale(Vec3::splat(200f32)),
                material: materials.add(ColorMaterial::from(Color::hsl(50.0, 1.0, 0.5))),
                ..Default::default()
            },
            RenderLayers::layer(1),
        ))
        .id();
    let mut inventory = commands.spawn((
        TilesInventory {
            screen_rect: Rect::new(200f32, 200f32, 200f32, 200f32),
        },
        SpatialBundle::default(),
    ));
    inventory.add_child(inventory_background);
}
