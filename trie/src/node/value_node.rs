use std::{rc::Rc, cell::RefCell};

use crate::writer::EncodeBuffer;

pub const NIL_VALUE_NODE: ValueNode = ValueNode(Vec::new());

pub struct ValueNode (pub Vec<u8>);

pub trait ToValueNode {
    fn to_value_node(&self) -> ValueNode;
}

impl ToValueNode for Vec<u8> {
    fn to_value_node(&self) -> ValueNode {
        ValueNode((&self).to_vec())
    }
}

impl ToValueNode for &[u8] {
    fn to_value_node(&self) -> ValueNode {
        ValueNode((&self).to_vec())
    }
}

impl ValueNode {
    pub fn new<T: ToValueNode>(v: T) -> Self {
        v.to_value_node()
    }
    pub fn default() -> Self {
        ValueNode(Vec::new())
    }
    pub fn copy(&self) -> ValueNode {
        ValueNode(self.0.clone())
    }
    pub fn equal(&self, v: Self) -> bool {
       self.0.eq(&v.0)
    }
}

impl ValueNode {
    // type MyType = ValueNode;
    pub(crate) fn cache(&self) -> (Option<HashNode>, bool) {
        return (None, true);
    }

    pub(crate) fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
        let mut wri = w.borrow_mut();
        wri.write_bytes(self.0.as_slice());
    }


    pub(crate) fn fstring(&self, _: String) -> String {
        format!("{} ", hex::encode(self.0.as_slice()))
    }

    pub(crate) fn kind(&self) -> super::NodeType {
        super::NodeType::ValueNode
    }
    pub(crate) fn into_value_node(&self) -> ValueNode {
        ValueNode(self.0.clone())
    }
}

pub use super::HashNode;