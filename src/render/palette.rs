use glow::{Context, HasContext, NativeUniformLocation};
use std::rc::Rc;

type Color = [f32; 3];

pub struct Palette {
    context: Rc<Context>,
    colors: [Color; Self::SIZE],
    loc: NativeUniformLocation,
}

impl Palette {
    const SIZE: usize = 16;

    pub fn new(context: Rc<Context>, loc: NativeUniformLocation) -> Self {
        const BLACK: Color = [0., 0., 0.];

        Self {
            context,
            colors: [BLACK; Self::SIZE],
            loc,
        }
    }

    pub fn colors(&self) -> &[Color] {
        &self.colors
    }

    pub fn colors_mut(&mut self) -> &mut [Color] {
        &mut self.colors
    }

    pub fn set_uniform(&self) {
        unsafe {
            let colors = &self.colors;
            let data = std::slice::from_raw_parts(colors.as_ptr().cast(), colors.len() * 3);
            self.context.uniform_3_f32_slice(Some(&self.loc), data);
        }
    }
}
