use crate::{hex, Parse, ParseError};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nib(u8);

impl Nib {
    pub fn new(byte: u8) -> Option<Self> {
        match byte {
            0..=0x0F => Some(Self(byte)),
            _ => None,
        }
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl<B> Parse<B> for Nib
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError> {
        let byte = ParseError::next(bytes)?;
        let idx = hex::read_u4(byte)?;
        Ok(Self::new(idx).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let actual = Nib::from_bytes(*b"a");
        let expected = Ok(Nib::new(0x0A).unwrap());
        assert_eq!(actual, expected);

        let actual = Nib::from_bytes(*b"q");
        let expected = Err(ParseError::Byte(b'q'));
        assert_eq!(actual, expected);
    }
}
