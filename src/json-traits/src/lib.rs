pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}

macro_rules! serialize_int {
    ($($type:ty),* $(,)?) => {
        $(
            impl Serialize for $type {
                fn serialize(&self, buf: &mut Vec<u8>) {
                    let mut n_buf = itoa::Buffer::new();
                    let n_str = n_buf.format(*self);
                    buf.extend_from_slice(n_str.as_bytes());
                }
            }
        )*
    };
}

macro_rules! serialize_float {
    ($($type:ty),* $(,)?) => {
        $(
            impl Serialize for $type {
                fn serialize(&self, buf: &mut Vec<u8>) {
                    let mut n_buf = ryu::Buffer::new();
                    let n_str = n_buf.format(*self);
                    buf.extend_from_slice(n_str.as_bytes());
                }
            }
        )*
    };
}

#[rustfmt::skip]
serialize_int!(
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128
);

#[rustfmt::skip]
serialize_float!(
    f32, f64
);

impl Serialize for bool {
    fn serialize(&self, buf: &mut Vec<u8>) {
        if *self {
            buf.extend_from_slice(b"true");
        } else {
            buf.extend_from_slice(b"false");
        }
    }
}

impl Serialize for &'_ str {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.push(b'"');
        buf.extend_from_slice(self.as_bytes());
        buf.push(b'"');
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize(&self, buf: &mut Vec<u8>) {
        match self {
            Some(t) => t.serialize(buf),
            None => buf.extend_from_slice(b"null"),
        }
    }
}
