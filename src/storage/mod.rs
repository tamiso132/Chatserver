pub(crate) mod db;

pub fn register_new_user(firstname:String, lastname:String, username:String, password:String) -> Result<(), db::StorageError>{
    db::UserLogin::create_user(firstname, lastname, username, password)
}