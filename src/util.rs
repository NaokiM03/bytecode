use crate::ByteCode;

impl<'a> ByteCode<'a> {
    /// Returns a reference to subslice corresponding to the given size.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
    /// use bytecode::ByteCode;
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
