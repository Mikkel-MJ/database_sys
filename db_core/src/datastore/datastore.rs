use crate::error::DatastoreError;
use super::kv::{Val, Key};
pub struct Datastore {
    pub store : DS,
}

pub enum DS {
    RocksDB(super::rocks_db::Datastore)
}

pub trait KVCommands {
    /// Add a key-value pair to the datastorage 
    fn add_kv(&self,key:Key,value:Val) -> Result<(),DatastoreError>;
    /// Get a key-value pair from the datastorage 
    fn get_kv(&self,key:Key) -> Result<Val,DatastoreError>;
}





