// this is being used by my rasperry pi

mod db;

use std::io::SeekFrom;
use std::mem::{self, align_of, size_of, transmute};
use std::ptr;

use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[repr(C, packed)]
pub struct Test {
    data: u32,
    data2: u8,
    data3: u16,
}

struct OverwriteInfo<T> {
    offset: u64,
    data: Vec<T>,
}
pub struct Overwrite<T> {
    offset: u64,
    over_write: Vec<OverwriteInfo<T>>,
}

impl<T> Overwrite<T> {
    fn new() -> Self {
        Self {
            offset: 0,
            over_write: vec![],
        }
    }
}

pub struct FileQueueGlobal<T> {
    filename: String,
    append: Mutex<Vec<T>>,
    overwrite: Mutex<Overwrite<T>>,
}

impl<T> FileQueueGlobal<T> {
    fn new(filename: String) -> Self {
        let overwrite: Overwrite<T> = Overwrite::new();
        Self {
            filename,
            append: Mutex::new(vec![]),
            overwrite: Mutex::new(overwrite),
        }
    }

    fn run() {}
}
pub async fn run_queue_write<T>(file_queue: FileQueueGlobal<T>) -> io::Result<()> {
    let mut options = OpenOptions::new();
    let file = options.write(true).open(file_queue.filename);

    let mut write_data: Vec<u8> = Vec::with_capacity(10000); // Hopefully it never goes beyond otherwise big panic lmfao
    let mut offset: Vec<(u64, usize, usize)> = vec![];
    {
        // Creating contingious write buffer
        let overwrite: &mut Vec<OverwriteInfo<T>> =
            &mut file_queue.overwrite.lock().await.over_write;
        for i in 0..overwrite.len() {
            let (buffer_size, start_buffer_index) =
                to_bytes_append_vec(&mut overwrite[i].data, &mut write_data);
            offset.push((overwrite[i].offset, buffer_size, start_buffer_index));
        }
        overwrite.clear();
    };

    {
        let d = file_queue.append.lock().await;
        let buffer_size = to_bytes_append_vec(&d, &mut write_data);
    }

    // Writing to file
    let mut f = file.await?;
    for i in 0..offset.len() {
        let buffer_start = offset[i].2;
        let buffer_size = offset[i].1;
        let file_offset = offset[i].0;

        f.seek(SeekFrom::Start(file_offset)).await?;
        f.write(&write_data[buffer_start..buffer_start + buffer_size])
            .await?;
    }

    let buffer_start = offset[offset.len() - 1].2 + offset[offset.len() - 1].1;

    f.seek(SeekFrom::End(0)).await?;
    f.write(&write_data[buffer_start..write_data.len() - 1])
        .await?;

    f.flush().await?;
    Ok(())
}

/// This works things with primitive fields structs only and no pointers
fn to_bytes<T>(value: &T) -> Vec<u8> {
    let size = size_of::<T>();
    let mut bytes = vec![0; size];
    unsafe {
        ptr::copy_nonoverlapping(value as *const T as *const u8, bytes.as_mut_ptr(), size);
    }
    bytes
}
///
/// continue adding to existing vector
fn to_bytes_append_vec<T>(value: &Vec<T>, list: &mut Vec<u8>) -> (usize, usize) {
    let size = size_of::<T>();
    let buffer_size = size * value.len();
    let mut start_size = list.len() - 1;

    unsafe {
        ptr::copy_nonoverlapping::<u8>(
            value.as_ptr() as *const u8,
            list.as_mut_ptr().add(list.len()),
            buffer_size,
        );
        list.set_len(buffer_size + start_size + 1);
    }
    (buffer_size, start_size)
}
///
fn to_bytes_vec<T>(value: &Vec<T>) -> Vec<u8> {
    let size = size_of::<T>();
    let buffer_size = size * value.len();

    let mut bytes: Vec<u8> = vec![0; buffer_size];
    unsafe {
        ptr::copy_nonoverlapping::<u8>(
            value.as_ptr() as *const u8,
            bytes.as_mut_ptr(),
            buffer_size,
        );
    }
    bytes
}

#[cfg(test)]
mod tests {

    use crate::storage::{to_bytes, to_bytes_vec, Test};

    #[test]
    fn it_works() {
        let x: [u8; 256] = [0; 256];
        let xyz: Vec<u8> = vec![0; 256];
        let zyz: &[u8] = &to_bytes_vec(&xyz);

        assert_eq!(x, zyz);
    }
}
