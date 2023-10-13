
use std::{rc::Rc, os, fs, ops::Add, cell::Cell, fmt};

use crypto::{sha2::{Sha256, Sha224}, digest::Digest};
use trie::{node::{hash_node, Node, NilNode}, ID, common::{Hash, self}, Trie};



// fn displayArr<T:std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }

// fn display_arr2<T:std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }


// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, rhs: Meters) -> Self::Output {
//         todo!()
//     }
// }

// struct Get  {
//     root: Rc<dyn Node>
// }
// impl Get {
//     fn prefix(&self) -> Rc<dyn Node> {
//         Rc::clone(&self.root)
//     }
// }


fn main() {
//     let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

//     let s1 = s.clone();
//     let s2 = s.clone();
//     // let mut s2 = s.borrow_mut();
//     s2.borrow_mut().push_str(", oh yeah!");
    let mut t: Trie = Trie::new(ID::trie_id(Hash::default()));

    println!("{}", t.hash());
    // return;    
    let mut s256 = Sha256::new();
    let num = 0_u64..=1000;
    let ret_size = s256.output_bytes();
    for v in num.clone() {
        s256.reset();
        let vs = v.to_le_bytes();
        s256.input(&vs);
        
        let mut ret = Vec::from_iter(std::iter::repeat(0_u8).take(ret_size));
        s256.result(&mut ret);
        // println!("{}: {}",v, hex::encode(ret.clone()));
        t.try_update(ret.clone(), Some(vs.to_vec())).unwrap();
        let ret = t.try_get(t.root.clone(), ret.clone()).unwrap();
        assert_ne!(ret, None);
        if let Some(val) = ret {
            assert_eq!(val, vs.to_vec());
        }
    }

    for v in num.clone() {
        
        s256.reset();
        let vs = v.to_le_bytes();
        s256.input(&vs);
        
        let mut ret = Vec::from_iter(std::iter::repeat(0_u8).take(ret_size));
        s256.result(&mut ret);

        if v & 1 == 0 {
            t.try_update(ret.clone(), None).unwrap();
        }
        // print!("get {} ", v);
        let ret = t.try_get(t.root.clone(), ret.clone()).unwrap();
        if v & 1 == 0 {
            assert_eq!(ret, None);
            // println!("{} Null",v);
        } else {
            if let Some(val) = ret {
                assert_eq!(val, vs.to_vec());
                // println!("{} {:?}", v, val);
            }
        }
    }

    let d = t.root.fstring("".to_string());
    s256.reset();
    s256.input_str(d.as_str());
    // println!("{}", d);
    println!("hash {} len {}", s256.result_str(), d.len());

}
