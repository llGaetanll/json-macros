pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}
