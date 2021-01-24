mod conversation;

use crate::ui::conversation::ConversationPlugin;
use crate::{AppState, GameState, STAGE};
use bevy::prelude::*;

use crate::assets::font_monogram;
pub use conversation::{CanTalk, ConversationId, HideConversation};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .on_state_enter(STAGE, AppState::InGame, init_ui.system())
            .add_plugin(ConversationPlugin)
            .on_state_update(STAGE, AppState::InGame, retry_system.system())
            .on_state_update(STAGE, AppState::InGame, click_retry_button.system())
            .on_state_exit(STAGE, AppState::InGame, remove_conversation_ui.system());
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
}

pub struct ContinueConversationButton;
pub struct ContinueConversationText;

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

struct RetryButton;

struct ConversationText;

pub struct ConversationUi;

fn init_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    let font = asset_server.load(font_monogram());
    let material = color_materials.add(Color::GRAY.into());
    commands
        .spawn(CameraUiBundle::default())
        // root node
        .spawn(NodeBundle {
            style: Style {
                margin: Rect {
                    bottom: Val::Px(0.),
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Auto,
                },
                padding: Rect::all(Val::Px(10.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                is_transparent: false,
            },
            material,
            ..Default::default()
        })
        .with(ConversationUi)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "".to_owned(),
                        font,
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(0.6, 0.6, 0.6),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ConversationText)
                .with(ConversationUi);
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .with(ContinueConversationButton)
                .with(ConversationUi)
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            text: Text {
                                value: "".to_string(),
                                font: asset_server.load(font_monogram()),
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..Default::default()
                                },
                            },
                            visible: Visible {
                                is_visible: false,
                                is_transparent: false,
                            },
                            ..Default::default()
                        })
                        .with(ContinueConversationText)
                        .with(ConversationUi);
                });
        });
}

fn retry_system(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    game_state: ChangedRes<GameState>,
    button_materials: Res<ButtonMaterials>,
) {
    if game_state.health < 1 {
        commands
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with(RetryButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        value: "Restart".to_string(),
                        font: asset_server.load(font_monogram()),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            });
    }
}

fn click_retry_button(
    commands: &mut Commands,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *game_state = GameState::default();
                commands.despawn(button);
                commands.despawn(text);
                state.set_next(AppState::RetryGame).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn remove_conversation_ui(
    commands: &mut Commands,
    conversation_query: Query<Entity, With<ConversationUi>>,
) {
    for ui in conversation_query.iter() {
        commands.despawn(ui);
    }
}
