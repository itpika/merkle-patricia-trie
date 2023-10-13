
use std::{io::{Write, self}, rc::Rc, cell::RefCell};

use crate::writer::EncodeBuffer;

use super::{Node};


pub struct ShortNode {
    pub(crate) key: Vec<u8>,
    pub(crate) val: Rc<dyn Node>,
    pub(crate) flags: super::NodeFlag,
}

impl ShortNode {
    pub(crate) fn new(key: Vec<u8>, val: Rc<dyn Node>, flags: super::NodeFlag) -> Self {
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
        return (self.flags.get_hash_node(), self.flags.dirty);
    }

    // fn encode(&self, w: Rc<RefCell<dyn Write>>) -> io::Result<usize> {
    //     let mut size = 0;
    //     let mut wri = w.borrow_mut();
    //     size += wri.write(self.key.as_slice())?;
    //     if self.val.kind() != super::NodeType::NullNode {
    //         size += self.val.encode(Rc::clone(&w))?;
    //     } else {
    //         size += wri.write(&[0x80])?;
    //     }
    //     Ok(size)
    // }
    fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
        let w_clone = Rc::clone(&w);
        let mut wri = w_clone.borrow_mut();
        wri.write_bytes(self.key.as_slice());

        if self.val.kind() != super::NodeType::NullNode {
            // self.val.encode(Rc::clone(&w));
            self.val.encode(w);
        } else {
            wri.write(0x80);
        }
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