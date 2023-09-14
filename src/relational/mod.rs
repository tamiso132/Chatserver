use std::{marker::PhantomData, ops::Add, vec};

use chrono::{Datelike, Utc};
use serde_derive::{Deserialize, Serialize};

use self::types::*;
use crate::storage::{
    self,
    file::{self, FileElement, FileIntention},
};

pub(super) mod id;
pub(super) mod types;

const VERSION_NAME: &str = "_version";
const VERSION_ITEM_NAME: &str = "_old_records";
const FREE_NAME: &str = "_freelist";

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
        match storage::read_chunk::<u32>(&name, offset.clone(), 0, 1, 0) {
            Ok(v) => s = v[0],
            Err(_) => s = 0,
        }
        match storage::read_all_file::<Free>(&name, offset.len()) {
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
        let mut vec_data = vec![];

        let mut ret_val = None;
        let more_one_element = self.vec.len() > 1;

        if more_one_element {
            let last_index = self.vec.len() - 1;
            let second_to_last_index = self.vec.len() - 2;

            self.vec[last_index].start -= 1;
            ret_val = Some(self.vec[last_index].start);

            vec_data.push(FileIntention::Update(FileElement::new_with_el(
                last_index,
                self.offset,
                &self.vec[last_index],
            )));

            let overlap_elements =
                self.vec[second_to_last_index].end + 1 == self.vec[last_index].start;

            if overlap_elements {
                self.vec[second_to_last_index].end = self.vec[last_index].end;

                self.vec.pop();
                vec_data.pop();

                vec_data.push(FileIntention::Update(FileElement::new_with_el(
                    second_to_last_index,
                    self.offset,
                    &self.vec[second_to_last_index],
                )));
            }
        } else {
            self.vec[0].end += 1;
            vec_data.push(FileIntention::Update(FileElement::new_with_el(
                0,
                self.offset,
                &self.vec[0],
            )));
        }

        vec_data.push(FileIntention::Update(FileElement::new_with_el(
            0, 0, &self.size,
        )));

        let _ = storage::file::update_file(&self.name, vec_data);
        ret_val
    }
    fn remove(&mut self, index_remove: u16) {
        for index in 0..self.vec.len() {
            if index_remove < self.vec[index].end {
                let already_removed = index_remove < self.vec[index].start;
                if already_removed {
                    return;
                }

                let mut file_updates = vec![];

                self.size -= 1;

                if index_remove == self.vec[index].start {
                    self.vec[index].start += 1;

                    file_updates.push(FileIntention::Update(FileElement::new_with_el(
                        index,
                        self.offset,
                        &self.vec[index],
                    )));
                } else {
                    // split into two elements
                    let first_part = Free {
                        start: self.vec[index].start,
                        end: index_remove - 1,
                    };

                    self.vec[index].start = index_remove + 1;
                    self.vec.insert(index, first_part);

                    file_updates.push(FileIntention::Update(FileElement::new_with_el(
                        index,
                        self.offset,
                        &self.vec[index],
                    )));
                    file_updates.push(FileIntention::Insert(FileElement::new_with_el(
                        index,
                        self.offset,
                        &self.vec[index],
                    )));
                }
                file_updates.push(FileIntention::Update(FileElement::new_with_el(
                    0, 0, &self.size,
                )));
                let _ = storage::file::update_file(&self.name, file_updates);

                break;
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

pub struct VersionControl<T: serde::Serialize + serde::de::DeserializeOwned> {
    /// Keep track of current versions of all table records
    ///
    ver: Vec<u16>,
    data: PhantomData<T>,
    version_filename: String,
}
impl<T: serde::Serialize + serde::de::DeserializeOwned> VersionControl<T> {
    pub fn new(filename: &str) -> Self {
        let ver_filename = format!("{}{}", filename, VERSION_NAME);
        Self {
            ver: vec![],
            data: Default::default(),
            version_filename: ver_filename,
        }
    }
    pub fn push(&mut self) {
        self.ver.push(0);
    }
    pub fn update_empty_slot(&mut self, index: usize) {
        self.ver[index] += 1;
    }
    pub fn update_existing_slot(&mut self, index: usize, old_data: &T) {
        self.ver[index] += 1;
        let mut filename = Utc::now().year().to_string();

        filename.push_str(&Utc::now().month0().to_string());
        let _ = storage::push_to_file::<T>(&filename, old_data);
    }
    pub fn removed_existing_record(&mut self, removed_data: &T) {
        let mut filename = Utc::now().year().to_string();
        filename.push_str(&Utc::now().month0().to_string());
        let _ = storage::push_to_file::<T>(&filename, removed_data);
    }
}
pub struct TableVer<T>
where
    T: for<'a> serde::Deserialize<'a> + serde::Serialize + Default,
{
    // can have a STable of every category
    vec: Vec<Option<T>>,
    free: FreeList,
    version_control: VersionControl<T>,
    filename: String,
}
impl<T> TableVer<T>
where
    T: for<'a> serde::Deserialize<'a> + serde::Serialize + Default,
{
    fn new(name: &str) -> TableVer<T> {
        let free_list = FreeList::new(name.to_owned() + FREE_NAME);
        let vec_list;
        match storage::read_all_file_option::<T>(name, &free_list, 0) {
            Ok(list) => {
                vec_list = list;
            }
            Err(_) => vec_list = vec![],
        }
        Self {
            version_control: VersionControl::new(name),
            vec: vec_list,
            free: free_list,
            filename: name.to_string(),
        }
    }
    fn push(&mut self, data: T) -> u16 {
        let ret;
        match self.free.push() {
            Some(index) => {
                storage::update_element(&self.filename, index as usize, &data, 0);

                self.version_control.update_empty_slot(index as usize);
                self.vec[index as usize] = Some(data);
                ret = index;
            }
            None => {
                let _ = storage::push_to_file(&self.filename, &Some(&data));

                self.version_control.push();
                self.vec.push(Some(data));
                ret = (self.vec.len() - 1) as u16;
            }
        }
        ret
    }
    fn update(&mut self, index: usize, data: T) {
        if self.vec.len() < index {
            match &mut self.vec[index] {
                Some(el) => {
                    storage::update_element(&self.filename, index, &data, 0);
                    *el = data;
                    self.version_control.update_existing_slot(index, el);
                }
                None => {}
            }
        }
    }
    fn remove(&mut self, index_remove: u16) {
        if self.vec.len() < index_remove as usize {
            return;
        }
        match &self.vec[index_remove as usize] {
            Some(x) => {
                self.free.remove(index_remove);
                self.version_control.removed_existing_record(x)
            }
            None => return,
        }

        self.vec[index_remove as usize] = None;
        storage::remove_element_option::<T>(&self.filename, None, index_remove as usize, 0);
    }
    /// for debug purposes only
    fn print_free(&self) {
        self.free.print_all();
    }
}

pub fn bench_test() -> TableVer<Product> {
    let mut x: TableVer<Product> = TableVer::new("file_path");
    x.remove(50);
    x
}

#[cfg(test)]
#[test]
fn it_works() {}
