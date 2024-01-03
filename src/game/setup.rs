use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers,
    sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::{
    backends::raycast::{RaycastBackendSettings, RaycastPickable},
    PickableBundle,
};
use bevy_pancam::*;
use std::{collections::HashMap, fs::File, io::BufReader};

use crate::{word_table::Tile, word_tree::load_from};

use super::WordsDictionary;

use super::{LAYER_DRAG, LAYER_INVENTORY};

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

#[derive(Component)]
pub struct GameMarker;

pub(super) fn unsetup(mut commands: Commands, q_to_despawn: Query<Entity, With<GameMarker>>) {
    commands.remove_resource::<WordsDictionary>();
    for e in q_to_despawn.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub(super) fn setup(mut commands: Commands) {
    let f = File::open("assets/scrabble.en.txt").expect("Could not read file.");
    let reader = BufReader::new(f);
    let tree_root = load_from(reader);
    commands.insert_resource(WordsDictionary(tree_root));
    // 2d world camera
    commands.spawn((
        Camera2dBundle::default(),
        PanCam::default(),
        RaycastPickable,
        MainCamera,
        GameMarker,
    ));

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
    commands.spawn((Table(table), GameMarker));
}

pub(super) fn create_inventory(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2d inventory camera
    commands.spawn((
        GameMarker,
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
        LAYER_INVENTORY.with(2),
        CameraUI,
    ));

    let inventory_background = commands
        .spawn((
            // As noted above, we are adding children here but we don't need to add an event
            // listener. Events on children will bubble up to the parent!
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::splat(200f32))))
                    .into(),
                transform: Transform::from_translation((Vec3::X + Vec3::Y) * 100f32),
                material: materials.add(ColorMaterial::from(Color::hsl(50.0, 1.0, 0.5))),
                ..Default::default()
            },
            LAYER_INVENTORY,
        ))
        .id();
    let mut inventory = commands.spawn((
        GameMarker,
        TilesInventory {
            screen_rect: Rect::new(200f32, 200f32, 400f32, 400f32),
        },
        SpatialBundle::default(),
    ));
    inventory.add_child(inventory_background);
}

pub(super) fn move_inventory(
    time: Res<Time>,
    mut q_inventory: Query<&mut Transform, With<TilesInventory>>,
) {
    for mut t in q_inventory.iter_mut() {
        //t.translation = (Vec3::X + Vec3::Y) * 100f32 * time.elapsed_seconds().sin();
    }
}
