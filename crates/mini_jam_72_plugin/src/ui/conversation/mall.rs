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
            initial_text: "Hi there. This is a very long sentence that is hopefully   \nspiked with some line breaks especially when getting   \ntoo long for the whole window.".to_owned(),
            questions_and_answers,
        },
    );


    let mut questions_and_answers = HashMap::default();
    questions_and_answers.insert(
        "Can you help me?".to_owned(),
        vec!["No".to_owned()],
    );
    conversations.insert(
        2,
        Conversation {
            initial_text: "Hi there. I am Klaus :)   ".to_owned(),
            questions_and_answers,
        },
    );


    let mut questions_and_answers = HashMap::default();
    questions_and_answers.insert(
        "Can you help me?".to_owned(),
        vec!["No".to_owned()],
    );
    conversations.insert(
        2,
        Conversation {
            initial_text: "Hi there. I am Klaus :)   ".to_owned(),
            questions_and_answers,
        },
    );

    Conversations { conversations }}