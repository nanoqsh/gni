use crate::{Col, Img, Nib, Parse, ParseError, Tri};

pub trait Output {
    fn palette(&mut self, idx: Nib, col: Col);

    fn clear(&mut self, idx: Nib);

    fn draw_triangle(&mut self, tri: Tri);

    fn image(&mut self, idx: u8, img: Img);

    fn set_image(&mut self, idx: u8);

    fn finish(&mut self);
}

pub fn parse_command<B, O>(bytes: &mut B, out: &mut O) -> Result<bool, ParseError>
where
    B: Iterator<Item = u8>,
    O: Output,
{
    let next = match bytes.next() {
        None => return Ok(false),
        Some(next) => next,
    };

    match next {
        b'p' => {
            let idx = Nib::parse(bytes)?;
            let col = Col::parse(bytes)?;
            out.palette(idx, col)
        }
        b'c' => {
            let idx = Nib::parse(bytes)?;
            out.clear(idx)
        }
        b't' => {
            let tri = Tri::parse(bytes)?;
            out.draw_triangle(tri)
        }
        b'i' => {
            let idx = u8::parse(bytes)?;
            if idx == 0 {
                return Err(ParseError::ZeroIndex);
            }

            let img = Img::parse(bytes)?;
            out.image(idx, img)
        }
        b's' => match ParseError::next(bytes)? {
            b'i' => {
                let idx = u8::parse(bytes)?;
                out.set_image(idx)
            }
            next => return Err(ParseError::Byte(next)),
        },
        b'\n' => {
            out.finish();
            return Ok(false);
        }
        _ => return Err(ParseError::Byte(next)),
    }

    match bytes.next() {
        Some(b'\n') => Ok(true),
        _ => Err(ParseError::NotNewLine),
    }
}
