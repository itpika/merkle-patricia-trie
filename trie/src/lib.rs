use std::{rc::Rc, cell::RefCell, fmt, error::Error};

use  common::{Hash, key_to_hex, prefix_len};
use node::{NodeType, ValueNode, FullNode, HashNode, ShortNode};
use writer::EncodeBuffer;

use crate::hasher::Hasher;

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

pub enum TrieNode {
    Short(Rc<RefCell<ShortNode>>),
    Full(Rc<RefCell<FullNode>>),
    Hash(HashNode),
    Value(ValueNode),
    Nil
}

impl TrieNode {
    pub fn from_short_node(n: ShortNode) -> TrieNode {
        TrieNode::Short(Rc::new(RefCell::new(n)))
    }
    pub fn from_full_node(n: FullNode) -> TrieNode {
        TrieNode::Full(Rc::new(RefCell::new(n)))
    }
}

impl TrieNode {
    fn kind(&self) -> NodeType {
        match *self {
            TrieNode::Short(_) => NodeType::ShortNode,
            TrieNode::Full(_) => NodeType::FullNode,
            TrieNode::Hash(_) => NodeType::HashNode,
            TrieNode::Value(_) => NodeType::ValueNode,
            TrieNode::Nil => NodeType::NullNode,
        }
    }
    fn cache(&self) -> (Option<HashNode>, bool) {
        match self {
            TrieNode::Short(n) => n.borrow().cache(),
            TrieNode::Full(n) => n.borrow().cache(),
            TrieNode::Hash(n) => n.cache(),
            TrieNode::Value(n) => n.cache(),
            TrieNode::Nil => (None, false),
        }
    }
    fn encode(&self, w: Rc<RefCell<EncodeBuffer>>) {
        match self {
            TrieNode::Short(n) => n.borrow().encode(w),
            TrieNode::Full(n) => n.borrow().encode(w),
            TrieNode::Hash(n) => n.encode(w),
            TrieNode::Value(n) => n.encode(w),
            TrieNode::Nil => {},
        };
    }
    pub fn fstring(&self, v: String) -> String {
        match self {
            TrieNode::Short(n) => n.borrow().fstring(v),
            TrieNode::Full(n) => n.borrow().fstring(v),
            TrieNode::Hash(n) => n.fstring(v),
            TrieNode::Value(n) => n.fstring(v),
            TrieNode::Nil => String::default(),
        }
    }
    fn to_value_node(&self) -> ValueNode {
        match self {
            TrieNode::Value(v) => v.clone(),
            _ => {
                todo!()
            }
        }
    }
    fn to_full_node(&self) -> Rc<RefCell<FullNode>> {
        match self {
            TrieNode::Full(v) => Rc::clone(v),
            _ => {
                todo!()
            }
        }
    }
    fn to_short_node(&self) -> Rc<RefCell<ShortNode>> {
        match self {
            TrieNode::Short(v) => Rc::clone(v),
            _ => {
                todo!()
            }
        }
    }
    fn to_hash_node(&self) -> HashNode {
        match self {
            TrieNode::Hash(v) => v.clone(),
            _ => {
                todo!()
            }
        }
    }
}

pub struct Trie {
    pub root: Rc<TrieNode>,
    owner: Hash,

    unhashed: u64
}

impl Trie {
    pub fn new(id: ID) -> Self {
        Trie { root: Rc::new(TrieNode::Nil), owner: id.owner, unhashed: 0 }
    }
    pub fn try_update(&mut self, key: Vec<u8>, value: Option<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        self.unhashed += 1;
        let key = key_to_hex(key.as_slice());
        match value {
            Some(value) => {
                let (_, n) = self.insert(Rc::clone(&self.root), Vec::new(), key, Rc::new(TrieNode::Value(ValueNode::new(value))))?;
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
            hash: None,
            dirty: true,
        }
    }
    // 插入node
    fn insert(&self, n: Rc<TrieNode>, prefix: Vec<u8>, key: Vec<u8>, value: Rc<TrieNode>) -> Result<(bool, Rc<TrieNode>), NodeError> {
        if key.len() == 0 {
            // 如果key为空
            match &*n {
                TrieNode::Value(vn) => {
                    let val_node = value.to_value_node();
                    return Ok((!vn.equal(val_node), Rc::clone(&value)));
                },
                _ => {
                    return Ok((true, Rc::clone(&value)));
                },
            }
        } else {
            // println!("kind {:?}", n.kind());
            match &*n {
                TrieNode::Nil => {
                    return Ok((true, Rc::new(TrieNode::from_short_node(ShortNode::new(key, Rc::clone(&value), self.new_flag())))));
                },
                TrieNode::Short(n) => {
                    let match_len = prefix_len(key.as_slice(), n.borrow().key.as_slice());
                    // 相同长度等于key
                    let mut next_prefix = prefix.clone();
                    if match_len == n.borrow().key.len() {
                        // next_prefix.append(&mut Vec::from(&key.clone()[..match_len]));
                        next_prefix.extend(&key[..match_len]);

                        let (dirty,nn) = self.insert(Rc::clone(&n.borrow().val), next_prefix, Vec::from(&key.clone()[match_len..]), value)?;
                        if !dirty {
                            return Ok((false, Rc::clone(&self.root)));
                        }
                        return Ok((true, Rc::new(TrieNode::from_short_node(ShortNode::new(n.borrow().key.clone(), nn, self.new_flag())))));
                    }

                    let mut branch = FullNode::from(self.new_flag());
                    
                    next_prefix.extend(&n.borrow().key[..match_len+1]);
                    let (_, n1) = self.insert(Rc::new(TrieNode::Nil), next_prefix, Vec::from(&n.borrow().key[match_len+1..]), Rc::clone(&n.borrow().val))?;
                    branch.children[n.borrow().key[match_len] as usize] = Some(n1);
                    
                    let mut next_prefix2 = prefix.clone();
                    next_prefix2.extend(&key[..match_len+1]);
                    let (_, n2) = self.insert(Rc::new(TrieNode::Nil), next_prefix2, Vec::from(&key[match_len+1..]), Rc::clone(&value))?;
                    branch.children[key[match_len] as usize] = Some(n2);

                    if match_len == 0 { // key没有相同前缀，作为分支节点返回
                        return Ok((true, Rc::new(TrieNode::from_full_node(branch))));
                    }

                    return Ok((true, Rc::new(TrieNode::from_short_node(ShortNode::new(Vec::from(&key[..match_len]), Rc::new(TrieNode::from_full_node(branch)), self.new_flag())))));
                }
                TrieNode::Value(_) => {
                    return Err(NodeError::from("invalid node"))
                }
                TrieNode::Hash(_) => {
                    return Err(NodeError::from("insert HashNode todo"))
                },
                TrieNode::Full(n) => {
                    // let n = n.into_full_node()?;
                    // 获取key[0]插槽位置的node
                    let slot_node = match &n.borrow().children[key[0] as usize] {
                        Some(child_node) => {
                            Rc::clone(child_node)
                        },
                        None => {
                            Rc::new(TrieNode::Nil)
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
                    let f_n = n.clone();
                    f_n.borrow_mut().flags = self.new_flag();
                    f_n.borrow_mut().children[key[0] as usize] = Some(nn);
                    return Ok((true, Rc::new(TrieNode::Full(f_n))));
                },
            }
        }
    }

    fn delete(&self, n: Rc<TrieNode>, mut prefix: Vec<u8>, key: Vec<u8>) -> Result<(bool, Rc<TrieNode>), NodeError> {
        // print!(" {:?} ", n.kind());
        match &*n {
            TrieNode::Short(sn) => {
                // let sn = n.to_short_node();
                let match_len = prefix_len(sn.borrow().key.as_slice(), key.as_slice());
                if match_len < sn.borrow().key.len() {
                    return  Ok((false, n));
                }
                if match_len == key.len() { // 公共长度等于key,匹配到了
                    return Ok((true, Rc::new(TrieNode::Nil)));
                }
                prefix.extend(&key[..sn.borrow().key.len()]);
                let (dirty, child_node) = self.delete(Rc::clone(&sn.borrow().val), prefix, Vec::from(&key[sn.borrow().key.len()..]))?;
                if !dirty {
                    return  Ok((false, n));
                }
                match &*child_node {
                    TrieNode::Short(child) => { // 如果也是shortNode，把key合并一下，作为一个新的shortNode
                        // let child = child_node.to_short_node();
                        let mut new_key = sn.borrow().key.clone();
                        new_key.extend_from_slice(child.borrow().key.as_slice());
                        return Ok((true, Rc::new(TrieNode::from_short_node(ShortNode::new(new_key, Rc::clone(&child.borrow().val), self.new_flag())))));
                    },
                    _ => { // 如果是其它类型，直接作为shortNode的value
                        return Ok((true, Rc::new(TrieNode::from_short_node(ShortNode::new(sn.borrow().key.clone(), child_node, self.new_flag())))));
                    }
                }
                // match child_node.kind() {
                //     NodeType::ShortNode => { // 如果也是shortNode，把key合并一下，作为一个新的shortNode
                //         let child = child_node.to_short_node();
                //         let mut new_key = sn.key.clone();
                //         new_key.extend(child.key);
                //         return Ok((true, Rc::new(TrieNode::Short(ShortNode::new(new_key, child.val, self.new_flag())))));
                //     },
                //     _ => { // 如果是其它类型，直接作为shortNode的value
                //         return Ok((true, Rc::new(TrieNode::Short(ShortNode::new(sn.key.clone(), child_node, self.new_flag())))));
                //     }
                // }
            },
            TrieNode::Hash(_) => todo!(),
            TrieNode::Value(_) => {
                Ok((true, Rc::new(TrieNode::Nil)))
            },
            TrieNode::Nil => {
                Ok((false, Rc::new(TrieNode::Nil)))
            },
            TrieNode::Full(f_n) => {

                // let mut f_n = n.to_full_node();
                let child_node = match &f_n.borrow().children[key[0] as usize] {
                    Some(v) => Rc::clone(v),
                    None => Rc::new(TrieNode::Nil),
                };
                prefix.push(key[0]);
                let (dirty, nn) = self.delete(child_node, prefix, Vec::from(&key[1..]))?;

                if !dirty {
                    return Ok((false, n));
                }
                
                let f_n = f_n.clone();
                // f_n = f_n.into_full_node()?; // copy
                f_n.borrow_mut().flags = self.new_flag();
                
                if nn.kind() != NodeType::NullNode {
                    f_n.borrow_mut().children[key[0] as usize] = Some(Rc::clone(&nn));
                    return Ok((true, Rc::new(TrieNode::Full(f_n))));
                } else { // 返回的node为NullNode, 说明已经被删除了,子节点位置赋予None
                    f_n.borrow_mut().children[key[0] as usize] = None;
                }
                
                // 判断fullNode的子节点数量，如果只有一个，合并返回一个shoryNode
                let mut pos = 100;
                for (i,v) in f_n.borrow().children.iter().enumerate() {
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
                        if let Some(nn) = &f_n.borrow().children[pos] {
                            if nn.kind() == NodeType::ShortNode  { // 最后一个子节点是shortNode,pos拼接key后返回一个shortNode
                                let sn = nn.to_short_node();
                                let mut new_key = Vec::from([pos as u8]);
                                new_key.extend_from_slice(sn.borrow().key.as_slice());
                                return Ok((true, Rc::new(TrieNode::from_short_node(ShortNode::new(new_key, Rc::clone(&sn.borrow().val), self.new_flag())))));
                            }
                            // if nn.kind() == NodeType::ShortNode  { // 最后一个子节点是shortNode,pos拼接key后返回一个shortNode
                            //     let sn = nn.to_short_node();
                            //     let mut new_key = Vec::from([pos as u8]);
                            //     new_key.extend(sn.key);
                            //     return Ok((true, Rc::new(TrieNode::Short(ShortNode::new(new_key, sn.val, self.new_flag())))));
                            // }
                        }
                    }
                    // 不是shortNode,pos作为key,返回一个shortNode
                    if let Some(nn) = &f_n.borrow().children[pos] {
                        return Ok((true, Rc::new(TrieNode::from_short_node(ShortNode::new(Vec::from([pos as u8]), Rc::clone(nn), self.new_flag())))));
                    }
                }
                
                Ok((true, Rc::new(TrieNode::Full(f_n))))
            },
            
        }
    }

    pub fn try_get(&mut self, n: Rc<TrieNode>, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let ret = self.get(n, key_to_hex(key).as_slice(), 0)?;
        if ret.did_resolve {
            self.root = ret.new_node;
        }
        Ok(ret.value)
    }
    fn get(&self, n: Rc<TrieNode>, key: &[u8], pos: usize) -> Result<GetResult, Box<dyn Error>> {
        match &*n {
            TrieNode::Value(vn) => {
                Ok(GetResult::from(Some(vn.0.clone()), false, n))    
            },
            TrieNode::Nil => {
                // println!("null node");
                return Ok(GetResult::from(None, false, Rc::new(TrieNode::Nil)))
            },
            TrieNode::Short(sn) => {
                // let mut sn = n.to_short_node();
                if sn.borrow().key.len() > key.len()- pos ||  !sn.borrow().key.eq(&Vec::from(&key[pos..(pos + sn.borrow().key.len())])){
                    // println!("not found");
                    return Ok(GetResult::from(None, false, n));
                }
                let ret = self.get(Rc::clone(&sn.borrow().val), key, pos + sn.borrow().key.len())?;
                if ret.did_resolve {
                    // let mut nn = sn.clone();
                    // nn.val = ret.new_node;
                    // return Ok(GetResult::from(ret.value, ret.did_resolve, Rc::new(TrieNode::Short(nn))));
                    sn.borrow_mut().val = ret.new_node;
                    return Ok(GetResult::from(ret.value, ret.did_resolve, Rc::new(TrieNode::Short(Rc::clone(sn)))));
                }
                return Ok(GetResult::from(ret.value, ret.did_resolve, n));
            },
            TrieNode::Full(f_n) => {
                // let mut f_n = n.to_full_node();
                let child_node = match &f_n.borrow().children[key[pos] as usize] {
                    Some(v) => Rc::clone(v),
                    None => Rc::new(TrieNode::Nil),
                };

                let ret = self.get(child_node, key, pos+1)?;
                if ret.did_resolve {
                    let f_n = n.to_full_node();
                    f_n.borrow_mut().children[key[pos] as usize] = Some(ret.new_node);
                    return Ok(GetResult::from(ret.value, ret.did_resolve, Rc::new(TrieNode::Full(f_n))));
                }
                return Ok(GetResult::from(ret.value, ret.did_resolve, n));
            },
            TrieNode::Hash(_) => todo!(),
        }
    }

    
    // 计算默克尔hash根
    pub fn hash(&mut self) -> Hash {
        let (hs, cached) = self.hash_root();
        self.root = cached; // 计算了hash后的root重新赋值
        hs
    }
    fn hash_root(&mut self) -> (Hash, Rc<TrieNode>) {
        if self.root.kind() == NodeType::NullNode {
            return (Hash::empty_root_hash(), Rc::clone(&self.root));
        }
        println!("unhashed {}", self.unhashed);
        let mut h = Hasher::new(self.unhashed >= 100);
        let (hashed, cached) = h.hash_node(Rc::clone(&self.root), true);
        self.unhashed = 0; // 未hash的数量重置
        // 强转hashNode
        let hn = hashed.to_hash_node();
        (Hash::from(hn.0), cached)
    }
}



pub struct GetResult {
    value: Option<Vec<u8>>,
    did_resolve: bool,
    new_node: Rc<TrieNode>,
}
impl GetResult {
    pub fn from(value: Option<Vec<u8>>, did_resolve: bool, new_node: Rc<TrieNode>) -> GetResult{
        GetResult{value, did_resolve, new_node}
    }
}

pub mod node;
pub mod common;
pub mod hasher;
pub mod writer;
