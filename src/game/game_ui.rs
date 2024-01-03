use bevy::{prelude::*, render::view::RenderLayers, ui::FocusPolicy};
use bevy_mod_picking::picking_core::Pickable;

use super::GameState;

#[derive(Component)]
pub struct GameMenuMarker;

#[derive(Event)]
pub struct ExitGame;

pub fn game_unsetup_ui(mut commands: Commands, q_menus: Query<Entity, With<GameMenuMarker>>) {
    for e in q_menus.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn exit_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Disabled);
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
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
    mut exit_game: EventWriter<ExitGame>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Will Exit".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                exit_game.send(ExitGame);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Exit?".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Exit".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn game_setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                ..default()
            },
            ..default()
        },
        GameMenuMarker,
        RenderLayers::layer(4),
        Pickable::IGNORE,
    ));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },
            GameMenuMarker,
            RenderLayers::layer(5),
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Pickable::default(),
                    ButtonBundle {
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
                    },
                    RenderLayers::layer(4),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font: default(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        RenderLayers::layer(4),
                    ));
                });
        });
}
