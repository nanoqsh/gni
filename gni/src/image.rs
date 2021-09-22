use crate::{Nib, Parse, ParseError};

#[derive(Debug, Eq, PartialEq)]
pub struct Img {
    data: Box<[Nib]>,
    size: (u8, u8),
}

impl Img {
    pub fn new<D>(data: D, size: (u8, u8)) -> Option<Self>
    where
        D: Into<Box<[Nib]>>,
    {
        let data = data.into();
        if data.len() != size.0 as usize * size.1 as usize {
            return None;
        }

        Some(Self { data, size })
    }

    pub fn data(&self) -> &[Nib] {
        &self.data
    }

    pub fn size(&self) -> (u8, u8) {
        self.size
    }
}

impl<B> Parse<B> for Img
where
    B: Iterator<Item = u8>,
{
    fn parse(bytes: &mut B) -> Result<Self, ParseError> {
        let w = u8::parse(bytes)?;
        let h = u8::parse(bytes)?;
        let len = w as usize * h as usize;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            let col = Nib::parse(bytes)?;
            data.push(col);
        }
        Ok(Self::new(data, (w, h)).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let actual = Img::from_bytes(
            *b"0404\
            0123\
            4567\
            89ab\
            cdef",
        );

        let expected = Ok(Img::new(
            [
                Nib::new(0x0).unwrap(),
                Nib::new(0x1).unwrap(),
                Nib::new(0x2).unwrap(),
                Nib::new(0x3).unwrap(),
                Nib::new(0x4).unwrap(),
                Nib::new(0x5).unwrap(),
                Nib::new(0x6).unwrap(),
                Nib::new(0x7).unwrap(),
                Nib::new(0x8).unwrap(),
                Nib::new(0x9).unwrap(),
                Nib::new(0xA).unwrap(),
                Nib::new(0xB).unwrap(),
                Nib::new(0xC).unwrap(),
                Nib::new(0xD).unwrap(),
                Nib::new(0xE).unwrap(),
                Nib::new(0xF).unwrap(),
            ],
            (4, 4),
        )
        .unwrap());

        assert_eq!(actual, expected);
    }
}
