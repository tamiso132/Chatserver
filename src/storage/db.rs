use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    uuid: u128,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct PersonalUserInfo {
    firstname: String,
    lastname: String,
    email: String,
}

struct ChatRooms {
    uuid: u128,
    chat_rooms: Vec<ChatRoom>,
}

#[derive(Serialize, Deserialize)]
struct ChatRoom {
    chat_name: String,
    chat_room_index: u64,
    username: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    user_index: u8,
    message: String,
}
