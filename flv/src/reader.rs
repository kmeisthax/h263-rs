/// A reader that allows demuxing an FLV container.
pub struct FlvReader<'a> {
    source: &'a [u8],

    position: usize,
}

impl<'a> FlvReader<'a> {
    pub fn from_source(source: &'a [u8]) -> Self {
        FlvReader {
            source,
            position: 0,
        }
    }

    /// Read a certain number of bytes from the buffer.
    ///
    /// This works like `Read`, but returns borrowed slices of the source
    /// buffer. The buffer position will be advanced so that repeated reads
    /// yield new data.
    ///
    /// If the requested number of bytes are not available, `None` is returned.
    pub fn read(&mut self, count: usize) -> Option<&'a [u8]> {
        let start = self.position;
        let end = self.position.checked_add(count)?;
        if end > self.source.len() {
            return None;
        }

        self.position = end;

        Some(&self.source[start..end])
    }

    /// Read a certain number of bytes from the buffer without advancing the
    /// buffer position.
    pub fn peek(&mut self, count: usize) -> Option<&'a [u8]> {
        let pos = self.position;
        let ret = self.read(count);

        self.position = pos;

        ret
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        Some(self.read(1)?[0])
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        Some(u16::from_be_bytes(
            self.read(2)?.try_into().expect("two bytes"),
        ))
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        Some(i16::from_be_bytes(
            self.read(2)?.try_into().expect("two bytes"),
        ))
    }

    pub fn read_u24(&mut self) -> Option<u32> {
        let bytes = self.read(3)?;

        Some(u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]))
    }

    pub fn peek_u24(&mut self) -> Option<u32> {
        let bytes = self.peek(3)?;

        Some(u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        Some(u32::from_be_bytes(
            self.read(4)?.try_into().expect("four bytes"),
        ))
    }

    pub fn read_f64(&mut self) -> Option<f64> {
        Some(f64::from_be_bytes(
            self.read(8)?.try_into().expect("eight bytes"),
        ))
    }

    pub fn position(&self) -> usize {
        self.position
    }
}
