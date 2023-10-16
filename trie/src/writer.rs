
pub struct EncodeBuffer {
    data_buf: Vec<u8>
}

impl EncodeBuffer {
    pub fn new() -> Self {
        EncodeBuffer { data_buf: Vec::new() }
    }
}

impl EncodeBuffer {
    pub fn write_bytes(&mut self, buf: &[u8]) {
        self.data_buf.extend_from_slice(buf);
    }
    pub fn write(&mut self, b: u8) {
        self.data_buf.push(b);
    }
    pub fn size(&mut self) -> usize {
        self.data_buf.len()
    }
    pub fn encode_bytes(&self) -> Vec<u8> {
        // let mut dst = Vec::from_iter(std::iter::repeat(0_u8).take(self.data_buf.len()));
        // dst.extend_from_slice(self.data_buf.as_slice());
        // dst
        self.data_buf.clone()
    }
    pub fn reset(&mut self) {
        self.data_buf = Vec::new();
    }
}

// impl Write for EncodeBuffer {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.data_buf.extend_from_slice(buf);
//         Ok(buf.len())
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         todo!()
//     }
// }