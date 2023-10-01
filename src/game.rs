use bevy::{
    ecs::{
        component,
        system::{EntityCommand, EntityCommands},
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
    text::DEFAULT_FONT_HANDLE,
};
//use bevy_eventlistener::prelude::*;
use bevy_mod_picking::prelude::*;
use std::{collections::HashMap, fs::File, io::BufReader, time::Duration};

use super::word_tree::load_from;
use crate::{word_table::Tile, word_tree::PossibleWords};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, DefaultPickingPlugins));
        let f = File::open("assets/scrabble.en.txt").expect("Could not read file.");
        let reader = BufReader::new(f);
        let tree_root = load_from(reader);
        app.insert_resource(WordsDictionary(tree_root));

        app.add_systems(Startup, setup);
        app.add_systems(PostStartup, create_tiles);
    }
}

#[derive(Resource)]
pub struct WordsDictionary(PossibleWords);

#[derive(Component)]
pub struct Table(super::word_table::Table);

fn setup(mut commands: Commands) {
    // 2d camera
    commands.spawn((Camera2dBundle::default(), RaycastPickCamera::default()));

    let table = super::word_table::Table {
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

fn round_to_nearest(value: f32, multiple: f32) -> f32 {
    (value / multiple).round() * multiple
}

fn create_tiles(
    mut commands: Commands,
    q_table: Query<&Table>,
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
                On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
                On::<Pointer<DragEnd>>::run(
                    move |event: ListenerMut<Pointer<DragEnd>>,
                          mut transforms: Query<&mut Transform>,
                          mut commands: Commands| {
                        let Ok(mut transform) = transforms.get_mut(event.listener()) else {
                            return;
                        };
                        transform.translation.x = round_to_nearest(transform.translation.x, 60f32); // Make the square follow the mouse
                        transform.translation.y = round_to_nearest(transform.translation.y, 60f32);
                        commands.entity(event.target()).insert(Pickable::default());
                    },
                ),
                On::<Pointer<Drag>>::listener_component_mut::<Transform>(|drag, transform| {
                    transform.translation.x += drag.delta.x; // Make the square follow the mouse
                    transform.translation.y -= drag.delta.y;
                }),
                On::<Pointer<Drop>>::commands_mut(|event, commands| {
                    //commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
                    //commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
                }),
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
