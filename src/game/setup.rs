use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers,
    sprite::MaterialMesh2dBundle, window::WindowResized,
};
use bevy_mod_picking::{
    backends::raycast::{RaycastBackendSettings, RaycastPickable},
    picking_core::Pickable,
    PickableBundle,
};
use bevy_pancam::*;
use std::{collections::HashMap, fs::File, io::BufReader};

use crate::{word_table::Tile, word_tree::load_from};

use super::{WordsDictionary, LAYER_WORLD};

#[derive(Component, Default)]
pub struct Table(pub crate::word_table::Table);

#[derive(Component, Reflect)]
pub struct TilesInventory {
    pub screen_rect: Rect,
}
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct CameraUI;

#[derive(Component)]
pub struct GameMarker;

#[derive(Component)]
pub struct FollowCamera;

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
        LAYER_WORLD,
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
    commands.spawn((
        SpatialBundle::default(),
        TilesInventory {
            screen_rect: Rect::new(
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::INFINITY,
            ),
        },
        Table(table),
        GameMarker,
    ));
}

pub(super) fn create_inventory(
    mut commands: Commands,
    q_camera: Query<Entity, With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let letter_size = 60f32;
    let size = Vec2::new(letter_size * 4f32 + letter_size, letter_size);
    let inventory_background = commands
        .spawn((
            // As noted above, we are adding children here but we don't need to add an event
            // listener. Events on children will bubble up to the parent!
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(size))).into(),
                transform: Transform::from_translation(Vec3::new(0f32, 0f32, -0.1f32)),
                material: materials.add(ColorMaterial::from(Color::hsl(50.0, 1.0, 0.5))),
                ..Default::default()
            },
            RaycastPickable,
            LAYER_WORLD,
        ))
        .id();
    let mut inventory = commands.spawn((
        GameMarker,
        TilesInventory {
            screen_rect: Rect::new(-size.x / 2f32, -size.y / 2f32, size.x / 2f32, size.y / 2f32),
            //screen_rect: Rect::new(-size.x / 2f32, 0f32, size.x / 2f32, size.y),
        },
        Table::default(),
        SpatialBundle::default(),
    ));
    inventory.insert(Transform::from_translation(Vec3::new(
        0f32,
        size.y / 2f32,
        2f32,
    )));
    let inventory_entity = inventory.add_child(inventory_background).id();

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0f32, 0f32, 2f32))),
            FollowCamera,
        ))
        .set_parent(q_camera.single())
        .add_child(inventory_entity);
}

pub fn on_window_resize(
    mut gizmos: Gizmos,
    mut to_scale: Query<&mut Transform, With<FollowCamera>>,
    camera: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
) {
    for (g_cam, cam) in camera.iter() {
        for mut t in to_scale.iter_mut() {
            t.translation = cam
                .ndc_to_world(g_cam, Vec3::new(0f32, -1f32, 0f32))
                .unwrap();
            gizmos.circle_2d(t.translation.truncate(), 120., Color::BLACK);
            t.translation =
                (g_cam.compute_matrix().inverse() * t.compute_matrix()).transform_point(Vec3::ZERO);
            t.translation.z = 2f32;
        }
    }
}

pub(super) fn scale_with_camera(
    mut to_scale: Query<&mut Transform, With<FollowCamera>>,
    camera: Query<&OrthographicProjection, With<MainCamera>>,
) {
    for mut t in to_scale.iter_mut() {
        t.scale = Vec3::ONE * camera.get_single().unwrap().scale;
    }
}
