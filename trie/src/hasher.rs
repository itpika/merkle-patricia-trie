use std::{rc::Rc, cell::RefCell, io::Write};

use crypto::{sha2::{Sha256}, digest::Digest};

use crate::{node::{HashNode, Node, NodeType, ShortNode}, common, writer::{EncodeBuffer}};

pub(crate) struct Hasher {
    hash: Sha256,
    parallel: bool,
    out_hash_size: usize,
    w: Rc<RefCell<EncodeBuffer>>,
}

impl Hasher {
    pub(crate) fn new(parallel: bool) -> Hasher {
        let mut s256 = Sha256::new();
        Hasher { hash: s256, parallel, out_hash_size: s256.output_bytes(), w: Rc::new(RefCell::new(EncodeBuffer::new())) }
    }

    pub(crate) fn hash_data(&mut self, data: &[u8]) -> HashNode {
        self.hash.input(data);
        // let mut out = Vec::from_iter(std::iter::repeat(0_u8).take(self.out_hash_size));
        let mut out = [0_u8;32];
        self.hash.result(&mut out);
        HashNode::from(out)
    }

    pub(crate) fn hash_node(&mut self, n: Rc<dyn Node>, force: bool) -> (Rc<dyn Node>, Rc<dyn Node>) {
        let (hs, _) = n.cache(); // 看缓存是否已经计算过
        if let Some(v) = hs {
            return (Rc::new(v), n);
        }
        match n.kind() {
            NodeType::ShortNode => {
                let sn = n.into_short_node().unwrap();
                // shortNode子节点hash
                let (collapsed, mut cached_node) = self.hash_short_node_children(sn);
                // shortNode计算hash
                let hashed = self.shortnode_to_hash(collapsed, force);

                if hashed.kind() == NodeType::HashNode {
                    // hash缓存起来
                    cached_node.flags.hash = Some(hashed.into_hash_node().unwrap());
                } else {
                    cached_node.flags.hash = None
                }

                return (hashed, Rc::new(cached_node));
            },
            NodeType::FullNode => {

            },
            _ => { // 正常情况不会到此
                return (Rc::clone(&n), n);
            }
        }
        todo!()
    }
    // 计算shortNode子节点hash
    fn hash_short_node_children(&mut self, n: ShortNode) -> (ShortNode, ShortNode) {
        let mut collapsed = n.into_short_node().unwrap();
        let mut cached = n.into_short_node().unwrap();

        collapsed.key = common::hex_to_compact(&n.key);

        if n.val.kind() == NodeType::FullNode || n.val.kind() == NodeType::ShortNode {
            (collapsed.val, cached.val) = self.hash_node(n.val, false);
        }
        (collapsed, cached)
    }
    // 计算shortNode节点hash
    fn shortnode_to_hash(&mut self, n: ShortNode, force: bool) -> Rc<dyn Node> {
        // node编码进bufer
        n.encode(Rc::clone(&self.w));
        let enc = self.encod_bytes();
        if enc.len() < 32 && !force {
            return Rc::new(n);
        }
        // 编码后的数据计算hash
        let hd = self.hash_data(enc.as_slice());
        Rc::new(hd)
    }

    // 取出buffer中的所有数据
    fn encod_bytes(&self) -> Vec<u8> {
        self.w.borrow().encode_bytes()
    }
}