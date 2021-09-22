mod color;
mod hex;
mod image;
pub mod input;
mod nibble;
pub mod output;
mod parse;
mod point;
mod triangle;

pub use crate::{
    color::Col,
    image::Img,
    nibble::Nib,
    parse::{Parse, ParseError},
    point::Pnt,
    triangle::Tri,
};
