use crate::tree::{Node, Cask};
use crate::Result;

pub struct Db {}

impl Db {
    pub fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Cask> {
        todo!()
    }
}
