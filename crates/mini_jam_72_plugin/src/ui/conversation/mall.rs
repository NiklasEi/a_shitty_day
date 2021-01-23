use crate::ui::conversation::{Conversation, Conversations};
use bevy::ecs::bevy_utils::HashMap;

pub fn get_mall_conversations() -> Conversations {
    let mut conversations = HashMap::default();

    let mut questions_and_answers = HashMap::default();
    questions_and_answers.insert(
        "Are you dumb?".to_owned(),
        vec!["Yes I am".to_owned(), "Are you?".to_owned()],
    );
    conversations.insert(
        1,
        Conversation {
            initial_text: "Hi there".to_owned(),
            questions_and_answers,
        },
    );

    Conversations { conversations }
}
