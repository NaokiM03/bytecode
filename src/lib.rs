#[derive(Debug)]
pub struct ByteCode {
    bytes: Vec<u8>,
    pos: usize,
}

impl From<Vec<u8>> for ByteCode {
    fn from(v: Vec<u8>) -> Self {
        ByteCode {
            bytes: v.to_vec(),
            pos: 0,
        }
    }
}

impl ByteCode {
    pub fn peek(&self, num: usize) -> Vec<u8> {
        self.bytes[self.pos..][0..num].to_owned()
    }

    pub fn start_with(&self, v: &[u8]) -> bool {
        self.peek(v.len()) == v
    }

    pub fn next(&mut self) {
        self.pos += 1;
    }

    pub fn skip(&mut self, num: usize) {
        self.pos += num;
    }

    pub fn take(&mut self, num: usize) -> Vec<u8> {
        let result = self.peek(num);
        self.skip(num);
        result
    }

    pub fn take_into_u16(&mut self) -> u16 {
        let bytes: [u8; 2] = self.take(2).try_into().unwrap();
        u16::from_be_bytes(bytes)
    }

    pub fn take_into_u32(&mut self) -> u32 {
        let bytes: [u8; 4] = self.take(4).try_into().unwrap();
        u32::from_be_bytes(bytes)
    }
}

#[test]
fn peek() {
    let bytes: ByteCode = vec![0, 1, 2, 3, 4, 5, 6, 7].into();
    assert_eq!(bytes.peek(3), [0, 1, 2]);
}

#[test]
fn start_with() {
    let bytes: ByteCode = vec![0, 1, 2, 3, 4, 5, 6, 7].into();
    assert!(bytes.start_with(&[0, 1, 2]));

    let bytes: ByteCode = vec![0x66, 0x6f, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00].into();
    assert!(bytes.start_with("foo".as_bytes()));
}

#[test]
fn next() {
    let mut bytes: ByteCode = vec![0, 1, 2, 3, 4, 5, 6, 7].into();
    assert_eq!(bytes.peek(3), [0, 1, 2]);
    bytes.next();
    assert_eq!(bytes.peek(3), [1, 2, 3]);
}

#[test]
fn skip() {
    let mut bytes: ByteCode = vec![0, 1, 2, 3, 4, 5, 6, 7].into();
    assert_eq!(bytes.peek(3), [0, 1, 2]);
    bytes.skip(3);
    assert_eq!(bytes.peek(3), [3, 4, 5]);
}

#[test]
fn take() {
    let mut bytes: ByteCode = vec![0, 1, 2, 3, 4, 5, 6, 7].into();
    assert_eq!(bytes.take(3), [0, 1, 2]);
    assert_eq!(bytes.peek(3), [3, 4, 5]);
}

#[test]
fn take_into_u16() {
    let mut bytes: ByteCode = vec![0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].into();
    assert_eq!(bytes.take_into_u16(), 65535);
    assert_eq!(bytes.peek(3), [0, 0, 0]);
}

#[test]
fn take_into_u32() {
    let mut bytes: ByteCode = vec![0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00].into();
    assert_eq!(bytes.take_into_u32(), 4294967295);
    assert_eq!(bytes.peek(3), [0, 0, 0]);
}
