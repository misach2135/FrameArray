use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArrayListError {
    #[error("remove data error")]
    RemoveError,
    #[error("Internal error!")]
    Internal,
    #[error("Index out of range")]
    OutOfRange,
    #[error("Element not found")]
    ElementNotFound,
}