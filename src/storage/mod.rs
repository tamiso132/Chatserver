use serde::Serialize;
use serde_json::json;

use crate::storage::db::FileData;

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



pub fn directory_sync_info(uuid: u64, directory: &str) {}

pub fn update_directory_sync(uuid: u64, directory: &str) -> String {
    let dirs = db::Directory::update_directory(uuid, directory);
    let mut str = String::new();
    // for dd in dirs {
    //     s.push_str(dd.0.as_str());
    // }
    for dir in dirs {
        let dir_path = dir.0;
        let added_files = get_file_names(dir.1);
        let missing_files = get_file_names(dir.2);
        let updated_files = get_file_names(dir.3);

        let val = json!({"path": dir_path, "Added Files": added_files, "Removed Files": missing_files, "Updated Files": updated_files});
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        val.serialize(&mut ser).unwrap();

        str.push_str(&String::from_utf8(buf).unwrap());
    }

    str
}
