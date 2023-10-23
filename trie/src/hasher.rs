use std::{rc::Rc, cell::RefCell, io::Write, future, process::Output};

use crypto::{sha2::{Sha256}, digest::Digest, hmac, sha3::Sha3};
use futures::{Future, executor::block_on};

use crate::{node::{HashNode, NodeType, ShortNode, FullNode, ValueNode}, common, writer::{EncodeBuffer}, TrieNode};

pub(crate) struct Hasher {
    hash: Sha3,
    parallel: bool,
    out_hash_size: usize,
    w: Rc<RefCell<EncodeBuffer>>,
}

impl Hasher {
    pub(crate) fn new(parallel: bool) -> Hasher {
        // let mut s256 = Sha256::new();
        let mut s256 = Sha3::keccak256();
        Hasher { hash: s256, parallel, out_hash_size: s256.output_bytes(), w: Rc::new(RefCell::new(EncodeBuffer::new())) }
    }

    pub(crate) fn hash_data(&mut self, data: &[u8]) -> HashNode {
        self.hash.reset();
        self.hash.input(data);
        // let mut out = Vec::from_iter(std::iter::repeat(0_u8).take(self.out_hash_size));
        let mut out = [0_u8;32];
        self.hash.result(&mut out);
        HashNode::from(out)
    }

    pub(crate) fn hash_node(&mut self, n: Rc<TrieNode>, force: bool) -> (Rc<TrieNode>, Rc<TrieNode>) {
        let (hs, _) = n.cache(); // 看缓存是否已经计算过
        if let Some(v) = hs {
            return (Rc::new(TrieNode::Hash(v)), n);
        }
        match &*n {
            TrieNode::Short(sn) => {
                let mut sn = n.to_short_node();
                // shortNode子节点hash
                let (collapsed, mut cached_node) = self.hash_short_node_children(sn);
                // shortNode计算hash
                let hashed = self.shortnode_to_hash(collapsed, force);

                if hashed.kind() == NodeType::HashNode {
                    // hash缓存起来
                    cached_node.flags.hash = Some(hashed.to_hash_node());
                } else {
                    cached_node.flags.hash = None;
                }

                return (hashed, Rc::new(TrieNode::Short(cached_node)));
            },
            TrieNode::Full(f_n) => {
                let   = n.to_full_node();
                // 计算子节点hash
                let (collapsed, mut cached_node) = self.hash_full_node_children(f_n);
                // 计算fullNode自己的hash
                let hashed = self.fullnode_to_hash(collapsed, force);

                if hashed.kind() == NodeType::HashNode {
                    // hash缓存起来
                    cached_node.flags.hash = Some(hashed.to_hash_node());
                } else {
                    cached_node.flags.hash = None
                }
                return (hashed, Rc::new(TrieNode::Full(cached_node)));
            },
            _ => { // 正常情况不会到此
                return (Rc::clone(&n), n);
            }
        }
    }

    // 计算shortNode子节点hash
    fn hash_short_node_children(&mut self, mut n: ShortNode) -> (ShortNode, ShortNode) {
        let mut collapsed = n.into_short_node();
        // let mut cached = n.into_short_node().unwrap();

        collapsed.key = common::hex_to_compact(&n.key);

        if n.val.kind() == NodeType::FullNode || n.val.kind() == NodeType::ShortNode {
            (collapsed.val, n.val) = self.hash_node(n.val, false);
        }
        (collapsed, n)
    }
    // 计算shortNode节点hash
    fn shortnode_to_hash(&mut self, n: ShortNode, force: bool) -> Rc<TrieNode> {
        // node编码进bufer
        n.encode(Rc::clone(&self.w));
        let enc = self.encod_bytes();
        if enc.len() < 32 && !force {
            return Rc::new(TrieNode::Short(n));
        }
        // 编码后的数据计算hash
        let hd = self.hash_data(enc.as_slice());
        Rc::new(TrieNode::Hash(hd))
    }

    // async fn async_hash_full_node_children(n: &Option<Rc<dyn Node>>, collapsed: Rc<RefCell<FullNode>>, cached: Rc<RefCell<FullNode>>, i: usize) {
    //     let mut new_hasher = Hasher::new(false);
    //     match n {
    //         Some(child_node) => {
    //             let (n1, n2) = new_hasher.hash_node(Rc::clone(child_node), false);
    //             collapsed.borrow_mut().children[i] = Some(n1);
    //             cached.borrow_mut().children[i] = Some(n2);
    //         },
    //         None => { // 计算hash赋个空valueNode
    //             collapsed.borrow_mut().children[i] = Some(Rc::new(ValueNode::default()));
    //         }
    //     }
    // }

    fn hash_full_node_children(&mut self, mut n: FullNode) -> (FullNode, FullNode) {
        let mut collapsed = n.into_full_node();
        // let mut cached = n.into_full_node().unwrap();
        
        if self.parallel && false {
            // let collapsed = Rc::new(RefCell::new(collapsed));
            // let cached = Rc::new(RefCell::new(cached));

            // // let arr = [impl Future<Output = ()>;16];
            // let mut arr = Vec::with_capacity(16);
            // for (i, _) in [0u8; 16].iter().enumerate() {
            //     let g = Hasher::async_hash_full_node_children(&n.children[i], 
            //         Rc::clone(&collapsed), Rc::clone(&cached), i);
            //     arr.push(g);
            // }
            // futures::future::join_all(arr);
            // let g = collapsed.borrow();
            // // let f = *g;
            // return (*g, *cached.borrow());
        } else {
            for (i, _) in [0u8; 16].iter().enumerate() {
                match &n.children[i] {
                    Some(child_node) => {
                        let (n1, n2) = self.hash_node(Rc::clone(child_node), false);
                        collapsed.children[i] = Some(n1);
                        n.children[i] = Some(n2);
                    },
                    None => { // 计算hash赋个空valueNode
                        collapsed.children[i] = Some(Rc::new(TrieNode::Value(ValueNode::default())));
                    }
                }
            }
        }
        (collapsed, n)
    }

    // 计算fullNode节点hash
    fn fullnode_to_hash(&mut self, n: FullNode, force: bool) -> Rc<TrieNode> {
        // node编码进bufer
        n.encode(Rc::clone(&self.w));
        let enc = self.encod_bytes();
        if enc.len() < 32 && !force {
            return Rc::new(TrieNode::Full(n));
        }
        // 编码后的数据计算hash
        let hd = self.hash_data(enc.as_slice());
        Rc::new(TrieNode::Hash(hd))
    }

    // 取出buffer中的所有数据
    fn encod_bytes(&self) -> Vec<u8> {
        let ret = self.w.borrow().encode_bytes();
        self.w.borrow_mut().reset();
        ret
    }
}