use std::sync::Arc;
use crate::IVec;

#[derive(Clone)]
pub struct Cask(pub(crate) Arc<TreeInner>);
pub struct TreeInner {
    pub(crate) root: IVec
}
impl TreeInner {}
pub(crate) struct Node {}

pub(crate) struct Cell {}
