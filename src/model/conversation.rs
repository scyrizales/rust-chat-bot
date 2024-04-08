use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageType {
    User(String),
    Assitant(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: MessageType,
    pub text: String,
}