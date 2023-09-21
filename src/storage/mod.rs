// this is being used by my rasperry pi

mod db;

use bincode::{deserialize, serialize, ErrorKind};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::mem::size_of_val;
//use std::os::unix::prelude::FileExt;
use std::os::windows::fs::FileExt;

use crate::relational::FreeList;

pub mod file {
    use std::{
        fs::OpenOptions,
        io::{self, Read, Seek, Write},
    };

    pub enum FileIntention {
        Push(FileElement),
        Update(FileElement),
        Insert(FileElement),
        Truncate(u64),
    }

    pub struct FileElement {
        index: usize,
        offset: usize,
        el: Vec<u8>,
    }
    impl FileElement {
        pub fn new(index: usize, offset: usize, capacity: usize) -> Self {
            Self {
                index,
                offset,
                el: Vec::with_capacity(capacity),
            }
        }
        pub fn new_with_el<T: serde::Serialize>(index: usize, offset: usize, el: &T) -> Self {
            Self {
                index,
                offset,
                el: bincode::serialize(el).unwrap(),
            }
        }
        pub fn append_element<T: serde::Serialize>(&mut self, e: &T) {
            let mut y = bincode::serialize(e).unwrap();
            self.el.append(&mut y);
        }
    }
    pub fn update_file(file_path: &String, data: Vec<FileIntention>) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        let mut size = file.metadata().map(|m| m.len()).unwrap_or(0);

        for intention in data {
            match intention {
                FileIntention::Push(info) => {
                    file.seek(std::io::SeekFrom::End(0)).unwrap();
                    size += info.el.len() as u64;
                    file.write_all(&info.el)?;
                }
                FileIntention::Update(info) => {
                    let file_offset = info.offset + info.el.len() * info.index;
                   // file.write_all_at(&info.el, file_offset as u64)?;
                }

                FileIntention::Insert(info) => {
                    let file_offset = info.offset + info.el.len() * info.index;
                    file.seek(std::io::SeekFrom::Start(file_offset as u64))?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).unwrap();

                    // Seek back to the insertion point
                    file.seek(std::io::SeekFrom::Start(file_offset as u64))?;

                    // Write the serialized data
                    file.write_all(&info.el).unwrap();

                    // Write the previously read data after the insertion
                    file.write_all(&buffer).unwrap();
                }
                FileIntention::Truncate(el_size) => file.set_len(size - el_size)?,
            }
        }
        Ok(())
    }
}

pub fn push_to_file<T>(file_path: &str, data: &T) -> io::Result<()>
where
    T: serde::Serialize,
{
    // Serialize the data to binary format
    let serialized_data = serialize(data).unwrap();

    // Open the file in write mode, creating it if it doesn't exist
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) // Append mode
        .open(file_path)?;

    // Write the serialized data to the file
    file.write_all(&serialized_data);

    Ok(())
}

pub fn update_element<T>(file_name: &str, index: usize, data: &T, offset: usize)
where
    T: serde::Serialize,
{
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();
    let seri = bincode::serialize(data).unwrap();
    let off = seri.len() * index + offset;

    #[cfg(target_os = "windows")]{
        file.seek(SeekFrom::Start(off as u64));
        file.write_all(&seri);
    }

    #[cfg(target_os = "linux")]{
        file.write_at(&seri, off as u64);
    }
}

pub fn insert_at_index<T>(file_name: &String, index: usize, data: &T, offset: usize)
where
    T: serde::Serialize,
{
    let serialized_data = bincode::serialize(data).unwrap();
    let data_length = serialized_data.len();

    // Open the file in read-write mode
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();

    // Seek to the desired insertion point
    let insertion_point = (data_length * index + offset) as u64;
    file.seek(SeekFrom::Start(insertion_point)).unwrap();

    // Read the data after the insertion point
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Seek back to the insertion point
    file.seek(SeekFrom::Start(insertion_point)).unwrap();

    // Write the serialized data
    file.write_all(&serialized_data).unwrap();

    // Write the previously read data after the insertion
    file.write_all(&buffer).unwrap();
}

pub fn read_vec<T: serde::Serialize + serde::de::DeserializeOwned>(
    file_path: &str,
) -> Result<Vec<T>, Box<ErrorKind>> {
    let mut file = File::open(file_path)?;

    // Read the entire content of the file into a buffer
    let mut buffer = vec![];
    let _ = file.read_to_end(&mut buffer);

    return bincode::deserialize::<Vec<T>>(&buffer);
}
pub fn read_all_file<T>(file_path: &str, offset_bytes: usize) -> Result<Vec<T>, Box<ErrorKind>>
where
    T: serde::de::DeserializeOwned + Default + serde::Serialize,
{
    // Open the file in read mode
    let mut file = File::open(file_path)?;

    let element_size = bincode::serialize(&T::default()).unwrap().len();
    // Read the entire content of the file into a buffer
    let mut buffer = vec![];
    file.seek(io::SeekFrom::Start(offset_bytes as u64))?;
    let _ = file.read_to_end(&mut buffer);

    let mut list = Vec::with_capacity(buffer.len() / element_size);

    for chunk in buffer.chunks(element_size) {
        list.push(match deserialize::<T>(chunk) {
            Ok(ele) => ele,
            Err(_) => T::default(),
        });
    }
    Ok(list)

    // Deserialize the content
}

pub fn read_all_file_option<T>(
    file_path: &str,
    free_list: &FreeList,
    offset: usize,
) -> Result<Vec<Option<T>>, Box<ErrorKind>>
where
    T: serde::de::DeserializeOwned + Default + serde::Serialize,
{
    let mut file = File::open(file_path)?;
    let length = bincode::serialize(&Some(T::default())).unwrap().len();

    let mut buffer = vec![];
    file.seek(io::SeekFrom::Start(offset as u64))?;
    let _ = file.read_to_end(&mut buffer);

    let mut list = Vec::with_capacity(free_list.get_size() as usize);

    let vec_free = free_list.get_vec();
    let mut free_index = 0;
    let curr_index = 0;

    let mut free = &vec_free[free_index];
    for chunk in buffer.chunks(length) {
        if free.start <= curr_index && free.end >= curr_index {
            list.push(Some(bincode::deserialize(chunk).unwrap()))
        } else if free.start > curr_index {
            list.push(None);
        } else {
            free_index += 1;
            free = &vec_free[free_index];
        }
    }
    Ok(list)
}

pub fn read_chunk<T>(
    file_path: &str,
    size: Vec<u8>,
    start: usize,
    amount_element: usize,
    offset: usize,
) -> Result<Vec<T>, Box<ErrorKind>>
where
    T: serde::de::DeserializeOwned,
{
    let mut file = File::open(file_path)?;
    file.seek(io::SeekFrom::Start((start * size.len() + offset) as u64))?;

    // Read the entire content of the file into a buffer
    let mut buffer = vec![0u8; (amount_element * size.len()) as usize];
    let _ = file.read_exact(&mut buffer);

    let mut list = Vec::with_capacity(buffer.len() / size.len());

    for chunk in buffer.chunks(size.len()) {
        list.push(deserialize::<T>(chunk).unwrap());
    }
    Ok(list)
}
