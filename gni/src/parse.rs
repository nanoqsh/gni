use crate::hex;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    Byte(u8),
    End,
    NotNewLine,
    ZeroIndex,
}

impl ParseError {
    pub(crate) fn next<B>(bytes: &mut B) -> Result<u8, Self>
    where
        B: Iterator<Item = u8>,
    {
        bytes.next().ok_or(Self::End)
    }
}

impl From<u8> for ParseError {
    fn from(b: u8) -> Self {
        Self::Byte(b)
    }
}

pub trait Parse<B>: Sized
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError>;

    fn from_bytes<I>(bytes: I) -> Result<Self, ParseError>
    where
        I: IntoIterator<IntoIter = B>,
    {
        let mut bytes = bytes.into_iter();
        Self::parse(&mut bytes)
    }
}

impl<B> Parse<B> for u8
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError> {
        let a = ParseError::next(bytes)?;
        let b = ParseError::next(bytes)?;
        hex::read_u8([a, b]).map_err(ParseError::Byte)
    }
}

impl<B> Parse<B> for u16
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError> {
        let a = u8::parse(bytes)? as u16;
        let b = u8::parse(bytes)? as u16;
        Ok(a << 8 | b)
    }
}
