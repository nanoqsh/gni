use crate::{Parse, ParseError};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Col(pub [u8; 3]);

impl Col {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b])
    }
}

impl<B> Parse<B> for Col
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError> {
        let r = u8::parse(bytes)?;
        let g = u8::parse(bytes)?;
        let b = u8::parse(bytes)?;
        Ok(Self::new(r, g, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let actual = Col::from_bytes(*b"001122");
        let expected = Ok(Col::new(0x00, 0x11, 0x22));
        assert_eq!(actual, expected);
    }
}
