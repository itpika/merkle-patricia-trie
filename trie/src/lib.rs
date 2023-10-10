use std::{rc::Rc, cell::RefCell, fmt, error::Error};

use  common::Hash;
use node::{Node, NodeType, NilNode, ValueNode, FullNode, HashNode, ShortNode};

pub struct ID {
    state_root: Hash,
    owner: Hash,
    root: Hash
}

impl ID {
    pub fn state_trie_id(root: Hash) -> Self {
        ID { state_root: root, owner: Hash::default(), root: Hash::default() }
    }
    pub fn trie_id(root: Hash) -> Self {
        ID { state_root: root, owner: Hash::default(), root: Hash::default() }
    }
    pub fn storage_trie_id(state_root: Hash, owner: Hash, root: Hash) -> Self {
        ID { state_root: state_root, owner: owner, root: root }
    }
}

#[derive(Debug,Clone)]
pub struct NodeError(String);
impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for NodeError {}

impl NodeError {
    fn from(v: &str) -> Self {
        NodeError(v.to_string())
    }
}


pub struct Trie {
    // root: T::MyType,
    // root: Rc<RefCell<dyn Node>>,
    pub root: Rc<dyn Node>,
    // root_full_node: Option<FullNode>,
    // root_short_node: Option<ShortNode>,
    // root_hash_node: Option<HashNode>,
    // root_value_node: Option<ValueNode>,
    // root: Rc<RefCell<T>>,
    // root: T,
    owner: Hash,

    unhashed: u64
}

impl Trie {
    pub fn new(id: ID) -> Self {
        // Trie { root: Rc::new(RefCell::new(NilNode)), owner: id.owner, unhashed: 0, root_full_node: None, root_short_node: None, root_hash_node: None, root_value_node: None }
        // Trie { root: Rc::new(NilNode), owner: id.owner, unhashed: 0, root_full_node: None, root_short_node: None, root_hash_node: None, root_value_node: None }
        Trie { root: Rc::new(NilNode), owner: id.owner, unhashed: 0 }
    }
    // pub fn try_get_full_node(&self) -> Result<&FullNode, NodeError> {
    //     match &self.root_full_node {
    //         Some(full_node) => Ok(full_node),
    //         None => Err(NodeError(String::from("not found full node")))
    //     }
    // }
    // pub fn try_get_short_node(&self) -> Result<&ShortNode, NodeError> {
    //     match &self.root_short_node {
    //         Some(sn) => Ok(sn),
    //         None => Err(NodeError(String::from("not found short node")))
    //     }
    // }
    // pub fn try_get_value_node(&self) -> Result<&ValueNode, NodeError> {
    //     match &self.root_value_node {
    //         Some(sn) => Ok(sn),
    //         None => Err(NodeError(String::from("not found value node")))
    //     }
    // }
    pub fn try_update(&mut self, key: Vec<u8>, value: Option<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        let key = key_to_hex(key);
        match value {
            Some(value) => {
                let (_, n) = self.insert(Rc::clone(&self.root), Vec::new(), key, Rc::new(ValueNode::new(value)))?;
                self.root = n;
            },
            None => {
                let (_, n) = self.delete(Rc::clone(&self.root), Vec::new(), key)?;
                self.root = n;
            }
        }
        Ok(())
    }
    fn new_flag(&self) -> node::NodeFlag {
        node::NodeFlag{
            hash: HashNode::default(),
            dirty: true,
        }
    }
    // 插入node
    fn insert(&self, n: Rc<dyn Node>, prefix: Vec<u8>, key: Vec<u8>, value: Rc<dyn Node>) -> Result<(bool, Rc<dyn Node>), NodeError> {
        if key.len() == 0 {
            // 如果key为空
            match n.kind() {
                NodeType::ValueNode => {
                    let vn = n.into_value_node()?;
                    let val_node = value.into_value_node()?;
                    // *self.root_value_node = &Some(val_node);
                    return Ok((!vn.equal(val_node), Rc::clone(&value)));
                },
                _ => {
                    return Ok((true, Rc::clone(&value)));
                },
            }
        } else {
            // println!("kind {:?}", n.kind());
            match n.kind() {
                NodeType::NullNode => {
                    return Ok((true, Rc::new(ShortNode::new(key, Rc::clone(&value), self.new_flag()))));
                }
                NodeType::ShortNode => {
                    let n = n.into_short_node()?;
                    // let n = self.try_get_short_node()?;
                    let match_len = prefix_len(key.clone(), n.key.clone());
                    // 相同长度等于key
                    let mut next_prefix = prefix.clone();
                    if match_len == n.key.len() {
                        // next_prefix.append(&mut Vec::from(&key.clone()[..match_len]));
                        next_prefix.extend(&key[..match_len]);

                        let (dirty,nn) = self.insert(Rc::clone(&n.val), next_prefix, Vec::from(&key.clone()[match_len..]), value)?;
                        if !dirty {
                            return Ok((false, Rc::clone(&self.root)));
                        }
                        return Ok((true, Rc::new(ShortNode::new(n.key.clone(), nn, self.new_flag()))));
                    }

                    let mut branch = FullNode::from(self.new_flag());
                    
                    next_prefix.extend(&n.key[..match_len+1]);
                    let (_, n1) = self.insert(Rc::new(NilNode), next_prefix, Vec::from(&n.key[match_len+1..]), Rc::clone(&n.val))?;
                    branch.children[n.key[match_len] as usize] = Some(n1);
                    
                    let mut next_prefix2 = prefix.clone();
                    next_prefix2.extend(&key[..match_len+1]);
                    let (_, n2) = self.insert(Rc::new(NilNode), next_prefix2, Vec::from(&key[match_len+1..]), Rc::clone(&value))?;
                    branch.children[key[match_len] as usize] = Some(n2);

                    if match_len == 0 { // key没有相同前缀，作为分支节点返回
                        return Ok((true, Rc::new(branch)));
                    }

                    return Ok((true, Rc::new(ShortNode::new(Vec::from(&key[..match_len]), Rc::new(branch), self.new_flag()))));
                }
                NodeType::ValueNode => {
                    return Err(NodeError::from("invalid node"))
                },
                NodeType::HashNode => {
                    return Err(NodeError::from("insert HashNode todo"))
                },
                NodeType::FullNode => {
                    // let n = self.try_get_full_node()?;
                    let n = n.into_full_node()?;
                    // 获取key[0]插槽位置的node
                    let slot_node = match &n.children[key[0] as usize] {
                        Some(child_node) => {
                            Rc::clone(child_node)
                        },
                        None => {
                            Rc::new(NilNode)
                        }
                    };
                    let mut next_prefix = prefix.clone();
                    next_prefix.push(key[0]);
                    // 以子插槽开始，插入value
                    let (dirty, nn) = self.insert(slot_node, next_prefix, Vec::from(&key[1..]), value)?;
                    if !dirty {
                        return Ok((false, Rc::clone(&self.root)));
                    }
                    // 插槽对应位置设置成新的生成好的node
                    let mut f_n = n.into_full_node()?;
                    f_n.flags = self.new_flag();
                    f_n.children[key[0] as usize] = Some(nn);
                    return Ok((true, Rc::new(f_n)));
                },
            }
        }
    }
    
    fn delete(&self, n: Rc<dyn Node>, prefix: Vec<u8>, key: Vec<u8>) -> Result<(bool, Rc<dyn Node>), NodeError> {
        // print!(" {:?} ", n.kind());
        match n.kind() {
            NodeType::ShortNode => {
                let sn = n.into_short_node()?;
                let match_len = prefix_len(sn.key.clone(), key.clone());
                if match_len < sn.key.len() {
                    return  Ok((false, n));
                }
                if match_len == key.len() { // 公共长度等于key,匹配到了
                    return Ok((true, Rc::new(NilNode)));
                }
                let mut next_prefix = prefix.clone();
                next_prefix.extend(&key[..sn.key.len()]);
                let (dirty, child_node) = self.delete(sn.val, next_prefix, Vec::from(&key[sn.key.len()..]))?;
                if !dirty {
                    return  Ok((false, n));
                }
                match child_node.kind() {
                    NodeType::ShortNode => { // 如果也是shortNode，把key合并一下，作为一个新的shortNode
                        let child = child_node.into_short_node()?;
                        let mut new_key = sn.key.clone();
                        new_key.extend(child.key);
                        return Ok((true, Rc::new(ShortNode::new(new_key, child.val, self.new_flag()))));
                    },
                    _ => { // 如果是其它类型，直接作为shortNode的value
                        return Ok((true, Rc::new(ShortNode::new(sn.key, child_node, self.new_flag()))));
                    }
                }
            },
            NodeType::HashNode => todo!(),
            NodeType::ValueNode => {
                Ok((true, Rc::new(NilNode)))
            },
            NodeType::NullNode => {
                Ok((false, Rc::new(NilNode)))
            },
            NodeType::FullNode => {

                let mut f_n = n.into_full_node()?;
                let child_node = match &f_n.children[key[0] as usize] {
                    Some(v) => Rc::clone(v),
                    None => Rc::new(NilNode),
                };
                let mut next_prefix = prefix.clone();
                next_prefix.push(key[0]);
                let (dirty, nn) = self.delete(child_node, next_prefix, Vec::from(&key[1..]))?;

                if !dirty {
                    return Ok((false, n));
                }
                
                f_n = f_n.into_full_node()?; // copy
                f_n.flags = self.new_flag();
                
                if nn.kind() != NodeType::NullNode {
                    f_n.children[key[0] as usize] = Some(Rc::clone(&nn));
                    return Ok((true, Rc::new(f_n)));
                } else { // 返回的node为NullNode, 说明已经被删除了,子节点位置赋予None
                    f_n.children[key[0] as usize] = None;
                }
                
                // 判断fullNode的子节点数量，如果只有一个，合并返回一个shoryNode
                let mut pos = 100;
                for (i,v) in f_n.children.iter().enumerate() {
                    if let Some(_) = v {
                        if pos == 100 {
                            pos = i // 表示有一个子节点
                        } else {
                            pos = 101; // 表示有多个子节点
                            break;
                        }
                    }
                }
                // println!("pos {}", pos);

                if pos < 17 { // 含有一个子节点
                    if pos != 16 { // pos不指向最后一个子节点
                        if let Some(nn) = &f_n.children[pos] {
                            if nn.kind() == NodeType::ShortNode  {
                                let sn = nn.into_short_node()?;
                                let mut new_key = Vec::from([pos as u8]);
                                new_key.extend(sn.key);
                                return Ok((true, Rc::new(ShortNode::new(new_key, sn.val, self.new_flag()))));
                            }
                        }
                    }

                    if let Some(nn) = &f_n.children[pos] {
                        return Ok((true, Rc::new(ShortNode::new(Vec::from([pos as u8]), Rc::clone(nn), self.new_flag()))));
                    }
                }
                
                Ok((true, Rc::new(f_n)))
            },
            
        }
    }

    pub fn try_get(&mut self, n: Rc<dyn Node>, key: Vec<u8>) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let ret = self.get(n, key_to_hex(key), 0)?;
        if ret.did_resolve {
            self.root = ret.new_node;
        }
        Ok(ret.value)
    }
    fn get(&self, n: Rc<dyn Node>, key: Vec<u8>, pos: usize) -> Result<GetResult, Box<dyn Error>> {
        match n.kind() {
            NodeType::NullNode => {
                Ok(GetResult::from(None, false, Rc::new(NilNode)))
            },
            NodeType::ValueNode => {
                let vn = n.into_value_node()?;
                Ok(GetResult::from(Some(vn.0), false, n))
            },
            NodeType::ShortNode => {
                let mut sn = n.into_short_node()?;
                if sn.key.len() > key.len()- pos ||  !sn.key.eq(&Vec::from(&key[pos..(pos + sn.key.len())])){
                    return Ok(GetResult::from(None, false, n));
                }
                let ret = self.get(Rc::clone(&sn.val), key, pos + sn.key.len())?;
                if ret.did_resolve {
                    sn = n.into_short_node()?;
                    sn.val = ret.new_node;
                }
                return Ok(GetResult::from(ret.value, ret.did_resolve, Rc::new(sn)));
            },
            NodeType::FullNode => {
                let mut f_n = n.into_full_node()?;
                let child_node = match &f_n.children[key[pos] as usize] {
                    Some(v) => Rc::clone(v),
                    None => Rc::new(NilNode),
                };

                let ret = self.get(child_node, key.clone(), pos+1)?;
                if ret.did_resolve {
                    f_n = n.into_full_node()?;
                    f_n.children[key[pos] as usize] = Some(ret.new_node);
                }
                return Ok(GetResult::from(ret.value, ret.did_resolve, Rc::new(f_n)));
            },
            NodeType::HashNode => todo!(),
        }
    }
}



pub struct GetResult {
    value: Option<Vec<u8>>,
    did_resolve: bool,
    new_node: Rc<dyn Node>,
}
impl GetResult {
    pub fn from(value: Option<Vec<u8>>, did_resolve: bool, new_node: Rc<dyn Node>) -> GetResult{
        GetResult{value, did_resolve, new_node}
    }
}

// key扩展
pub(crate) fn key_to_hex(key: Vec<u8>) -> Vec<u8> {
    let mut bt: Vec<u8> = Vec::new();
    for v in key.iter() {
        bt.push(v/16);
        bt.push(v%16);
    }
    bt.push(16);
    bt
}

// key的相同前缀长度
pub(crate) fn prefix_len(a:Vec<u8>, b: Vec<u8>) -> usize {
    let mut i = 0_usize;
    let mut length = a.len();
    if b.len() < length {
        length = b.len()
    }
    while i < length {
        match a.get(i) {
            Some(a_v) => {
                match b.get(i) {
                    Some(b_v) => {
                        if a_v != b_v {
                            break;
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        i += 1;
    }
    i
}

pub mod node;
pub mod common;

