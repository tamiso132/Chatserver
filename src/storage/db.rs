use super::storage_save::*;
use bincode::{deserialize, serialize, ErrorKind};
use std::marker::PhantomData;

use chrono::{DateTime, Utc};

// pub struct FreeID {
//     start: usize,
//     amount: u16,
// }

// pub struct Table<T> {
//     file_name: String,
//     table_id: u16,

//     last_update: DateTime<Utc>,

//     ones: Vec<u8>,

//     d: PhantomData<T>,
// }

// impl<T> Table<T> {
//     // very unsafe storage, no checkings are done
//     fn push_element(&mut self, element: T)
//     where
//         T: serde::Serialize,
//     {
//         push_to_file(&self.file_name, &element).expect("error when adding element");
//         self.last_update = Utc::now();
//     }

//     fn update_element(&mut self, index: usize, element: T)
//     where
//         T: serde::Serialize,
//     {
//         update_specific_element(&self.file_name, index, serialize(&element).unwrap());
//         self.last_update = Utc::now();
//     }

//     fn remove_element(&mut self, index: usize) {
//         remove_element(&self.file_name, &self.ones, index);

//         self.last_update = Utc::now();
//     }
// }
