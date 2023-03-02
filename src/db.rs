pub struct Db {}
pub struct Tree {}
use crate::Result;

impl Db {
    pub fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Tree> {
        todo!()
    }
}
