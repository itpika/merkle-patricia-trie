
pub const INDICES:[&str;17] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "[17]"];

#[derive(Debug, PartialEq)]
pub enum NodeType {
    HashNode,
    ValueNode,
    ShortNode,
    FullNode,
    NullNode
}
// pub trait Node<T: Node+Clone> {
pub trait Node {
    // type MyType;
    fn cache(&self) -> (Option<HashNode>, bool);
    // fn encode(&self, w: Rc<RefCell<dyn std::io::Write>>) -> io::Result<usize>;
    fn encode(&self, w: Rc<RefCell<EncodeBuffer>>);
    fn fstring(&self, v: String) -> String;
    fn kind(&self) -> NodeType;
    fn into_value_node(&self) -> Result<ValueNode, NodeError> {
        Err(NodeError::from("not found"))
    }
    fn into_hash_node(&self) -> Result<HashNode, NodeError> {
        Err(NodeError::from("not found"))
    }
    fn into_full_node(&self) -> Result<FullNode, NodeError> {
        Err(NodeError::from("not found"))
    }
    fn into_short_node(&self) -> Result<ShortNode, NodeError> {
        Err(NodeError::from("not found"))
    }
    fn to_string(&self) -> String {
        self.fstring(String::default())
    }
}

// impl<T> NodeClone for T where T: Node + Clone {}

#[derive(Clone)]
pub struct NilNode;
impl NilNode {
    pub fn new() -> Self {
        NilNode
    }
}

impl Node for NilNode {
    // type MyType = NilNode;
    fn cache(&self) -> (Option<HashNode>, bool) {
        (None, false)
    }

    fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
    }

    fn fstring(&self, v: String) -> String {
        String::default()
    }

    fn kind(&self) -> NodeType {
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
            Some(v) => Self { hash: Some(v.copy()), dirty: (self.dirty) },
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
            Some(v) => Some(v.copy()),
            None => None,
        }
    }
}

pub mod full_node;
use std::cell::RefCell;
use std::io;
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