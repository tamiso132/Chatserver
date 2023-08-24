use bincode::{deserialize, serialize, ErrorKind};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::mem::size_of_val;
use std::os::unix::prelude::FileExt;
use std::os::unix::raw::off_t;

use crate::relational::FreeList;

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

pub fn remove_element(file_path: &str, ones: &Vec<u8>, index: usize, offset: usize) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) // Append mode
        .open(file_path)
        .unwrap();

    file.write_at(&ones, (ones.len() * index + offset) as u64)
        .expect("Error removing");
}

pub fn remove_element_option<T>(file_path: &str, n: Option<T>, index: usize, offset: usize)
where
    T: serde::Serialize,
{
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) // Append mode
        .open(file_path)
        .unwrap();

    let y = bincode::serialize(&n).unwrap();
    let s = size_of_val(&n);
    file.write_at(&y, (y.len() * index + offset) as u64)
        .expect("Error removing");
}

pub fn remove_elements(
    file_path: &str,
    ones: &Vec<u8>,
    index: usize,
    amount: usize,
    offset: usize,
) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) // Append mode
        .open(file_path)
        .unwrap();

    let buffer: Vec<u8> = vec![0; amount * ones.len()];
    file.write_at(&buffer, (ones.len() * index + offset) as u64)
        .expect("Error removing");
}
pub fn update_element<T>(file_name: &str, index: usize, data: &T, offset: usize)
where
    T: serde::Serialize,
{
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();
    let seri = bincode::serialize(data).unwrap();
    let off = seri.len() * index + offset;
    file.write_at(&seri, off as u64);
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

pub fn truncate_end_of_file(file_name: &String, data: Vec<u8>) -> Result<(), Box<ErrorKind>> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;
    let size = file.metadata()?.len();
    if size > data.len() as u64 {
        file.set_len(size - data.len() as u64);
    }

    Ok(())
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
