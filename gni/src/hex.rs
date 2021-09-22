pub(crate) fn read_u4(a: u8) -> Result<u8, u8> {
    match a {
        b'0'..=b'9' => Ok(a - b'0'),
        b'a'..=b'f' => Ok(a - b'a' + 10),
        _ => Err(a),
    }
}

pub(crate) fn read_u8([a, b]: [u8; 2]) -> Result<u8, u8> {
    Ok(read_u4(a)? << 4 | read_u4(b)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let actual = read_u8(*b"00");
        let expected = Ok(0x00);
        assert_eq!(actual, expected);

        let actual = read_u8(*b"0f");
        let expected = Ok(0x0F);
        assert_eq!(actual, expected);

        let actual = read_u8(*b"ff");
        let expected = Ok(0xFF);
        assert_eq!(actual, expected);

        let actual = read_u8(*b"FF");
        let expected = Err(b'F');
        assert_eq!(actual, expected);

        let actual = read_u8(*b"qw");
        let expected = Err(b'q');
        assert_eq!(actual, expected);
    }
}
