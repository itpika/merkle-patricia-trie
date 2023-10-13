use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io;
use std::ops::Add;
use std::rc::Rc;

use crate::writer::EncodeBuffer;

use super::{Node, NodeFlag};
use super::INDICES;

// enum NodeValue<T: Node+Clone> {
//     Val(Box<T>),
//     Nil,
// }
// impl<T: Node + Clone> Clone for NodeValue<T> {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Val(v) => Self::Val(v.clone()),
//             Self::Nil => NodeValue::Nil,
//         }
//     }
// }

pub struct FullNode {
    pub(crate) children: [Option<Rc<dyn Node>>;17],
    pub(crate) flags: super::NodeFlag,
}

impl FullNode {
    pub fn default() -> Self {
        FullNode { children: [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None], flags: NodeFlag::default() }
    }
    pub(crate) fn from(flags: NodeFlag) -> Self {
        FullNode { children: [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None], flags: flags }
    }
}

// impl <T: Node+Clone> Clone for FullNode<T> {
//     fn clone(&self) -> Self {
//         Self { children: self.children.clone(), flags: self.flags.clone() }
//     }
// }

impl Node for FullNode {
    // type MyType = FullNode<T>;
    fn cache(&self) -> (Option<HashNode>, bool) {
        (self.flags.get_hash_node(), self.flags.dirty)
    }
    fn kind(&self) -> super::NodeType {
        super::NodeType::FullNode
    }

    fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
        for v in self.children.iter() {
            match v {
                None => {
                    let mut w_clone = w.as_ref().borrow_mut();
                    w_clone.write(0x80);
                },
                Some(node) => {
                    node.encode(Rc::clone(&w));
                },
            }
        }
    }

    fn fstring(&self, ind: String) -> String {
        let mut resp = format!("[\n{}  ", ind);
        for (i,v) in self.children.iter().enumerate() {
            match v.as_ref() {
                None => {
                    resp = resp.add(format!("{}: <nil> ", INDICES[i]).as_str());
                },
                Some(node) => {
                    let fmt_str = format!("{}: {}", INDICES[i].clone(), node.fstring(ind.clone()+"  "));
                    resp = resp.add(fmt_str.as_str());
                }
            }
        }
        resp.add(&format!("\n{}] ", ind))
        // resp.add(&g)
    }

    fn into_full_node(&self) -> Result<FullNode, crate::NodeError> {
        let mut cp = FullNode::default();
        for (i,v) in self.children.iter().enumerate() {
            match v {
                Some(n) => {
                    cp.children[i] = Some(Rc::clone(n));
                },
                None => {}
            }
        }
        cp.flags = self.flags.clone();
        Ok(cp)
    }
}

pub use super::HashNode;