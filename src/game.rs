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

const LAYER_WORLD: RenderLayers = RenderLayers::layer(0);
const LAYER_INVENTORY: RenderLayers = RenderLayers::layer(1);
const LAYER_DRAG: RenderLayers = RenderLayers::layer(2);

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
        app.add_systems(Update, setup::move_inventory);
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

#[derive(Component, Clone)]
pub struct TilePos(IVec2);

impl From<&IVec2> for TilePos {
    fn from(value: &IVec2) -> Self {
        Self(*value)
    }
}

impl TilePos {
    pub fn from_world_pos(pos: &Vec2) -> Self {
        Self(IVec2::new((pos.x / 60f32) as i32, (-pos.y / 60f32) as i32))
    }

    pub fn to_local_pos(&self) -> Vec2 {
        IVec2::new(self.0.x, -self.0.y).as_vec2() * 60f32
    }
}

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
            Transform::from_translation(TilePos::from(kv.0).to_local_pos().extend(0f32));
        commands
            .spawn((
                TilePos(*kv.0),
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
                          mut parent: Query<&Parent>,
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
                        if let Ok(parent) = parent.get(commands.entity(event.listener()).id()) {
                            let parent_transform = *t.get(parent.get()).unwrap();
                            let mut transform = t.get_mut(event.listener()).unwrap();
                            transform.translation =
                                parent_transform.transform_point(transform.translation);
                            commands.entity(event.listener()).remove_parent();
                        }
                        commands.entity(event.target()).insert(Pickable::IGNORE);
                        commands.entity(event.listener()).insert(LAYER_DRAG);
                        commands.entity(event.target()).insert(LAYER_DRAG);

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
                    tracing::event!(Level::DEBUG, "drag to {:?}", transform.translation);
                }),
                On::<Pointer<DragEnd>>::run(
                    move |event: ListenerMut<Pointer<DragEnd>>,
                          mut pancams: Query<&mut PanCam>,
                          mut tile_dropped_event: EventWriter<TileDropped>| {
                        for mut pancam in &mut pancams {
                            pancam.enabled = true;
                        }
                        // HACK: to circumvent DragEnd being sometimes before Drag.
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
                    LAYER_WORLD,
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
    mut q_table: Query<&mut setup::Table>,
    camera_world: Query<&Transform, With<MainCamera>>,
    camera_ui: Query<&OrthographicProjection, With<MainCamera>>,
    mut transforms: Query<
        (Entity, &mut Transform, &mut TilePos),
        (Without<MainCamera>, Without<TilesInventory>),
    >,
    mut q_inventory: Query<
        (Entity, &GlobalTransform, &mut Transform, &TilesInventory),
        Without<MainCamera>,
    >,
    mut tile_dropped_event: EventReader<TileDropped>,
) {
    for tile_dropped in tile_dropped_event.read() {
        let Ok((tile_entity, mut transform, mut tile_pos)) =
            transforms.get_mut(tile_dropped.listener)
        else {
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

        let mut layer = LAYER_WORLD;
        let mut table = q_table.single_mut();
        if let Ok((i_entity, i_global_transform, i_transform, i_inventory)) =
            q_inventory.get_single()
        {
            let global_pos_inventory = i_transform.transform_point(Vec3::ZERO);

            let range_x =
                global_pos_inventory.x..(global_pos_inventory.x + i_inventory.screen_rect.width());
            let range_y =
                global_pos_inventory.y..(global_pos_inventory.y + i_inventory.screen_rect.height());
            dbg!(&range_y);
            if range_x.contains(&transform.translation.x)
                && range_y.contains(&transform.translation.y)
            {
                dbg!("inside");
                commands.entity(tile_entity).set_parent(i_entity);
                transform.translation -= i_transform.translation;
                //transform.translation = i_global_transform.transform_point(transform.translation);
                layer = LAYER_INVENTORY;
                table.0.tiles.remove(&tile_pos.0);
            }
        }
        let possible_new_tile_pos = TilePos::from_world_pos(&transform.translation.xy());
        if table.0.tiles.contains_key(&possible_new_tile_pos.0) {
            let original_position = &TilePos::to_local_pos(&tile_pos);
            transform.translation = original_position.extend(0f32);
        } else {
            commands
                .entity(tile_dropped.listener)
                .insert(possible_new_tile_pos.clone());
            let copy = table.0.tiles[&tile_pos.0].clone();
            table.0.tiles.remove(&tile_pos.0);
            table.0.tiles.insert(possible_new_tile_pos.0, copy);
        }

        tracing::event!(
            Level::INFO,
            "(event) stop drag to {:?}",
            transform.translation
        );
        commands.entity(tile_dropped.listener).insert(*transform);
        commands
            .entity(tile_dropped.target)
            .insert(Pickable::default());
        commands.entity(tile_dropped.listener).insert(layer);
        commands.entity(tile_dropped.target).insert(layer);
        transform.translation.z = 0f32;
    }
}
