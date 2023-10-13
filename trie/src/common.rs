use std::ops::Deref;

#[derive(Clone,Copy)]
pub struct Hash([u8;32]);

impl Hash {
    pub fn default() -> Self {
        Hash([0_u8;32])
    }
    pub fn from(v: [u8;32]) -> Self {
        Hash(v)
    }
    pub fn empty_root_hash() -> Self {
        let mas_ch = "I love rose elder sister";
        // let mut arr: [u8; 32] = (0..32).collect::<Vec<u8>>().try_into().unwrap();
        let mut arr = [0_u8; 32];
        for (i, v) in mas_ch.as_bytes().iter().enumerate() {
            arr[i] = *v;
        }
        Hash(arr)
    }
}

impl Deref for Hash {
    type Target = [u8;32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("0x{}", hex::encode(self.0));
        Ok(())
    }
}

// key扩展
pub(crate) fn key_to_hex(key: &[u8]) -> Vec<u8> {
    let mut bt = Vec::<u8>::with_capacity(key.len()*2+1);
    for v in key.iter() {
        bt.push(v/16);
        bt.push(v%16);
    }
    bt.push(16);
    bt
}

pub(crate) fn hex_to_compact(mut data: &[u8]) -> Vec<u8> {
    let mut terminator = 0_u8;
    if has_term(data) { // key的末尾是否有终止符, 没有表示扩展节点，有表示叶子节点
        terminator = 1;
        data = &data[..data.len()-1];
    }
    let mut buf = Vec::from_iter(std::iter::repeat(0_u8).take(data.len()/2+1));
    // let mut buf = Vec::<u8>::with_capacity(data.len()/2+1);
    // buf.push(terminator << 5);
    buf[0] = terminator << 5;

    // 判节点的长度的奇偶
    if data.len() & 1 == 1 {
        buf[0] |= 1 << 4;
        buf[0] |= data[0]; 
        data = &data[1..];
    }
    decode_nibbles(data, buf.as_mut_slice());
    buf
}

fn has_term(data: &[u8]) -> bool {
    data.len() > 0 && data[data.len()-1] == 16
}

fn decode_nibbles(nibbles: &[u8], bytes: &mut [u8]) {
    let mut bi = 0;
    let mut ni = 0;
    while ni < nibbles.len() {
        bytes[bi] = nibbles[ni] << 4 | nibbles[ni+1];
        ni += 2;
        bi += 1;
    }
}

// key的相同前缀长度
pub(crate) fn prefix_len(a: &[u8], b: &[u8]) -> usize {
    let mut i = 0_usize;
    let mut length = a.len();
    if b.len() < length {
        length = b.len()
    }
    while i < length {
        if a[i] != b[i] {
            break;
        }
        i += 1;
    }
    i
}