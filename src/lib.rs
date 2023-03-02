use std::path::Path;
use thiserror::Error;

pub mod db;
pub mod ivec;
pub use ivec::IVec;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown data store error")]
    Unknown,
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn open<P: AsRef<Path>>(path: P) -> db::Db {
    db::Db {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
