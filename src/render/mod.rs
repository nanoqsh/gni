mod draw_buffer;
mod images;
mod palette;
mod shader_program;

use crate::Window;
use draw_buffer::DrawBuffer;
use glow::{Context, HasContext};
use gni::Img;
use images::Images;
use palette::Palette;
use shader_program::Program;
use std::rc::Rc;

pub use draw_buffer::Vertex;

pub struct Render {
    context: Rc<Context>,
    _program: Program,
    buffer: DrawBuffer,
    images: Images,
    palette: Palette,
}

impl Render {
    pub fn new(window: &Window) -> Self {
        let context = unsafe {
            Context::from_loader_function(|s| window.context().get_proc_address(s).cast()).into()
        };

        let program = Program::new(Rc::clone(&context));
        let buffer = DrawBuffer::new(Rc::clone(&context));
        let images = Images::new(Rc::clone(&context), &program);
        let palette = Palette::new(Rc::clone(&context), program.palette_loc());
        palette.set_uniform();

        Self {
            context,
            _program: program,
            buffer,
            images,
            palette,
        }
    }

    pub fn clear(&self, idx: u8) {
        let [r, g, b] = self.palette.colors()[idx as usize];
        unsafe {
            let mask = glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT;
            self.context.clear(mask);
            self.context.clear_color(r, g, b, 1.);
        }
    }

    pub fn add_to_buffer(&mut self, triangle: [Vertex; 3]) {
        if self.buffer.add(triangle) {
            return;
        }

        self.draw_buffer();
        assert!(self.buffer.add(triangle));
    }

    pub fn draw_buffer(&mut self) {
        self.buffer.draw();
        self.buffer.clear();
    }

    pub fn add_image(&mut self, idx: u8, img: &Img) {
        unsafe {
            let data = img.data();
            self.images.add(
                idx,
                img.size(),
                std::slice::from_raw_parts(data.as_ptr().cast(), data.len()),
            );
        }
    }

    pub fn set_image(&mut self, idx: u8) {
        self.images.bind(idx)
    }

    pub fn set_color(&mut self, idx: u8, color: [f32; 3]) {
        let colors = self.palette.colors_mut();
        colors[idx as usize] = color;
        self.palette.set_uniform();
    }

    pub fn check_error(&self) {
        let err = unsafe { self.context.get_error() };
        let msg = match err {
            glow::NO_ERROR => return,
            glow::INVALID_ENUM => "Invalid enum",
            glow::INVALID_VALUE => "Invalid value",
            glow::INVALID_OPERATION => "Invalid operation",
            glow::INVALID_FRAMEBUFFER_OPERATION => "Invalid framebuffer operation",
            glow::OUT_OF_MEMORY => "Out of memory",
            glow::STACK_UNDERFLOW => "Stack underflow",
            glow::STACK_OVERFLOW => "Stack overflow",
            _ => "Undefined error",
        };
        panic!("{}", msg);
    }
}
