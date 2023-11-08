// this is being used by my rasperry pi

mod db;

use std::mem::{self, align_of, size_of, transmute};
use std::ptr;

use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[repr(C, packed)]
pub struct Test {
    data: u32,
    data2: u8,
    data3: u16,
}

pub struct FileQueue {
    append: Vec<u8>,
}

pub async fn push_file<T>(data: Vec<T>, filename: String) -> io::Result<()> {
    let mut options = OpenOptions::new();
    let file = options.write(true).append(true).open(filename);

    let list: &[u8] = &to_bytes_vec(&data);

    file.await?.write(list).await?;
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
