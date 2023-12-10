use self::db::{StorageError, UserLogin};

pub(crate) mod db;

pub fn register_new_user(
    firstname: &str,
    lastname: &str,
    username: &str,
    password: &str,
) -> Result<(u64), db::StorageError> {
    db::UserLogin::create_user(
        firstname.to_owned(),
        lastname.to_owned(),
        username.to_owned(),
        password.to_owned(),
    )
}

pub fn login_user(username: &str, password: &str) -> Result<u64, StorageError> {
    db::UserLogin::login_user(username, password)
}

pub fn retrive_all_usernames() -> Vec<String> {
    UserLogin::retrieve_all_users()
}

pub fn send_message(message: &str, username: String, room_index: u64) {
    db::Message::add(room_index, username, message.to_owned());
}

pub fn create_room(chat_name: &str, my_uuid: u64, other_usernames: Vec<&str>) {
    db::UserChatRoom::add_new_chat(chat_name, my_uuid, other_usernames)
}
pub fn retrieve_all_rooms(uuid: u64) -> Result<Vec<(String, u64)>, StorageError> {
    db::UserChatRoom::retrieve_chat_rooms(uuid)
}

pub fn retrieve_latest_messages(
    room_index: u64,
    latest_message_index: u64,
) -> Result<(Vec<db::Message>, usize), StorageError> {
    db::Message::retrieve_latest(room_index, latest_message_index)
}
