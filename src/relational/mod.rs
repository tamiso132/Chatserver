use std::vec;

use serde_derive::{Deserialize, Serialize};

use self::types::*;
use crate::storage::{self, storage_save};

pub(super) mod id;
pub(super) mod types;

#[derive(Deserialize, Serialize)]
pub struct TableInfo {
    table_id: u16,
    element_size: u16,
}
impl TableInfo {
    pub fn buffer_size() -> u16 {
        let info_len = bincode::serialize(&TableInfo {
            table_id: 0,
            element_size: 0,
        })
        .unwrap()
        .len();
        info_len as u16
    }
}

macro_rules! table_init {
    ($type:ident, $element:ident, $table_id:expr) => {
        struct $type {
            vec: Vec<Option<$element>>,
            id: u16,
        }
        impl $type {
            fn new() -> Self {
                Self {
                    vec: Vec::new(),
                    id: $type::get_table_id(),
                }
            }
            fn get_table_id() -> u16 {
                $table_id
            }

            fn push(&mut self, data: $element) {
                self.vec.push(Some(data));
            }

            fn push_many(&mut self, mut data: Vec<Option<$element>>) {
                self.vec.append(&mut data);
            }

            fn update(&mut self, index: usize, new_val: $element) {
                if self.vec.len() < index {
                    self.vec[index] = Some(new_val);
                }
            }
            fn get_all_ref(&self) -> &Vec<Option<$element>> {
                &self.vec
            }
        }
    };
}

macro_rules! table_only_push {
    ($type:ident, $element:ident, $table_id:expr) => {
        struct $type {
            vec: Vec<$element>,
            id: u16,
        }
        impl $type {
            fn new() -> Self {
                Self {
                    vec: Vec::new(),
                    id: $type::get_table_id(),
                }
            }
            fn get_table_id() -> u16 {
                $table_id
            }

            fn push(&mut self, data: $element) {
                self.vec.push(data);
            }

            fn push_many(&mut self, mut data: Vec<$element>) {
                self.vec.append(&mut data)
            }
        }
    };
}

table_init!(Categories, Category, 0);
table_init!(Brands, Brand, 1);
table_init!(Products, Product, 2);

table_only_push!(Receipts, Receipt, 3);

#[derive(Serialize, Deserialize)]
pub struct Free {
    pub start: u16,
    pub end: u16,
}
impl Default for Free {
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FreeList {
    name: String,
    size: u32,
    vec: Vec<Free>,
    offset: usize,
}
impl FreeList {
    fn new(name: String) -> Self {
        let d = Free { start: 0, end: 0 };
        let mut v = vec![];
        let s: u32;
        let offset = bincode::serialize(&(5 as u32)).unwrap();
        match storage_save::read_chunk::<u32>(&name, offset.clone(), 0, 1, 0) {
            Ok(v) => s = v[0],
            Err(_) => s = 0,
        }
        match storage_save::read_all_file::<Free>(&name, offset.len()) {
            Ok(list) => {
                v = list;
            }
            Err(_) => {
                v.push(d);
            }
        }

        Self {
            name: name,
            size: s,
            vec: v,
            offset: offset.len(),
        }
    }
    pub fn get_size(&self) -> u16 {
        self.size as u16
    }
    pub fn get_vec(&self) -> &Vec<Free> {
        &self.vec
    }
    fn push(&mut self) -> Option<u16> {
        self.size += 1;
        storage_save::update_element(&self.name, 0, &self.size, 0);
        let len = self.vec.len();
        let mut ret = None;
        if self.vec.len() > 1 {
            self.vec[len - 1].start -= 1;
            ret = Some(self.vec[len - 1].start);
            if self.vec[len - 2].end + 1 == self.vec[len - 1].start {
                self.vec[len - 2].end = self.vec[len - 1].end;
                let _ = storage_save::truncate_end_of_file(
                    &self.name,
                    bincode::serialize(&self.vec[0]).unwrap(),
                );
                storage_save::update_element(&self.name, len - 2, &self.vec[len - 2], self.offset);
                self.vec.pop();
                return ret;
            }
            storage_save::update_element(&self.name, len - 1, &self.vec[len - 1], self.offset);
        } else {
            self.vec[0].end += 1;
            storage_save::update_element(&self.name, 0, &self.vec[0], self.offset);
        }
        ret
    }
    fn remove(&mut self, index_remove: u16) {
        for index in 0..self.vec.len() {
            if index_remove < self.vec[index].end {
                self.size -= 1;
                storage_save::update_element(&self.name, 0, &self.size, self.offset);
                if index_remove == self.vec[index].start {
                    self.vec[index].start += 1;
                    storage_save::update_element(
                        &self.name,
                        index,
                        &bincode::serialize(&self.vec[index]).unwrap(),
                        self.offset,
                    );
                    return;
                } else {
                    // split into two elements
                    let first_part = Free {
                        start: self.vec[index].start,
                        end: index_remove - 1,
                    };
                    self.vec[index].start = index_remove + 1;
                    storage_save::update_element(&self.name, index, &self.vec[index], self.offset);
                    storage_save::insert_at_index(&self.name, index, &first_part, self.offset);

                    self.vec.insert(index, first_part);
                    return;
                }
            }
        }
    }
    fn print_all(&self) {
        for index in 0..self.vec.len() {
            println!(
                "index:{index}\nBetween {} - {}\n",
                self.vec[index].start, self.vec[index].end
            );
        }
    }
}

pub struct STable<T>
where
    T: for<'a> serde::Deserialize<'a> + serde::Serialize + Default,
{
    // can have a STable of every category
    vec: Vec<Option<T>>,
    free: FreeList,
    filename: String,
}

impl<T> STable<T>
where
    T: for<'a> serde::Deserialize<'a> + serde::Serialize + Default,
{
    fn new(name: &str) -> STable<T> {
        let free_list = FreeList::new(name.to_owned() + "_freelist");
        let vec_list;
        match storage::storage_save::read_all_file_option::<T>(name, &free_list, 0) {
            Ok(list) => {
                vec_list = list;
            }
            Err(_) => vec_list = vec![],
        }
        Self {
            vec: vec_list,
            free: free_list,
            filename: name.to_string(),
        }
    }
    fn push(&mut self, data: T) -> u16 {
        let ret;
        match self.free.push() {
            Some(index) => {
                storage::storage_save::update_element(&self.filename, index as usize, &data, 0);
                self.vec[index as usize] = Some(data);
                ret = index;
            }
            None => {
                let _ = storage::storage_save::push_to_file(&self.filename, &Some(&data));
                self.vec.push(Some(data));
                ret = (self.vec.len() - 1) as u16;
            }
        }
        ret
    }
    fn update(&mut self, index: usize, data: T) {
        if self.vec.len() < index {
            match self.vec[index] {
                Some(_) => {
                    storage::storage_save::update_element(&self.filename, index, &data, 0);
                    self.vec[index] = Some(data)
                }
                None => {}
            }
        }
    }
    fn remove(&mut self, index_remove: u16) {
        if self.vec.len() < index_remove as usize {
            return;
        }
        self.free.remove(index_remove as u16);
        self.vec[index_remove as usize] = None;
        storage_save::remove_element_option::<T>(&self.filename, None, index_remove as usize, 0);
    }
    /// for debug purposes only
    fn print_free(&self) {
        self.free.print_all();
    }
}

pub fn bench_test() -> STable<Product> {
    let mut x: STable<Product> = STable::new("file_path");
    x
}

#[cfg(test)]
#[test]
fn it_works() {}
