use crate::render::{Render, Vertex};
use gni::{output::Output, Col, Img, Nib, Pnt, Tri};

pub struct Executor {
    render: Render,
}

impl Executor {
    pub fn new(render: Render) -> Self {
        Self { render }
    }

    fn palette(&mut self, idx: Nib, Col([r, g, b]): Col) {
        let qc = r as f32 / 255.;
        let wc = g as f32 / 255.;
        let ec = b as f32 / 255.;
        self.render.set_color(idx.get(), [qc, wc, ec]);
        self.render.check_error();
    }

    fn clear(&self, idx: Nib) {
        self.render.clear(idx.get());
        self.render.check_error();
    }

    fn draw_triangle(&mut self, Tri([a, b, c]): Tri) {
        fn vertex(p: Pnt) -> Vertex {
            const ADDITION: f32 = 1. / 512.;

            let xp = p.pos[0] as f32 / 256.;
            let yp = p.pos[1] as f32 / 256.;
            let zp = p.pos[2] as f32 / 256.;
            let ut = p.tex[0] as f32 / 256. + ADDITION;
            let vt = p.tex[1] as f32 / 256. + ADDITION;
            let col = p.col.get() as u32;

            Vertex {
                pos: [xp, yp, zp],
                tex: [ut, vt],
                col,
            }
        }

        let qv = vertex(a);
        let wv = vertex(b);
        let ev = vertex(c);
        self.render.add_to_buffer([qv, wv, ev]);
    }

    fn image(&mut self, idx: u8, img: Img) {
        self.render.add_image(idx, &img)
    }

    fn set_image(&mut self, idx: u8) {
        self.render.set_image(idx)
    }

    fn finish(&mut self) {
        self.render.draw_buffer();
        self.render.check_error();
    }
}

impl Output for Executor {
    fn palette(&mut self, idx: Nib, col: Col) {
        Self::palette(self, idx, col)
    }

    fn clear(&mut self, idx: Nib) {
        Self::clear(self, idx)
    }

    fn draw_triangle(&mut self, tri: Tri) {
        Self::draw_triangle(self, tri)
    }

    fn image(&mut self, idx: u8, img: Img) {
        Self::image(self, idx, img)
    }

    fn set_image(&mut self, idx: u8) {
        Self::set_image(self, idx)
    }

    fn finish(&mut self) {
        Self::finish(self)
    }
}
