use crate::error::DatastoreError;

use super::{datastore::KVCommands, kv::Val};

pub struct Datastore {
    pub db: rocksdb::DB
}


impl Datastore {
    /// setup a datastore based on an embedded rocksDB database
    pub async fn new(path: &str) -> Result<Datastore,DatastoreError> {
        let mut options = rocksdb::Options::default();

        options.set_error_if_exists(false);
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        match rocksdb::DB::open(&options,path) {
            Ok(db) => Ok(Datastore { db }),
            Err(e) => Err(DatastoreError::Generic(e.to_string())),
        }
    }
}

impl KVCommands for Datastore {
    fn add_kv(&self,key:super::kv::Key,value:super::kv::Val) -> Result<(),DatastoreError> {
        match self.db.put(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(DatastoreError::Generic(format!("error saving kv pair: {e}"))),
        }

    }

    fn get_kv(&self,key:super::kv::Key) -> Result<super::kv::Val,DatastoreError> {
        match self.db.get_pinned(key) {
            Ok(v) => 
                match v {
                    Some(value) => Ok(value.to_vec()),
                    None => Err(DatastoreError::Generic("could not get value".to_owned())),
                }
            Err(e) => Err(DatastoreError::Generic(format!("failed with error: {e}"))),
        }
    }

}
