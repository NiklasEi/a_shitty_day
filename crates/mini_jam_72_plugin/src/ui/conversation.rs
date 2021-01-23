mod mall;

use crate::map::Maps;
use crate::player::Player;
use crate::ui::conversation::mall::get_mall_conversations;
use crate::ui::ConversationText;
use crate::{AppState, GameState, STAGE};
use bevy::ecs::bevy_utils::HashMap;
use bevy::prelude::*;

pub struct ConversationPlugin;

impl Plugin for ConversationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ConversationId>()
            .on_state_enter(STAGE, AppState::InGame, load_conversations.system())
            .on_state_update(STAGE, AppState::InGame, trigger_conversation.system())
            .on_state_update(STAGE, AppState::InGame, show_conversation.system());
    }
}

pub type ConversationId = usize;

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
    mut conversation_id: ResMut<Events<ConversationId>>,
    can_talk_query: Query<(&Transform, &CanTalk)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player_position in player_query.iter() {
        for (conversation_position, can_talk) in can_talk_query.iter() {
            if player_position
                .translation
                .distance(conversation_position.translation)
                < 32.
            {
                conversation_id.send(can_talk.id);
                return;
            }
        }
    }
}

fn show_conversation(
    mut conversation_reader: Local<EventReader<ConversationId>>,
    conversation_id: Res<Events<ConversationId>>,
    mut text_query: Query<&mut Text, With<ConversationText>>,
    conversations: Res<Conversations>,
) {
    if let Some(id) = conversation_reader.latest(&conversation_id) {
        for mut text in text_query.iter_mut() {
            text.value = conversations
                .conversations
                .get(id)
                .unwrap()
                .initial_text
                .clone();
        }
    }
}
