use bincode::{deserialize, serialize, ErrorKind};
use std::marker::PhantomData;

use chrono::{DateTime, Utc};

use super::FileQueueGlobal;

pub struct Id {
    id: u64, // first 16 bit also is the filename and unique table id, table, next 48 bits are for rows id
}

pub struct Column {
    number_of_elements: u64,
    pointer: *mut u8,
}

pub struct Table<T> {
    file_queue: FileQueueGlobal<T>,
    max_allocated: u32,
    data: Vec<T>,
}

impl<T> Table<T> {
    pub fn push_record(&mut self, element: T) {
        self.data.push(element);
        //     self.file_queue.add_append(vec![element]);
        // sync
    }
}
