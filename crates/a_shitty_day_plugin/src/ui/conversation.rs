mod mall;

use crate::map::{Coordinate, Maps};
use crate::player::Player;
use crate::ui::conversation::mall::get_mall_conversations;
use crate::ui::{ContinueConversationText, ConversationText, ConversationUi};
use crate::{AppState, GameState, STAGE};
use bevy::ecs::bevy_utils::HashMap;
use bevy::prelude::*;

pub struct ConversationPlugin;

impl Plugin for ConversationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ConversationId>()
            .add_event::<HideConversation>()
            .on_state_enter(STAGE, AppState::InGame, load_conversations.system())
            .on_state_update(STAGE, AppState::InGame, trigger_conversation.system())
            .on_state_update(STAGE, AppState::InGame, hide_conversation.system())
            .on_state_update(STAGE, AppState::InGame, show_conversation.system());
    }
}

pub type ConversationId = usize;
pub struct HideConversation;

pub struct CanTalk {
    pub id: ConversationId,
}

pub struct Conversations {
    conversations: HashMap<ConversationId, Conversation>,
}

pub struct Conversation {
    initial_text: String,
    questions_and_answers: HashMap<String, Vec<String>>,
}

fn load_conversations(commands: &mut Commands, game_state: Res<GameState>) {
    let conversations = match game_state.current_map {
        Maps::Mall => get_mall_conversations(),
        Maps::SecondMall => todo!(),
    };
    commands.insert_resource(conversations);
}

fn trigger_conversation(
    mut game_state: ResMut<GameState>,
    mut conversation_id: ResMut<Events<ConversationId>>,
    can_talk_query: Query<(&Transform, &CanTalk)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if game_state.talking_to.is_some() {
        return;
    }
    for player_position in player_query.iter() {
        for (conversation_position, can_talk) in can_talk_query.iter() {
            if player_position
                .translation
                .distance(conversation_position.translation)
                < 32.
            {
                conversation_id.send(can_talk.id);
                game_state.talking_to = Some(Coordinate {
                    x: conversation_position.translation.x,
                    y: conversation_position.translation.y,
                });
                return;
            }
        }
    }
}

fn show_conversation(
    mut conversation_reader: Local<EventReader<ConversationId>>,
    conversation_id: Res<Events<ConversationId>>,
    mut conversation_text_query: Query<&mut Text, With<ConversationText>>,
    mut continue_text_query: Query<&mut Text, With<ContinueConversationText>>,
    mut ui_query: Query<&mut Visible, With<ConversationUi>>,
    conversations: Res<Conversations>,
) {
    if let Some(id) = conversation_reader.latest(&conversation_id) {
        println!("show");
        for mut ui in ui_query.iter_mut() {
            ui.is_visible = true;
        }
        for mut text in conversation_text_query.iter_mut() {
            text.value = conversations
                .conversations
                .get(id)
                .unwrap()
                .initial_text
                .clone();
        }
        for mut text in continue_text_query.iter_mut() {
            text.value = "Continue".to_owned();
        }
    }
}

fn hide_conversation(
    mut conversation_reader: Local<EventReader<HideConversation>>,
    conversation: Res<Events<HideConversation>>,
    mut ui_query: Query<&mut Visible, With<ConversationUi>>,
) {
    if conversation_reader.latest(&conversation).is_some() {
        println!("hiding");
        for mut ui in ui_query.iter_mut() {
            ui.is_visible = false;
        }
    }
}
