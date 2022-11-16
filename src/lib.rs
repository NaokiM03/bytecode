use std::{
    fmt::Debug,
    ops::{AddAssign, Index, SubAssign},
    slice::SliceIndex,
};

use tiny_ansi::TinyAnsi;

pub struct ByteCode<'a> {
    inner: &'a [u8],
    pos: usize,
}

impl Debug for ByteCode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        let header = "         00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".cyan();
        result.push_str(&header);
        result.push('\n');

        let current_pos = self.pos;
        let mut replica = ByteCode {
            inner: self.inner,
            pos: self.pos,
        };
        replica.reset();

        // If the amount of data is large, it may be better to use a lookup table.
        let mut content: Vec<String> = replica
            .inner
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect();
        if let Some(byte) = content.get_mut(current_pos) {
            *byte = byte.green();
        }
        for (i, line) in content.chunks(16).map(|line| line.join(" ")).enumerate() {
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

impl<'a> ByteCode<'a> {
    /// Creates a new `ByteCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// ```
    pub fn new(slice: &'a [u8]) -> Self {
        ByteCode {
            inner: slice,
            pos: 0,
        }
    }

    /// Extracts a current remaining slice.
    ///
    /// Equivalent to `&bytes[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// let slice = bytes.as_slice();
    /// ```
    pub fn as_slice(&self) -> &'a [u8] {
        self.inner
    }

    /// Returns the number of elements.
    ///
    /// Note that consumed elements are also counted.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(bytes.len(), 8);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len() + self.pos
    }

    /// Resets the pointer to original state.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
    /// let mut bytes = ByteCode::new(&v);
    /// bytes += 5;
    /// assert_eq!(bytes.as_slice(), [5, 6, 7]);
    /// bytes.reset();
    /// assert_eq!(bytes.as_slice(), v);
    /// ```
    pub fn reset(&mut self) {
        *self -= self.pos;
    }

    /// Returns `true` if all elements have been consumed.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    ///
    /// bytes += 8;
    /// assert!(bytes.is_end());
    ///
    /// bytes -= 6;
    /// assert!(!bytes.is_end());
    /// ```
    pub fn is_end(&self) -> bool {
        self.pos == self.len()
    }
}

impl<'a> AddAssign<usize> for ByteCode<'a> {
    /// Move the pointer to the next.
    fn add_assign(&mut self, rhs: usize) {
        if rhs > self.inner.len() {
            panic!(
                "index out of bounds: the slice can only move forward {}, but tried to move {}",
                self.inner.len(),
                rhs
            );
        }
        self.inner = unsafe {
            let ptr = self.inner.as_ptr().add(rhs);
            std::slice::from_raw_parts(ptr, self.inner.len() - rhs)
        };
        self.pos += rhs;
    }
}

impl<'a> SubAssign<usize> for ByteCode<'a> {
    /// Move the pointer to the prev.
    fn sub_assign(&mut self, rhs: usize) {
        if rhs > self.pos {
            panic!(
                "index out of bounds: the slice can only move back {}, but tried to move {}",
                self.pos, rhs
            );
        }

        self.inner = unsafe {
            let ptr = self.inner.as_ptr().sub(rhs);
            std::slice::from_raw_parts(ptr, self.inner.len() + rhs)
        };
        self.pos -= rhs;
    }
}

impl<'a, I: SliceIndex<[u8]>> Index<I> for ByteCode<'a> {
    type Output = I::Output;
    fn index(&self, i: I) -> &Self::Output {
        Index::index(self.inner, i)
    }
}

impl<'a> ByteCode<'a> {
    /// Returns a reference to subslice corresponding to the given size.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(bytes.peek(3), [0, 1, 2]);
    /// ```
    pub fn peek(&'a self, num: usize) -> &'a [u8] {
        if num > self.inner.len() {
            panic!(
                "range end index {} out of range for slice of length {}",
                num,
                self.inner.len()
            );
        }
        &self[0..num]
    }

    /// Returns `true` if given subslice is a prefix of the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// assert!(bytes.starts_with(&[0, 1, 2]));
    /// ```
    pub fn starts_with(&self, v: &[u8]) -> bool {
        self.peek(v.len()) == v
    }

    /// Move the pointer to the next.
    /// Note that nothing is returned.
    ///
    /// Equivalent to `bytes += 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// bytes += 3;
    /// ```
    pub fn next(&mut self) {
        *self += 1;
    }

    /// Move the pointer to the prev.
    /// Note that nothing is returned.
    ///
    /// Equivalent to `bytes -= 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// bytes += 5;
    /// bytes -= 3;
    /// ```
    pub fn prev(&mut self) {
        *self -= 1;
    }

    /// Move the pointer forward by the given number.
    ///
    /// Equivalent to `bytes += num`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// bytes.skip(3);
    /// ```
    pub fn skip(&mut self, num: usize) {
        *self += num;
    }

    /// Returns a vector containing a copy of subslice corresponding to the given size.
    /// Moves the pointer forward by the length of subslice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(bytes.take(3), [0, 1, 2]);
    /// ```
    pub fn take(&mut self, num: usize) -> Vec<u8> {
        let result = self.peek(num).to_owned();
        self.skip(num);
        result
    }

    /// Returns the first 2 elements of the slice converted into `u16`.
    /// Moves the pointer forward 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// assert_eq!(bytes.take_into_u16(), u16::MAX);
    /// ```
    pub fn take_into_u16(&mut self) -> u16 {
        let bytes: [u8; 2] = self.take(2).try_into().unwrap();
        u16::from_be_bytes(bytes)
    }

    /// Returns the first 4 elements of the slice converted into `u32`.
    /// Moves the pointer forward 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode_rs::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00]);
    /// assert_eq!(bytes.take_into_u32(), u32::MAX);
    /// ```
    pub fn take_into_u32(&mut self) -> u32 {
        let bytes: [u8; 4] = self.take(4).try_into().unwrap();
        u32::from_be_bytes(bytes)
    }
}

#[test]
fn new() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let bytes = ByteCode::new(&v);
    assert_eq!(bytes.inner, v);
    assert_eq!(bytes.pos, 0);
}

#[test]
fn as_slice() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let bytes = ByteCode::new(&v);
    assert_eq!(bytes.as_slice(), v);
}

#[test]
fn len() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    bytes.skip(3);
    assert_eq!(bytes.inner.len(), 5); // `inner` is not public
    assert_eq!(bytes.len(), 8);
}

#[test]
fn reset() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut bytes = ByteCode::new(&v);
    bytes.skip(5);
    assert_eq!(bytes.inner, [5, 6, 7]);
    assert_eq!(bytes.pos, 5);
    bytes.reset();
    assert_eq!(bytes.inner, v);
    assert_eq!(bytes.pos, 0);
}

#[test]
fn is_end() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    bytes.skip(8);
    assert!(bytes.is_end());
}

#[test]
fn add_assign() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);

    bytes += 1;
    assert_eq!(bytes.inner, [1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(bytes.pos, 1);

    bytes += 3;
    assert_eq!(bytes.inner, [4, 5, 6, 7]);
    assert_eq!(bytes.pos, 4);
}

#[test]
fn sub_assign() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);

    bytes += 8;
    assert_eq!(bytes.inner, []);
    assert_eq!(bytes.pos, 8);

    bytes -= 1;
    assert_eq!(bytes.inner, [7]);
    assert_eq!(bytes.pos, 7);

    bytes -= 3;
    assert_eq!(bytes.inner, [4, 5, 6, 7]);
    assert_eq!(bytes.pos, 4);
}

#[test]
fn index() {
    let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);

    assert_eq!(bytes[4], 4);
    assert_eq!(bytes[2..], [2, 3, 4, 5, 6, 7]);
    assert_eq!(bytes[..6], [0, 1, 2, 3, 4, 5]);
    assert_eq!(bytes[2..6], [2, 3, 4, 5]);
    assert_eq!(bytes[2..=6], [2, 3, 4, 5, 6]);
}

#[test]
fn peek() {
    let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(bytes.peek(3), [0, 1, 2]);
}

#[test]
#[should_panic]
fn peek_out_of_range() {
    let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    bytes.peek(9);
}

#[test]
fn starts_with() {
    let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    assert!(bytes.starts_with(&[0, 1, 2]));

    let bytes = ByteCode::new(&[0x66, 0x6f, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert!(bytes.starts_with("foo".as_bytes()));
}

#[test]
fn next() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(bytes.peek(3), [0, 1, 2]);
    bytes.next();
    assert_eq!(bytes.peek(3), [1, 2, 3]);
}

#[test]
fn prev() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    bytes.skip(4);
    assert_eq!(bytes.peek(3), [4, 5, 6]);
    bytes.prev();
    assert_eq!(bytes.peek(3), [3, 4, 5]);
}

#[test]
fn skip() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(bytes.peek(3), [0, 1, 2]);
    bytes.skip(3);
    assert_eq!(bytes.peek(3), [3, 4, 5]);
}

#[test]
fn take() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(bytes.take(3), [0, 1, 2]);
    assert_eq!(bytes.peek(3), [3, 4, 5]);
}

#[test]
fn take_into_u16() {
    let mut bytes = ByteCode::new(&[0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(bytes.take_into_u16(), u16::MAX);
    assert_eq!(bytes.peek(3), [0, 0, 0]);
}

#[test]
fn take_into_u32() {
    let mut bytes = ByteCode::new(&[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(bytes.take_into_u32(), u32::MAX);
    assert_eq!(bytes.peek(3), [0, 0, 0]);
}
