
use std::{io::{Write, self}, rc::Rc, cell::RefCell};

use super::{Node};


pub struct ShortNode {
    pub(crate) key: Vec<u8>,
    pub(crate) val: Rc<dyn Node>,
    pub(crate) flags: super::NodeFlag,
}

impl ShortNode {
    pub fn new(key: Vec<u8>, val: Rc<dyn Node>, flags: super::NodeFlag) -> Self {
        ShortNode{key: key, val: val, flags: flags}
    }
}

// impl <T: Node + Clone> Clone for ShortNode<T> {
//     fn clone(&self) -> Self {
//         Self { key: self.key.clone(), val: self.val.clone(), flags: self.flags.clone() }
//     }

// }

impl Node for ShortNode {
    // type MyType = ShortNode<T>;
    fn cache(&self) -> (Option<HashNode>, bool) {
        return (Some(self.flags.hash.copy()), self.flags.dirty);
    }

    fn encode(&self, w: Rc<RefCell<dyn Write>>) -> io::Result<usize> {
        let mut w = w.borrow_mut();
        w.write(self.key.as_slice())
        // self.val.encode(w)
    }

    fn fstring(&self, ind: String) -> String {
        let k_str = self.key.as_slice();
        let v_str = self.val.as_ref().fstring(ind+"  ");
        format!("{{{}: {}}} ", hex::encode(&k_str), v_str)
    }

    fn kind(&self) -> super::NodeType {
        super::NodeType::ShortNode
    }

    fn into_short_node(&self) -> Result<ShortNode, crate::NodeError> {
        Ok(ShortNode { key: self.key.clone(), val: Rc::clone(&self.val), flags: self.flags.clone() })
    }
}

pub use super::HashNode;