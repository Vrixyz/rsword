use bevy::{
    log::Level, prelude::*, render::view::RenderLayers, sprite::MaterialMesh2dBundle,
    text::DEFAULT_FONT_HANDLE, transform::systems::propagate_transforms,
};
//use bevy_eventlistener::prelude::*;
use bevy_mod_picking::{backends::raycast::RaycastPickTarget, prelude::*};
use bevy_pancam::*;
use std::{fs::File, io::BufReader};

use self::setup::{create_inventory, CameraUI, MainCamera, TilesInventory};

use super::word_tree::load_from;
use crate::word_tree::PossibleWords;

mod setup;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, DefaultPickingPlugins, PanCamPlugin));
        let f = File::open("assets/scrabble.en.txt").expect("Could not read file.");
        let reader = BufReader::new(f);
        let tree_root = load_from(reader);
        app.insert_resource(WordsDictionary(tree_root));

        app.add_systems(Startup, setup::setup);
        app.add_systems(PostStartup, (create_tiles, create_inventory));
    }
}

#[derive(Resource)]
pub struct WordsDictionary(PossibleWords);

fn round_to_nearest(value: f32, multiple: f32) -> f32 {
    dbg!((value / multiple).round() * multiple)
}

fn create_tiles(
    mut commands: Commands,
    q_table: Query<&setup::Table>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(table) = q_table.get_single() else {
        return;
    };
    let text_style = TextStyle {
        font: DEFAULT_FONT_HANDLE.typed_weak(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    for kv in &table.0.tiles {
        let tile_transform =
            Transform::from_translation((*kv.0 * IVec2::new(1, -1)).extend(0).as_vec3() * 60f32);
        commands
            .spawn((
                /**/
                Text2dBundle {
                    text: Text::from_section(kv.1.character.to_string(), text_style.clone())
                        .with_alignment(text_alignment),
                    transform: tile_transform,
                    ..default()
                },
                On::<Pointer<DragStart>>::run(
                    move |event: ListenerMut<Pointer<DragStart>>,
                          mut t: Query<&mut Transform, Without<MainCamera>>,
                          camera_world: Query<
                        (&Transform, &OrthographicProjection),
                        With<MainCamera>,
                    >,
                          mut pancams: Query<&mut PanCam>,
                          mut commands: Commands| {
                        for mut pancam in &mut pancams {
                            bevy::utils::tracing::event!(Level::INFO, "disabling pancams");
                            pancam.enabled = false;
                        }
                        commands.entity(event.target()).insert(Pickable::IGNORE);
                        commands
                            .entity(event.listener())
                            .insert(RenderLayers::layer(1));
                        commands
                            .entity(event.target())
                            .insert(RenderLayers::layer(1));
                        let mut transform = t.get_mut(event.listener()).unwrap();
                        let (camera_world_transform, proj) = camera_world.single();

                        dbg!(camera_world_transform.scale);

                        let to_world =
                            Mat4::from_scale(Vec3::ONE / proj.scale) * transform.compute_matrix();
                        let to_camera = camera_world_transform.compute_matrix();

                        let to_ui = to_world * to_camera.inverse();

                        let position_relative_to_camera = Transform::from_matrix(to_ui);

                        *transform = position_relative_to_camera;
                        //transform.scale = Vec3::ONE * proj.scale;
                        transform.translation.z = 10f32;
                    },
                ), // Disable picking + pancam
                On::<Pointer<Drag>>::listener_component_mut::<Transform>(|drag, transform| {
                    transform.translation.x += drag.delta.x; // Make the square follow the mouse
                    transform.translation.y -= drag.delta.y;
                }),
                On::<Pointer<DragEnd>>::run(
                    move |event: ListenerMut<Pointer<DragEnd>>,
                          mut pancams: Query<&mut PanCam>,
                          camera_world: Query<&Transform, With<MainCamera>>,
                          camera_ui: Query<&OrthographicProjection, With<MainCamera>>,
                          mut transforms: Query<&mut Transform, Without<MainCamera>>,
                          inventories: Query<&TilesInventory>,
                          mut commands: Commands| {
                        for mut pancam in &mut pancams {
                            pancam.enabled = true;
                        }
                        let Ok(mut transform) = transforms.get_mut(event.listener()) else {
                            return;
                        };
                        let camera_world_projection = camera_ui.single();
                        let camera_transform = camera_world.single();

                        let to_ui = Mat4::from_scale(Vec3::ONE * camera_world_projection.scale)
                            * transform.compute_matrix();
                        let to_camera = camera_transform.compute_matrix();

                        let to_ui = to_ui * to_camera;
                        *transform = Transform::from_matrix(to_ui);
                        transform.translation.x = round_to_nearest(transform.translation.x, 60f32); // Make the square follow the mouse
                        transform.translation.y = round_to_nearest(transform.translation.y, 60f32);
                        commands.entity(event.target()).insert(Pickable::default());
                        commands
                            .entity(event.listener())
                            .insert(RenderLayers::layer(0));
                        commands
                            .entity(event.target())
                            .insert(RenderLayers::layer(0));
                        transform.translation.z = 0f32;
                    },
                ),
            ))
            .with_children(|parent| {
                parent.spawn((
                    // As noted above, we are adding children here but we don't need to add an event
                    // listener. Events on children will bubble up to the parent!
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::from_translation(Vec3::NEG_Z)
                            .with_scale(Vec3::splat(60f32)),
                        material: materials.add(ColorMaterial::from(Color::hsl(120.0, 1.0, 0.5))),
                        ..Default::default()
                    },
                    PickableBundle::default(),
                    RaycastPickTarget::default(),
                ));
            });
    }
}
