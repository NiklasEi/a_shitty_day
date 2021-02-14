mod conversation;

use crate::ui::conversation::ConversationPlugin;
use crate::{AppState, STAGE};
use bevy::prelude::*;

use crate::assets::font_monogram;
use bevy_kira_audio::Audio;
pub use conversation::{CanTalk, ConversationId, HideConversation};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_resource(AudioState { paused: false })
            .on_state_enter(STAGE, AppState::InGame, init_ui.system())
            .add_plugin(ConversationPlugin)
            .on_state_update(STAGE, AppState::InGame, audio_state_button.system())
            .on_state_exit(STAGE, AppState::InGame, clean_up_ui.system());
    }
}

struct AudioState {
    paused: bool,
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

struct AudioStateButton;

struct ConversationText;

pub struct ConversationUi;

fn init_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    let font = asset_server.load(font_monogram());
    let gray = color_materials.add(Color::GRAY.into());
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
            material: gray.clone(),
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
        })
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Auto,
                    left: Val::Auto,
                    right: Val::Percent(50.),
                    top: Val::Px(0.),
                },
                ..Default::default()
            },
            material: gray,
            ..Default::default()
        })
        .with_children(|parent| {
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
                    ..Default::default()
                })
                .with(AudioStateButton);
        });
}

fn audio_state_button(
    button_materials: Res<ButtonMaterials>,
    audio: Res<Audio>,
    mut audio_state: ResMut<AudioState>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Mutated<Interaction>, With<AudioStateButton>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if audio_state.paused {
                    audio.resume();
                } else {
                    audio.pause();
                }
                audio_state.paused = !audio_state.paused;
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

fn clean_up_ui(
    commands: &mut Commands,
    conversation_query: Query<Entity, With<ConversationUi>>,
    audio_state_query: Query<Entity, With<AudioStateButton>>,
) {
    for ui in conversation_query.iter() {
        commands.despawn(ui);
    }
    for retry in audio_state_query.iter() {
        commands.despawn_recursive(retry);
    }
}
