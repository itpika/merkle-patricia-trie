
use std::{io::{Write}, rc::Rc, cell::RefCell};

use crate::{writer::EncodeBuffer, TrieNode};

pub struct ShortNode {
    pub(crate) key: Vec<u8>,
    pub(crate) val: Rc<TrieNode>,
    pub(crate) flags: super::NodeFlag,
}

impl ShortNode {
    pub(crate) fn new(key: Vec<u8>, val: Rc<TrieNode>, flags: super::NodeFlag) -> Self {
        ShortNode{key: key, val: val, flags: flags}
    }
}

// impl <T: Node + Clone> Clone for ShortNode<T> {
//     fn clone(&self) -> Self {
//         Self { key: self.key.clone(), val: self.val.clone(), flags: self.flags.clone() }
//     }

// }

impl ShortNode {
    // type MyType = ShortNode<T>;
    pub(crate) fn cache(&self) -> (Option<HashNode>, bool) {
        return (self.flags.get_hash_node(), self.flags.dirty);
    }

    pub(crate) fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
        {
            let mut wri = w.borrow_mut();
            wri.write_bytes(self.key.as_slice());
        }

        if self.val.kind() != super::NodeType::NullNode {
            // self.val.encode(Rc::clone(&w));
            self.val.encode(w);
        } else {
            w.borrow_mut().write(0x80);
        }
    }

    pub(crate) fn fstring(&self, ind: String) -> String {
        let k_str = self.key.as_slice();
        let v_str = self.val.fstring(ind+"  ");
        format!("{{{}: {}}} ", hex::encode(&k_str), v_str)
    }

    pub(crate) fn kind(&self) -> super::NodeType {
        super::NodeType::ShortNode
    }

    pub(crate) fn into_short_node(&self) -> ShortNode {
        ShortNode { key: self.key.clone(), val: Rc::clone(&self.val), flags: self.flags.clone() }
    }
}

pub use super::HashNode;