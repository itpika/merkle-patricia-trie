

#[derive(Clone,Debug,Copy)]
pub struct Hash([u8;32]);

impl Hash {
    pub fn default() -> Self {
        Hash([0_u8;32])
    }
}

