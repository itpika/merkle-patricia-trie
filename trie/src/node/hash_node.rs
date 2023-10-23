use std::{rc::Rc, cell::RefCell};
use crate::writer::EncodeBuffer;

pub struct HashNode (pub(crate) [u8; 32]);
// type HashNode2 = Box<[u8; 32]>;

// impl Node for HashNode2 {

//     fn cache(&self) -> (Option<HashNode>, bool) {
//         return (None, true);
//     }

//     fn encode(&self, mut w: Box<dyn Write>) {
//         let _ = w.write(self.as_slice());
//     }

//     fn fstring(&self, _: String) -> String {
//         format!("<{}>", hex::encode(self.as_slice()))
//     }
// }

impl HashNode {
    pub fn new() -> Self {
        HashNode([0_u8;32])
    }
    pub fn from(v: [u8;32]) -> Self {
        HashNode(v)
    }
    pub fn default() -> Self {
        HashNode([0_u8;32])
    }
    pub fn copy(&self) -> Self {
        HashNode(self.0.clone())
    }
}


impl HashNode {
    // type MyType = HashNode;
    pub(crate) fn cache(&self) -> (Option<HashNode>, bool) {
        return (None, true);
    }

    pub(crate) fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
        let mut wri = w.borrow_mut();
        wri.write_bytes(self.0.as_slice());
    }
    pub(crate) fn kind(&self) -> super::NodeType {
        super::NodeType::HashNode
    }
    pub(crate) fn fstring(&self, _: String) -> String {
        format!("<{}>", hex::encode(self.0))
    }

    pub(crate) fn into_hash_node(&self) -> HashNode {
        HashNode(self.0)
    }
}

