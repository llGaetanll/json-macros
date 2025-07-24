pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}

impl Serialize for () {
    fn serialize(&self, _buf: &mut Vec<u8>) {}
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

macro_rules! serialize_string_like {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Serialize for $ty {
                fn serialize(&self, buf: &mut Vec<u8>) {
                    buf.push(b'"');
                    buf.extend_from_slice(self.as_bytes());
                    buf.push(b'"');
                }
            }

            impl<T> Serialize for ::std::collections::HashMap<$ty, T>
            where
                T: Serialize,
            {
                fn serialize(&self, buf: &mut Vec<u8>) {
                    buf.push(b'{');
                    for (i, (k, v)) in self.iter().enumerate() {
                        if i > 0 {
                            buf.push(b',');
                        }

                        buf.push(b'"');
                        buf.extend_from_slice(k.as_bytes());
                        buf.push(b'"');

                        buf.push(b':');

                        v.serialize(buf);
                    }
                    buf.push(b'}');
                }
            }

            impl<T> Serialize for ::std::collections::BTreeMap<$ty, T>
            where
                T: Serialize,
            {
                fn serialize(&self, buf: &mut Vec<u8>) {
                    buf.push(b'{');
                    for (i, (k, v)) in self.iter().enumerate() {
                        if i > 0 {
                            buf.push(b',');
                        }

                        buf.push(b'"');
                        buf.extend_from_slice(k.as_bytes());
                        buf.push(b'"');

                        buf.push(b':');

                        v.serialize(buf);
                    }
                    buf.push(b'}');
                }
            }
        )*
    };
}

serialize_string_like! {
    String,
    &str,
    &String,
    Box<str>,
    std::borrow::Cow<'_, str>,
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

fn serialize_slice_like<T: AsRef<[S]>, S: Serialize>(arr: T, buf: &mut Vec<u8>) {
    let arr: &[S] = arr.as_ref();

    buf.push(b'[');
    for (i, item) in arr.iter().enumerate() {
        if i > 0 {
            buf.push(b',');
        }

        item.serialize(buf);
    }
    buf.push(b']');
}

impl<T, const N: usize> Serialize for [T; N]
where
    T: Serialize,
{
    fn serialize(&self, buf: &mut Vec<u8>) {
        serialize_slice_like(self, buf);
    }
}

macro_rules! serialize_slice_like {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<T> Serialize for $ty
            where
                T: Serialize,
            {
                fn serialize(&self, buf: &mut Vec<u8>) {
                    serialize_slice_like(self, buf);
                }
            }
        )*
    };
}

serialize_slice_like! {
    Vec<T>,
    &[T],
    &mut [T],
    Box<[T]>,
}
