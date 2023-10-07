use std::{io::{Write, self}, rc::Rc, cell::RefCell};

use super::Node;

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

impl Node for ValueNode {
    // type MyType = ValueNode;
    fn cache(&self) -> (Option<HashNode>, bool) {
        return (None, true);
    }

    fn encode(&self, w: Rc<RefCell<dyn Write>>) -> io::Result<usize> {
        let mut w = w.borrow_mut();
        w.write(self.0.as_slice())
    }

    fn fstring(&self, _: String) -> String {
        format!("{} ", hex::encode(self.0.as_slice()))
    }

    fn kind(&self) -> super::NodeType {
        super::NodeType::ValueNode
    }
    fn into_value_node(&self) -> Result<ValueNode, crate::NodeError> {
        Ok(ValueNode(self.0.clone()))
    }
}

pub use super::HashNode;