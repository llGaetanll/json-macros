use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;

/// A trait that allows us to guard what types of values we can serialize in key position.
///
/// Put simply: `vec![1, 2, 3]` makes no sense as a key, but `"foo"` or `String::from("foo")` do.
/// This is a restricted version of `Serialize` that prevents us from using any type as a JSON key.
pub trait SerializeStr<W: Write>: private::Sealed {
    fn serialize_str(&self, buf: &mut W) -> std::io::Result<()>;
}

mod private {
    pub trait Sealed {}
}

macro_rules! serialize_str {
    ($($type:ty),* $(,)?) => {
        $(
            impl private::Sealed for $type { }

            impl<W: Write> SerializeStr<W> for $type {
                fn serialize_str(&self, buf: &mut W) -> std::io::Result<()> {
                    buf.write_all(b"\"")?;
                    buf.write_all(self.as_bytes())?;
                    buf.write_all(b"\"")?;

                    Ok(())
                }
            }

            impl<W: Write> Serialize<W> for $type {
                fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
                    buf.write_all(b"\"")?;
                    buf.write_all(self.as_bytes())?;
                    buf.write_all(b"\"")?;

                    Ok(())
                }
            }
        )*
    };
}

serialize_str! {
    String,
    &str,
    &String,
    Box<str>,
    std::borrow::Cow<'_, str>,
}

pub trait Serialize<W: Write> {
    fn serialize(&self, buf: &mut W) -> std::io::Result<()>;
}

impl<W: Write> Serialize<W> for () {
    fn serialize(&self, _buf: &mut W) -> std::io::Result<()> {
        Ok(())
    }
}

macro_rules! serialize_int {
    ($($type:ty),* $(,)?) => {
        $(
            impl<W: Write> Serialize<W> for $type {
                fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
                    let mut n_buf = itoa::Buffer::new();
                    let n_str = n_buf.format(*self);
                    buf.write_all(n_str.as_bytes())
                }
            }
        )*
    };
}

macro_rules! serialize_float {
    ($($type:ty),* $(,)?) => {
        $(
            impl<W: Write> Serialize<W> for $type {
                fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
                    let mut n_buf = ryu::Buffer::new();
                    let n_str = n_buf.format(*self);
                    buf.write_all(n_str.as_bytes())
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

impl<W: Write> Serialize<W> for bool {
    fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
        if *self {
            buf.write_all(b"true")?;
        } else {
            buf.write_all(b"false")?;
        }

        Ok(())
    }
}

impl<T, W: Write, S: SerializeStr<W>> Serialize<W> for HashMap<S, T>
where
    T: Serialize<W>,
{
    fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
        buf.write_all(b"{")?;
        for (i, (k, v)) in self.iter().enumerate() {
            if i > 0 {
                buf.write_all(b",")?;
            }

            k.serialize_str(buf)?;

            buf.write_all(b":")?;

            v.serialize(buf)?;
        }
        buf.write_all(b"}")
    }
}

impl<T, W: Write, S: SerializeStr<W>> Serialize<W> for BTreeMap<S, T>
where
    T: Serialize<W>,
{
    fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
        buf.write_all(b"{")?;
        for (i, (k, v)) in self.iter().enumerate() {
            if i > 0 {
                buf.write_all(b",")?;
            }

            k.serialize_str(buf)?;

            buf.write_all(b":")?;

            v.serialize(buf)?;
        }
        buf.write_all(b"}")
    }
}

impl<T, W: Write> Serialize<W> for Option<T>
where
    T: Serialize<W>,
{
    fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
        match self {
            Some(t) => t.serialize(buf),
            None => buf.write_all(b"null"),
        }
    }
}

fn serialize_slice_like<T: AsRef<[S]>, S: Serialize<W>, W: Write>(
    arr: T,
    buf: &mut W,
) -> std::io::Result<()> {
    let arr: &[S] = arr.as_ref();

    buf.write_all(b"[")?;
    for (i, item) in arr.iter().enumerate() {
        if i > 0 {
            buf.write_all(b",")?;
        }

        item.serialize(buf)?;
    }
    buf.write_all(b"]")
}

impl<T, W: Write, const N: usize> Serialize<W> for [T; N]
where
    T: Serialize<W>,
{
    fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
        serialize_slice_like(self, buf)
    }
}

macro_rules! serialize_slice_like {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<T, W: Write> Serialize<W> for $ty
            where
                T: Serialize<W>,
            {
                fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
                    serialize_slice_like(self, buf)
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

// WARN: Maybe this is a mistake
impl<F, W: Write> Serialize<W> for F
where
    F: Fn(&mut W) -> std::io::Result<()>,
{
    fn serialize(&self, buf: &mut W) -> std::io::Result<()> {
        self(buf)
    }
}
