use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    uuid: u32,
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct ResponseUser{
    firstname: String,
    lastname: String,
    username: String,
    password: String
}

#[derive(Serialize, Deserialize)]
struct PersonalUserInfo {
    uuid:u32,
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
