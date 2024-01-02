use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    log::{self, Level},
    prelude::*,
    render::view::RenderLayers,
    sprite::MaterialMesh2dBundle,
    utils::tracing,
};
//use bevy_eventlistener::prelude::*;
use bevy_mod_picking::{
    backends::raycast::{RaycastBackendSettings, RaycastPickable},
    prelude::*,
};
use bevy_pancam::*;

use std::{fs::File, io::BufReader};

use self::setup::{create_inventory, GameMarker, MainCamera, TilesInventory};

use super::word_tree::load_from;
use crate::{
    game::{
        self,
        game_ui::{exit_game, ExitGame},
    },
    word_tree::PossibleWords,
};

mod game_ui;
mod setup;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    Disabled,
    Loading,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        });
        app.insert_resource(RaycastBackendSettings {
            require_markers: true,
        });

        app.add_systems(Update, button_system.run_if(in_state(GameState::Disabled)))
            .add_systems(Update, load_game.run_if(on_event::<StartGame>()));
        app.add_systems(Update, start_game.run_if(in_state(GameState::Loading)));
        app.add_plugins((DefaultPlugins, DefaultPickingPlugins, PanCamPlugin));
        app.add_state::<GameState>();
        app.add_event::<StartGame>();
        app.add_event::<game::game_ui::ExitGame>();
        app.add_systems(OnEnter(GameState::Disabled), setup_ui);
        app.add_systems(OnExit(GameState::Disabled), unsetup_ui);
        app.add_systems(OnEnter(GameState::Loading), setup::setup);
        app.add_systems(OnEnter(GameState::Loading), game::game_ui::game_setup_ui)
            .add_systems(Update, exit_game.run_if(on_event::<ExitGame>()));
        app.add_systems(OnEnter(GameState::Disabled), game::game_ui::game_unsetup_ui);
        app.add_systems(
            Update,
            game::game_ui::button_system.run_if(in_state(GameState::Loading)),
        );
        app.add_systems(
            Update,
            game::game_ui::button_system.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(Update, button_system.run_if(in_state(GameState::Disabled)));
        app.add_systems(OnExit(GameState::Playing), setup::unsetup);
        app.add_systems(
            OnEnter(GameState::Playing),
            (create_tiles, create_inventory),
        );
        app.add_event::<TileDropped>();
        app.add_systems(
            Update,
            react_tile_dropped
                .run_if(on_event::<TileDropped>())
                .run_if(in_state(GameState::Playing)),
        );
        app.configure_sets(
            PreUpdate,
            (
                bevy_eventlistener::EventListenerSet,
                bevy_pancam::PanCamSystemSet,
            )
                .chain(),
        );
        dbg!("test");
    }
}

#[derive(Event)]
struct StartGame;

#[derive(Component)]
struct MenuMarker;

pub fn load_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Loading);
}
pub fn start_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut start_game: EventWriter<StartGame>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Will Play".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                start_game.send(StartGame);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Play?".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Play".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn unsetup_ui(mut commands: Commands, q_menus: Query<Entity, With<MenuMarker>>) {
    for e in q_menus.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 100,
                ..default()
            },
            ..default()
        },
        MenuMarker,
    ));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MenuMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: default(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
#[derive(Resource)]
pub struct WordsDictionary(PossibleWords);

#[derive(Event)]
pub struct TileDropped {
    pub listener: Entity,
    pub target: Entity,
}

fn round_to_nearest(value: f32, multiple: f32) -> f32 {
    (value / multiple).round() * multiple
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
        font: Default::default(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    for kv in &table.0.tiles {
        let tile_transform =
            Transform::from_translation((*kv.0 * IVec2::new(1, -1)).extend(0).as_vec3() * 60f32);
        commands
            .spawn((
                GameMarker,
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
                        tracing::event!(Level::INFO, "disable pancams");
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
                    tracing::event!(Level::INFO, "drag to {:?}", transform.translation);
                }),
                On::<Pointer<DragEnd>>::run(
                    move |event: ListenerMut<Pointer<DragEnd>>,
                          mut pancams: Query<&mut PanCam>,
                          mut tile_dropped_event: EventWriter<TileDropped>| {
                        for mut pancam in &mut pancams {
                            pancam.enabled = true;
                        }
                        tile_dropped_event.send(TileDropped {
                            listener: event.listener(),
                            target: event.target(),
                        });
                        tracing::event!(Level::INFO, "(input) stop drag",);
                    },
                ),
            ))
            .with_children(|parent| {
                parent.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::from_translation(Vec3::NEG_Z)
                            .with_scale(Vec3::splat(60f32)),
                        material: materials.add(ColorMaterial::from(Color::hsl(120.0, 1.0, 0.5))),
                        ..Default::default()
                    },
                    PickableBundle::default(),
                    RaycastPickable,
                ));
            });
    }
}

fn react_tile_dropped(
    mut commands: Commands,
    camera_world: Query<&Transform, With<MainCamera>>,
    camera_ui: Query<&OrthographicProjection, With<MainCamera>>,
    mut transforms: Query<&mut Transform, Without<MainCamera>>,
    mut tile_dropped_event: EventReader<TileDropped>,
) {
    for tile_dropped in tile_dropped_event.read() {
        let Ok(mut transform) = transforms.get_mut(tile_dropped.listener) else {
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
        tracing::event!(
            Level::INFO,
            "(event) stop drag to {:?}",
            transform.translation
        );
        // HACK: to circumvent DragEnd being sometimes before Drag.
        commands.entity(tile_dropped.listener).insert(*transform);
        commands
            .entity(tile_dropped.target)
            .insert(Pickable::default());
        commands
            .entity(tile_dropped.listener)
            .insert(RenderLayers::layer(0));
        commands
            .entity(tile_dropped.target)
            .insert(RenderLayers::layer(0));
        transform.translation.z = 0f32;
    }
}
