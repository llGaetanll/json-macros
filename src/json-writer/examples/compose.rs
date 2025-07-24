fn main() {
    let mut buf: Vec<u8> = Vec::new();

    let real = |buf: &mut Vec<u8>| {
        buf.extend(b"true");
    };

    let verdict = |buf: &mut Vec<u8>| {
        buf.push(b'{');

        buf.push(b'"');
        buf.extend_from_slice("verdict".as_bytes());
        buf.push(b'"');

        buf.push(b':');

        real(buf);

        buf.push(b'}');
    };

    verdict(&mut buf);
}
