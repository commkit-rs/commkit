pub enum Direction {
    None,
    Tx,
    Rx,
}

pub trait ByteTransfer {
    fn as_bytes(&self) -> &[u8];

    fn len(&self) -> usize {
        self.as_bytes().len()
    }

    fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }
}