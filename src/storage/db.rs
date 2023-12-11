use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{DirEntryExt, FileExt},
};

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::ser::SerializeStruct;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

pub enum StorageError {
    AlreadyExist(String),
    LoginFailed,
    HasNoRooms,
    HasNoMessages,
    HasNoSavedDirectory,
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
        Err(_) => {
            let json = json!({"value": 1});
            chat_room_index.write_all_at(json.to_string().as_bytes(), 0);
            0
        }
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
                println!("{}: {}, {}", latest_message_index, msgs.len(), b.len());

                if b.len() > 0 {
                    Ok((b.to_vec(), msgs.len()))
                } else {
                    Err(StorageError::HasNoMessages)
                }
            }
            Err(_) => Err(StorageError::HasNoMessages),
        }
    }
}

#[derive(Deserialize, Clone)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Date {
    fn diff(&self, date: &Date) -> bool {
        let year_diff = self.year != date.year;
        let month_diff = self.month != date.month;
        let day_diff = self.day != date.day;
        let hour_diff = self.hour != date.hour;
        let minute_diff = self.minute != date.minute;
        let second_diff = self.second != date.second;

        if year_diff || month_diff || day_diff || hour_diff || minute_diff || second_diff {
            return true;
        }
        false
    }
}

#[derive(Deserialize, Clone)]
pub struct FileData {
    name: String,
    size: u64,
    last_modified: Date,
}

#[derive(Deserialize, Clone)]
pub struct Directory {
    name: String,
    path: String,
    files: Vec<FileData>,
    directories: Option<Vec<Directory>>,
}

impl serde::Serialize for Directory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Directory", 4).unwrap();
        state.serialize_field("name", &self.name);
        state.serialize_field("path", &self.path);
        state.serialize_field("files", &self.files);
        state.serialize_field("directories", &self.directories);
        state.end()
    }
}

impl serde::Serialize for FileData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("File", 3).unwrap();
        state.serialize_field("name", &self.name);
        state.serialize_field("size", &self.size);
        state.serialize_field("last_modified", &self.last_modified);
        state.end()
    }
}

impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Date", 6).unwrap();
        state.serialize_field("year", &self.year);
        state.serialize_field("month", &self.month);
        state.serialize_field("day", &self.day);
        state.serialize_field("hour", &self.hour);
        state.serialize_field("minute", &self.minute);
        state.serialize_field("second", &self.second);
        state.end()
    }
}

impl Directory {
    pub fn update_directory(
        uuid: u64,
        dir_sent: &str,
    ) -> Vec<(String, Vec<FileData>, Vec<FileData>, Vec<FileData>)> {
        let mut file_data = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(format!("database/filesync-{}.json", uuid))
            .unwrap();

        let mut buffer = String::new();

        file_data.read_to_string(&mut buffer);
        let dir_sent = serde_json::from_str::<Directory>(dir_sent).unwrap();

        match serde_json::from_str::<Directory>(&buffer) {
            Ok(dir) => {
                let diff = Directory::check_diff_dir(dir.clone(), dir_sent.clone());
                file_data.write_all_at(
                    serde_json::to_string_pretty(&dir_sent).unwrap().as_bytes(),
                    0,
                );
                diff
            }
            Err(e) => {
                let f = dir_sent.clone().get_files_in_dir();
                let file_remove: Vec<FileData> = vec![];
                let file_changed: Vec<FileData> = vec![];
                let mut ret = vec![];
                for index in 0..f.0.len() {
                    ret.push((
                        f.0[index].clone(),
                        f.1[index].clone(),
                        file_remove.clone(),
                        file_changed.clone(),
                    ));
                }
                file_data.write_all_at(
                    serde_json::to_string_pretty(&dir_sent).unwrap().as_bytes(),
                    0,
                );
                ret
            }
        }
    }

    fn check_diff_dir(
        saved_directory: Directory,
        sent_directory: Directory,
    ) -> Vec<(String, Vec<FileData>, Vec<FileData>, Vec<FileData>)> {
        let mut sent_files = sent_directory.files;
        let mut saved_files = saved_directory.files;

        let mut changed_files = vec![];
        let mut new_files = vec![];
        let mut files_removed = vec![];

        let mut ret = vec![];
        let mut remove_index = 0;

        for (saved_index, saved_file) in saved_files.clone().iter().enumerate() {
            for (sent_index, sent_file) in sent_files.clone().into_iter().enumerate() {
                if saved_file.name == sent_file.name {
                    sent_files.remove(sent_index - remove_index);
                    saved_files.remove(saved_index - remove_index);
                    remove_index += 1;

                    if saved_file.last_modified.diff(&sent_file.last_modified) {
                        changed_files.push(sent_file);
                    }
                    break;
                }
            }
        }

        for sent in sent_files {
            // files that are
            new_files.push(sent);
        }

        for file in saved_files {
            files_removed.push(file);
        }

        if sent_directory.directories.is_some() {
            if saved_directory.directories.is_some() {
                let mut send_dirs = sent_directory.directories.unwrap();
                let mut saved_dirs = saved_directory.directories.unwrap();

                for saved_dir in &saved_dirs {
                    for (sent_dir_index, sent_dir) in send_dirs.clone().into_iter().enumerate() {
                        if saved_dir.name == sent_dir.name {
                            let diff = Self::check_diff_dir(saved_dir.to_owned(), sent_dir);
                            for index in 0..diff.len() {
                                ret.push((
                                    diff[index].0.clone(),
                                    diff[index].1.clone(),
                                    diff[index].2.clone(),
                                    diff[index].3.clone(),
                                ))
                            }
                        }
                    }
                }
            }
        } else {
            let saved_dirs = saved_directory.directories.unwrap();
            for dir in saved_dirs {
                let dir_files = dir.get_files_in_dir();
                let changed_files: Vec<FileData> = vec![];
                let d = (dir_files.0, dir_files.1, changed_files);
                for index in 0..d.0.len() {
                    ret.push((d.0[index].clone(), d.1[index].clone(), vec![], vec![]));
                }
            }
        }

        ret.push((
            saved_directory.name,
            new_files,
            files_removed,
            changed_files,
        ));

        return ret;
    }
    fn get_files_in_dir(self) -> (Vec<String>, Vec<Vec<FileData>>) {
        let mut name = vec![];
        let mut files = vec![];

        name.push(self.name);
        files.push(self.files);

        if self.directories.is_some() {
            let dirs = self.directories.unwrap();
            for dir in dirs {
                let d = dir.get_files_in_dir();
                let dir_files = d.1;
                let dir_name = d.0;
                name.extend(dir_name);
                files.extend(dir_files);
            }
        }

        (name, files)
    }
}
