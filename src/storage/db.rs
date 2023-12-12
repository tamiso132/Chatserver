#[cfg(target_os = "linux")]
use std::os::unix::fs::{DirEntryExt, FileExt};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

#[cfg(target_os = "windows")]
use std::os::windows::fs::FileExt;

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{ser::SerializeStruct, Serialize};
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

#[cfg(target_os = "windows")]
fn write_all_at(file: &mut File, s: String) {
    file.set_len(0);
    file.seek_write(s.as_bytes(), 0);
}

#[cfg(target_os = "linux")]
fn write_all_at(file: &mut File, s: String) {
    file.write_all_at(&s.as_bytes(), 0);
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

            write_all_at(&mut file, write_data);
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
                write_all_at(&mut file, serde_json::to_string(&x).unwrap());
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
                    write_all_at(&mut chat_info_file, data);
                }
                Err(e) => {
                    let v = vec![&new_chat];
                    let data = serde_json::to_string(&v).unwrap();
                    write_all_at(&mut chat_info_file, data);
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
            write_all_at(&mut chat_room_index, json!({"value": index}).to_string());
            index
        }
        Err(_) => {
            let json = json!({"value": 1});
            write_all_at(&mut chat_room_index, json!({"value": 1}).to_string());
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
                write_all_at(&mut chat_room_index, serde_json::to_string(&msgs).unwrap());
            }
            Err(_) => {
                let msg = vec![Message { username, message }];
                write_all_at(&mut chat_room_index, serde_json::to_string(&msg).unwrap());
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
                    Ok((b.to_vec(), msgs.len()))
                } else {
                    Err(StorageError::HasNoMessages)
                }
            }
            Err(_) => Err(StorageError::HasNoMessages),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Deserialize, Clone, Debug)]
pub struct FileData {
    pub name: String,
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
    ) -> Vec<(
        String,
        Option<Vec<FileData>>,
        Option<Vec<FileData>>,
        Option<Vec<FileData>>,
    )> {
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
                let diff = Directory::difference_directory(dir.clone(), dir_sent.clone());
                write_all_at(
                    &mut file_data,
                    serde_json::to_string_pretty(&dir_sent).unwrap(),
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
                write_all_at(
                    &mut file_data,
                    serde_json::to_string_pretty(&dir_sent).unwrap(),
                );
                todo!();
            }
        }
    }

    pub fn get_file_names(files: Option<Vec<FileData>>) -> Option<Vec<String>> {
        {
            if files.is_some() {
                let files = files.unwrap();
                let names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
                Some(names)
            } else {
                None
            }
        }
    }
    fn difference_files(
        saved_directory: Directory,
        sent_directory: Directory,
    ) -> (
        Option<String>,
        Option<Vec<FileData>>,
        Option<Vec<FileData>>,
        Option<Vec<FileData>>,
    ) {
        let mut sent_files = sent_directory.files;
        let mut saved_files = saved_directory.files;

        let saved_files_c = saved_files.clone();
        let sent_files_c = sent_files.clone();

        //let mut new_files;
        let mut changed_files = vec![];

        let mut saved_index_to_remove = vec![];
        let mut sent_index_to_remove = vec![];
        // let mut removed_files;
        for (sent_index, sent_file) in sent_files_c.iter().enumerate() {
            'Inner: for (saved_index, saved_file) in saved_files_c.iter().enumerate() {
                if sent_file.name == saved_file.name {
                    if saved_file.last_modified.diff(&sent_file.last_modified) {
                        changed_files.push(saved_file.clone());
                    }
                    sent_index_to_remove.push(sent_index);
                    saved_index_to_remove.push(saved_index);
                    break 'Inner;
                }
            }
        }
        let mut i = 0;
        saved_files.retain(|_| {
            let keep = !saved_index_to_remove.contains(&i);
            i += 1;
            keep
        }); // missing files
        i = 0;
        sent_files.retain(|_| {
            let keep = !sent_index_to_remove.contains(&i);
            i += 1;
            keep
        }); // new files

        let missing_files = if saved_files.len() == 0 {
            None
        } else {
            Some(saved_files)
        };

        let new_files = if sent_files.len() == 0 {
            None
        } else {
            Some(sent_files)
        };

        let changed_files = if changed_files.len() == 0 {
            None
        } else {
            Some(changed_files)
        };

        let path = if changed_files.is_none() && new_files.is_none() && missing_files.is_none() {
            None
        } else {
            Some(saved_directory.path)
        };

        (path, new_files, missing_files, changed_files)
    }
    fn difference_directory(
        saved_directory: Directory,
        sent_directory: Directory,
    ) -> Vec<(
        String,
        Option<Vec<FileData>>,
        Option<Vec<FileData>>,
        Option<Vec<FileData>>,
    )> {
        let mut ret = vec![];
        let files = Self::difference_files(saved_directory.clone(), sent_directory.clone());
        if files.0.is_some() {
            ret.push((files.0.unwrap(), files.1, files.2, files.3));
        }

        if saved_directory.directories.is_none() && sent_directory.directories.is_none() {
            return ret;
        }

        if saved_directory.directories.is_none() {
            // TODO, add all sent as new files
            for e in sent_directory.directories.unwrap() {
                let x = e.get_files_in_dir();
                for index in 0..x.0.len() {
                    ret.push((x.0[index].clone(), x.1[index].clone(), None, None));
                }
            }
        } else if sent_directory.directories.is_none() {
            // TODO, add all saved files as missing files
            for e in saved_directory.directories.unwrap() {
                let x = e.get_files_in_dir();
                for index in 0..x.0.len() {
                    ret.push((x.0[index].clone(), None, x.1[index].clone(), None));
                }
            }
        } else {
            // TODO, compare directiories
            let mut saved_dirs = saved_directory.directories.unwrap();
            let mut sent_dirs = sent_directory.directories.unwrap();

            let mut saved_to_remove_indices = vec![];
            let mut sent_to_remove_indices = vec![];

            for (saved_index, saved_dir) in saved_dirs.clone().iter().enumerate() {
                for (sent_index, sent_dir) in sent_dirs.iter().enumerate() {
                    if saved_dir.name == sent_dir.name {
                        saved_to_remove_indices.push(saved_index);
                        sent_to_remove_indices.push(sent_index);
                        let d = Self::difference_directory(saved_dir.clone(), sent_dir.clone());
                        ret.extend(d);
                    }
                }
            }
            let mut i = 0;
            sent_dirs.retain(|_| {
                let keep = !sent_to_remove_indices.contains(&i);
                i += 1;
                keep
            });

            let mut i = 0;
            saved_dirs.retain(|_| {
                let keep = !saved_to_remove_indices.contains(&i);
                i += 1;
                keep
            });

            for e in sent_dirs {
                // add new directories
                let x = e.get_files_in_dir();
                for index in 0..x.0.len() {
                    ret.push((x.0[index].clone(), x.1[index].clone(), None, None));
                }
            }

            for e in saved_dirs {
                /// missing directories
                let x = e.get_files_in_dir();
                for index in 0..x.0.len() {
                    ret.push((x.0[index].clone(), None, x.1[index].clone(), None));
                }
            }
        }
        ret

        //    for f in 0..x.0.len(){
        //     let mut buf = Vec::new();
        //     let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        //     let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);

        //     let obj = &json!({"Directory name": x.0[f], "Files": file_names[f]});
        //     obj.serialize(&mut ser).unwrap();

        //         txt.push_str(&String::from_utf8(buf).unwrap());
        //    }
    }
    fn get_files_in_dir(self) -> (Vec<String>, Vec<Option<Vec<FileData>>>) {
        let mut name = vec![];
        let mut files = vec![];

        name.push(self.name);
        if self.files.len() > 0 {
            files.push(Some(self.files));
        } else {
            files.push(None);
        }

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
fn get_names_of_files(files: &Vec<Vec<FileData>>) -> Vec<Vec<String>> {
    files
        .iter()
        .map(|f| f.iter().map(|f| f.name.clone()).collect())
        .collect()
}
