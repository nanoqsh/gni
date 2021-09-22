use crate::{Parse, ParseError, Pnt};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tri(pub [Pnt; 3]);

impl<B> Parse<B> for Tri
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError> {
        let a = Pnt::parse(bytes)?;
        let b = Pnt::parse(bytes)?;
        let c = Pnt::parse(bytes)?;

        Ok(Self([a, b, c]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Nib;

    #[test]
    fn parse() {
        let point = Pnt {
            pos: [0x0011, 0x2233, 0x4455],
            tex: [0x66, 0x77],
            col: Nib::new(0x08).unwrap(),
        };
        let actual = Tri::from_bytes(*b"001122334455667780011223344556677800112233445566778");
        let expected = Ok(Tri([point, point, point]));
        assert_eq!(actual, expected);
    }
}
