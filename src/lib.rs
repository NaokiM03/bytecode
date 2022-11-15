use std::fmt::Debug;

use tiny_ansi::TinyAnsi;

pub struct ByteCode {
    bytes: Vec<u8>,
    pos: usize,
}

impl Debug for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        let header = "         00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".cyan();
        result.push_str(&header);
        result.push('\n');

        let mut bytes: Vec<String> = self
            .bytes
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect();
        if let Some(byte) = bytes.get_mut(self.pos) {
            *byte = byte.green();
        }
        let lines = bytes.chunks(16).map(|line| line.join(" "));
        for (i, line) in lines.enumerate() {
            let line_number = {
                let mut line_number = "0".repeat(8);
                let hex = format!("{:02X}", i);
                line_number.replace_range((8 - hex.len() - 1)..7, &hex);
                line_number
            };
            result.push_str(&line_number);
            result.push(' ');
            result.push_str(&line);
            result.push('\n');
        }
        write!(f, "\n{}", result)
    }
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
