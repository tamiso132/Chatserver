use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::FileExt,
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

pub enum StorageError {
    AlreadyExist(String),
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
        file.read_to_string(&mut content);

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
                last_uuid = x[last - 1].uuid;
                let new_user = UserLogin {
                    uuid: last_uuid,
                    firstname,
                    lastname,
                    username,
                    password,
                };
                x.push(new_user);
                file.write_all_at(&serde_json::to_string(&x).unwrap().as_bytes(), 0)
                    .unwrap();
            }
            Err(_) => todo!(),
        }
        Ok(last_uuid)
    }
}
#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct PersonalUserInfo {
    uuid: u32,
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
