
use std::{rc::Rc, os, fs};

use crypto::{sha2::{Sha256, Sha224}, digest::Digest};
use trie::{node::{hash_node, Node, NilNode}, ID, common::Hash, Trie};



// fn displayArr<T:std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }

// fn display_arr2<T:std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }

fn main() {
    
    let mut t: Trie = Trie::new(ID::trie_id(Hash::default()));
    
    let mut s256 = Sha256::new();
    for v in 0_u8..=132 {
        s256.reset();
        s256.input(&[v]);

        
        let ret_size = s256.output_bytes();
        let mut ret = Vec::from_iter(std::iter::repeat(0_u8).take(ret_size));
        s256.result(&mut ret);
        // println!("{}: {}",v, hex::encode(ret.clone()));
        // s256.result_str()
        // let value = Vec::from([v]);
        t.update(ret.clone(), Vec::from([v])).unwrap();
        // println!("---");
    }
    println!("{}", t.root.fstring("".to_string()));
    let d = t.root.fstring("".to_string());
    s256.reset();
    s256.input_str(d.as_str());
    println!("hash {} len {}", s256.result_str(), d.len());

}
