pub struct BitIter<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> BitIter<'a> {
    const BYTE_SIZE: usize = size_of::<u8>() * 8;

    pub fn new(bytes: &'a [u8]) -> Self {
        BitIter { bytes, index: 0 }
    }
}

impl<'a> Iterator for BitIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_index = self.index / Self::BYTE_SIZE;
        let byte = *self.bytes.get(byte_index)?;

        let bit_index = 7 - (self.index % 8);
        let bit = (byte >> bit_index) & 1;

        self.index += 1;
        Some(bit)
    }
}
