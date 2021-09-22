use crate::{Nib, Parse, ParseError};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pnt {
    pub pos: [i16; 3],
    pub tex: [u8; 2],
    pub col: Nib,
}

impl<B> Parse<B> for Pnt
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError> {
        let x = u16::parse(bytes)? as i16;
        let y = u16::parse(bytes)? as i16;
        let z = u16::parse(bytes)? as i16;
        let u = u8::parse(bytes)?;
        let v = u8::parse(bytes)?;
        let col = Nib::parse(bytes)?;

        Ok(Self {
            pos: [x, y, z],
            tex: [u, v],
            col,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let actual = Pnt::from_bytes(*b"0011223344551122e");
        let expected = Ok(Pnt {
            pos: [0x0011, 0x2233, 0x4455],
            tex: [0x11, 0x22],
            col: Nib::new(0x0E).unwrap(),
        });
        assert_eq!(actual, expected);
    }
}
