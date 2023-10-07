
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
    let num = 0_u8..=133;
    let ret_size = s256.output_bytes();
    for v in num.clone() {
        s256.reset();
        s256.input(&[v]);

        
        let mut ret = Vec::from_iter(std::iter::repeat(0_u8).take(ret_size));
        s256.result(&mut ret);
        // println!("{}: {}",v, hex::encode(ret.clone()));
        t.try_update(ret.clone(), Vec::from([v])).unwrap();
        // println!("---");
        let ret = t.try_get(t.root.clone(), ret.clone()).unwrap();
        assert_ne!(ret, None);
        if let Some(val) = ret {
            assert_eq!(val, Vec::from([v]));
        }
    }
    let d = t.root.fstring("".to_string());
    s256.reset();
    s256.input_str(d.as_str());
    println!("{}", d);
    println!("hash {} len {}", s256.result_str(), d.len());

    for v in num {
        s256.reset();
        s256.input(&[v]);
        
        let ret_size = s256.output_bytes();
        let mut ret = Vec::from_iter(std::iter::repeat(0_u8).take(ret_size));
        s256.result(&mut ret);

        let ret = t.try_get(t.root.clone(), ret.clone()).unwrap();
        assert_ne!(ret, None);
        if let Some(val) = ret {
            assert_eq!(val, Vec::from([v]));
        }
    }


    

}
