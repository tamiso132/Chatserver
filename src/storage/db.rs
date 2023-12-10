use std::{fs::{File, OpenOptions}, io::{Read, Write}};

use serde_derive::{Deserialize, Serialize};

pub enum StorageError{
    AlreadyExist(&'static str),
}


#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    uuid: u32,
    firstname: String,
    lastname: String,
    username: String,
    password: String,
}

impl UserLogin {
    pub fn create_user(firstname:String, lastname:String, username:String, password:String) -> Result<(), StorageError>{
        let mut file = OpenOptions::new().read(true).create(true).open("database/users.json").unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content);


        let mut last_uuid = 0;
        match serde_json::from_str::<Vec<UserLogin>>(&content) {
            Ok(mut x) => {
                for user in x{
                    if user.username == username{
                        return Err(StorageError::AlreadyExist(username.as_str()));
                    }
                }
                let last = x.len();
                last_uuid = x[last-1].uuid;
                let new_user = UserLogin{ uuid: last_uuid, firstname, lastname, username, password };
                x.push(new_user);
                file.write_all(&serde_json::to_string(&x).unwrap().as_bytes());
            },
            Err(e) => {
                if file.metadata().unwrap().len() > 0 {
                    let new_user = UserLogin{ uuid: 0, firstname, lastname, username, password };
                    let v = vec![new_user];
                    file.write_all(&serde_json::to_string(&v).unwrap().as_bytes());
                    return Ok(());
                }
                panic!("error {}", e);
            },
        }
        Ok(())
    }
}
#[derive(Serialize, Deserialize)]
pub struct ResponseUser{
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String
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
