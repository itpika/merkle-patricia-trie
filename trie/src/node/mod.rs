
pub const INDICES:[&str;17] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "[17]"];

#[derive(Debug, PartialEq)]
pub enum NodeType {
    HashNode,
    ValueNode,
    ShortNode,
    FullNode,
    NullNode
}

#[derive(Clone)]
pub struct NilNode;
impl NilNode {
    pub fn new() -> Self {
        NilNode
    }
}

impl NilNode {
    // type MyType = NilNode;
    pub(crate) fn cache(&self) -> (Option<HashNode>, bool) {
        (None, false)
    }

    pub(crate) fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
    }

    pub(crate) fn fstring(&self, v: String) -> String {
        String::default()
    }

    pub(crate) fn kind(&self) -> NodeType {
        NodeType::NullNode
    }
}

pub(crate) struct NodeFlag {
    pub(crate) hash: Option<HashNode>, // 表示是否计算有hash数据
    pub(crate) dirty: bool, 
}

impl Clone for NodeFlag {
    fn clone(&self) -> Self {
        match &self.hash {
            Some(v) => Self { hash: Some(v.clone()), dirty: (self.dirty) },
            None => Self { hash: None, dirty: (self.dirty) },
        }
    }
}

impl NodeFlag {
    pub fn default() -> Self {
        NodeFlag { hash: None, dirty: false }
    }
    pub fn get_hash_node(&self) -> Option<HashNode> {
        match &self.hash {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

pub mod full_node;
use std::cell::RefCell;
use std::rc::Rc;

pub use full_node::FullNode;

pub mod hash_node;
pub use hash_node::HashNode;


pub mod short_node;
pub use short_node::ShortNode;

pub mod value_node;
pub use value_node::ValueNode;
pub use value_node::NIL_VALUE_NODE;

use crate::NodeError;
use crate::writer::EncodeBuffer;