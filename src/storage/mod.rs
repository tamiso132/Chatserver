pub(crate) mod db;

pub fn register_new_user(firstname:&str, lastname:&str, username:&str, password:&str) -> Result<(u64), db::StorageError>{
    db::UserLogin::create_user(firstname.to_owned(), lastname.to_owned(), username.to_owned(), password.to_owned())
}