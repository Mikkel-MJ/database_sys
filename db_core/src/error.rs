#[derive(thiserror::Error,Debug)]
pub enum DatastoreError {
    #[error("Datastore error: {0}")]
    Generic(String),
}