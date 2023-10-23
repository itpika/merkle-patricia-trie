use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;

use crate::TrieNode;
use crate::writer::EncodeBuffer;

use super::{NodeFlag};
use super::INDICES;


pub struct FullNode {
    pub(crate) children: [Option<Rc<TrieNode>>;17],
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

impl FullNode {
    // type MyType = FullNode<T>;
    pub(crate) fn cache(&self) -> (Option<HashNode>, bool) {
        (self.flags.get_hash_node(), self.flags.dirty)
    }
    pub(crate) fn kind(&self) -> super::NodeType {
        super::NodeType::FullNode
    }

    pub(crate) fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
        for v in self.children.iter() {
            match v {
                None => {
                    let mut w_clone = w.borrow_mut();
                    w_clone.write(0x80);
                },
                Some(node) => {
                    node.encode(Rc::clone(&w));
                },
            }
        }
    }

    pub(crate) fn fstring(&self, ind: String) -> String {
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

    pub(crate) fn into_full_node(&self) -> FullNode {
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
        cp
    }
}

pub use super::HashNode;