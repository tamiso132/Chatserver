use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{DirEntryExt, FileExt},
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

pub enum StorageError {
    AlreadyExist(String),
    LoginFailed,
    HasNoRooms,
    HasNoMessages,
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    uuid: u64,
    firstname: String,
    lastname: String,
    username: String,
    password: String,
}

impl UserLogin {
    pub fn create_user(
        firstname: String,
        lastname: String,
        username: String,
        password: String,
    ) -> Result<u64, StorageError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open("database/users.json");

        let mut file = match file {
            Ok(file) => {
                // File opened successfully, you can work with the file here
                file
            }
            Err(e) => {
                // Handle the error
                panic!("Error: {:?}", e);
            }
        };

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let mut last_uuid = 0;
        if content.len() == 0 {
            let new_user = UserLogin {
                uuid: last_uuid,
                firstname,
                lastname,
                username,
                password,
            };
            let v = vec![new_user];

            let write_data = serde_json::to_string(&v).unwrap();

            file.write_all_at(&write_data.as_bytes(), 0);
            return Ok(last_uuid);
        }

        match serde_json::from_str::<Vec<UserLogin>>(&content) {
            Ok(mut x) => {
                for user in &x {
                    if user.username == username {
                        return Err(StorageError::AlreadyExist(username.clone()));
                    }
                }
                let last = x.len();
                let new_user = UserLogin {
                    uuid: last as u64,
                    firstname,
                    lastname,
                    username,
                    password,
                };
                x.push(new_user);
                file.write_all_at(&serde_json::to_string(&x).unwrap().as_bytes(), 0)
                    .unwrap();
                Ok(last as u64)
            }
            Err(_) => todo!(),
        }
    }

    pub fn retrieve_all_users() -> Vec<String> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open("database/users.json")
            .unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer);

        match serde_json::from_str::<Vec<UserLogin>>(&buffer) {
            Ok(users) => {
                let mut usernames = vec![];
                for user in users {
                    usernames.push(user.username);
                }
                usernames
            }
            Err(err) => {
                vec![]
            }
        }
    }

    pub fn login_user(username: &str, password: &str) -> Result<u64, StorageError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open("database/users.json")
            .unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content);

        match serde_json::from_str::<Vec<UserLogin>>(&content) {
            Ok(users) => {
                for user in users {
                    if user.username == username {
                        if user.password == password {
                            return Ok(user.uuid);
                        } else {
                            return Err(StorageError::LoginFailed);
                        }
                    }
                }
                Err(StorageError::LoginFailed)
            }
            Err(_) => Err(StorageError::LoginFailed),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserChatRoom {
    chat_room_name: String,
    chat_room_index: u64,
}

impl UserChatRoom {
    pub fn add_new_chat(chat_name: &str, my_uuid: u64, other_usernames: Vec<&str>) {
        // find all uuid

        let mut user_buffer = String::new();
        let mut username_file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open("database/users.json")
            .unwrap();

        username_file.read_to_string(&mut user_buffer);
        let uuids = match serde_json::from_str::<Vec<UserLogin>>(&user_buffer) {
            Ok(users) => {
                let mut uuids = vec![my_uuid];
                for user in users {
                    for other in other_usernames.clone() {
                        if user.username == other {
                            uuids.push(user.uuid);
                            break;
                        }
                    }
                }
                uuids
            }
            Err(_) => panic!("should not come here at all"),
        };

        let chat_room_index = create_chat_room();
        let new_chat = UserChatRoom {
            chat_room_name: chat_name.to_string(),
            chat_room_index: chat_room_index,
        };
        for uuid in uuids {
            let mut chat_info_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(false)
                .open(format!("database/userchatrooms-{}.json", uuid))
                .unwrap();

            let mut content = String::new();
            chat_info_file.read_to_string(&mut content);

            match serde_json::from_str::<Vec<UserChatRoom>>(&content) {
                Ok(mut v) => {
                    v.push(new_chat.clone());
                    let data = serde_json::to_string(&v).unwrap();
                    chat_info_file.write_all_at(data.as_bytes(), 0);
                }
                Err(e) => {
                    let v = vec![&new_chat];
                    let data = serde_json::to_string(&v).unwrap();
                    chat_info_file.write_all_at(data.as_bytes(), 0);
                }
            }
        }
    }
    pub fn retrieve_chat_rooms(uuid: u64) -> Result<Vec<(String, u64)>, StorageError> {
        let mut chat_info_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(format!("database/userchatrooms-{}.json", uuid))
            .unwrap();

        let mut content = String::new();
        chat_info_file.read_to_string(&mut content);

        match serde_json::from_str::<Vec<UserChatRoom>>(&content) {
            Ok(mut v) => {
                let data: Vec<(String, u64)> = v
                    .into_iter()
                    .map(|v| (v.chat_room_name, v.chat_room_index))
                    .collect();
                Ok(data)
            }

            Err(e) => Err(StorageError::HasNoRooms),
        }
    }
}

fn create_chat_room() -> u64 {
    let mut chat_room_index = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open("database/chat_room_counter.json")
        .unwrap();
    let mut content = String::new();

    chat_room_index.read_to_string(&mut content);

    let index = match serde_json::from_str::<Value>(&content) {
        Ok(index) => {
            let mut index = index["value"].as_u64().unwrap();
            index += 1;
            chat_room_index
                .write_all_at(json!({"value": index}).to_string().as_bytes(), 0)
                .unwrap();
            index
        }
        Err(_) => 0,
    };
    let mut chat_room_index = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(format!("database/messages-{}.json", index))
        .unwrap();

    index
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    username: String,
    message: String,
}

impl Message {
    pub fn add(index: u64, username: String, message: String) {
        let mut chat_room_index = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(format!("database/messages-{}.json", index))
            .unwrap();

        let mut content = String::new();

        chat_room_index.read_to_string(&mut content);

        match serde_json::from_str::<Vec<Message>>(&content) {
            Ok(mut msgs) => {
                let msg = Message { username, message };
                msgs.push(msg);
                chat_room_index.write_all_at(serde_json::to_string(&msgs).unwrap().as_bytes(), 0);
            }
            Err(_) => {
                let msg = vec![Message { username, message }];
                chat_room_index.write_at(serde_json::to_string(&msg).unwrap().as_bytes(), 0);
            }
        };
    }
    pub fn retrieve_latest(
        room_index: u64,
        latest_message_index: u64,
    ) -> Result<(Vec<Message>, usize), StorageError> {
        let mut chat_room_index = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(format!("database/messages-{}.json", room_index))
            .unwrap();

        let mut content = String::new();

        chat_room_index.read_to_string(&mut content);

        match serde_json::from_str::<Vec<Message>>(&content) {
            Ok(mut msgs) => {
                let b = msgs[latest_message_index as usize..msgs.len()].to_owned();
                if b.len() > 0 {
                    Ok((b.to_vec(), msgs.len() - 1))
                } else {
                    Err(StorageError::HasNoMessages)
                }
            }
            Err(_) => Err(StorageError::HasNoMessages),
        }
    }
}
