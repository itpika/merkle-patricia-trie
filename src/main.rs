
use std::{rc::Rc, cell::{Cell, RefCell}, fmt, future, time::{SystemTime, self}};

use crypto::{sha2::{Sha256, Sha224}, digest::Digest, sha3::Sha3};
use tokio::time::sleep;
use trie::{node::{hash_node, Node, NilNode}, ID, common::{Hash, self}, Trie};



// fn displayArr<T:std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }

// fn display_arr2<T:std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }

async fn eat() {
    println!("eat");
    let v = song().await;
    println!("{}", v);
}

async fn song() -> String {
    println!("song");
    // sleep(std::time::Duration::from_secs(3))
    sleep(std::time::Duration::from_secs(3)).await;
    "song over".to_string()
}

async fn lon() {
    println!("lon");
}

async fn do_work() {
    let w1 = eat();
    let w3 = lon();
    futures::join!(w1, w3);
}

use futures::{executor::block_on};

#[tokio::main]
async fn main2() {
    block_on(do_work());
}

fn main() {
    let mut t: Trie = Trie::new(ID::trie_id(Hash::default()));

    // return;    
    let mut s256 = Sha256::new();
    let num = 0_u64..=130000;
    let ret_size = s256.output_bytes();
    for v in num.clone() {
        s256.reset();
        let vs = v.to_le_bytes();
        s256.input(&vs);
        
        let mut ret = Vec::from_iter(std::iter::repeat(0_u8).take(ret_size));
        s256.result(&mut ret);
        // println!("{}: {}",v, hex::encode(ret.clone()));
        t.try_update(ret.clone(), Some(vs.to_vec())).unwrap();
        let ret = t.try_get(t.root.clone(), ret.as_slice()).unwrap();
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
        let ret = t.try_get(t.root.clone(), ret.as_slice()).unwrap();
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
    let st = now();
    println!("root hash {}", t.hash());
    println!("{:?}", now()-st);

}

fn now() -> time::Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
}