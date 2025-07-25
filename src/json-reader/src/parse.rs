pub trait Parse: Sized {
    type Error;

    /// Extract `Self` and return the rest of the slice
    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error>;

    /// Like `parse`, but maps the result through `f` if it succeefs. Allows for cleaner code
    fn parse_map<F, T>(input: &[u8], f: F) -> Result<(T, &[u8]), Self::Error>
    where
        F: Fn(Self) -> T,
    {
        let (res, input) = Self::parse(input)?;

        Ok((f(res), input))
    }
}
