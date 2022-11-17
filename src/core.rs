use std::{
    fmt::Debug,
    ops::{AddAssign, Index, SubAssign},
    slice::SliceIndex,
};

use tiny_ansi::TinyAnsi;

pub struct ByteCode<'a> {
    pub(crate) inner: &'a [u8],
    pub(crate) pos: usize,
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
    ///
    /// let bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(bytes.len(), 8);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len() + self.pos
    }

    /// Returns the pointer position.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode::ByteCode;
    ///
    /// let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    /// bytes += 5;
    /// assert_eq!(bytes.pos(), 5);
    /// ```
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Resets the pointer to original state.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
fn pos() {
    let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    bytes.skip(5);
    assert_eq!(bytes.pos(), 5);
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
