use crate::render::shader_program::Program;
use glow::{Context, HasContext, NativeTexture, NativeUniformLocation};
use std::{
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

pub struct Images {
    context: Rc<Context>,
    map: HashMap<u8, NativeTexture>,
    use_tex_loc: NativeUniformLocation,
    active: u8,
}

impl Images {
    pub fn new(context: Rc<Context>, program: &Program) -> Self {
        let use_tex_loc = program.use_tex_loc();
        unsafe {
            context.uniform_1_i32(Some(&program.tex_loc()), 0);
            context.active_texture(glow::TEXTURE0);
            context.uniform_1_u32(Some(&use_tex_loc), 0);
        }

        Self {
            context,
            map: HashMap::default(),
            use_tex_loc,
            active: 0,
        }
    }

    pub fn add(&mut self, idx: u8, size: (u8, u8), data: &[u8]) {
        let tex = unsafe {
            assert_ne!(idx, 0);
            assert_eq!(size.0 as usize * size.1 as usize, data.len());

            let tex = self
                .context
                .create_texture()
                .expect("Cannot create texture");
            self.context.bind_texture(glow::TEXTURE_2D, Some(tex));
            self.context.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.context.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            let (width, height) = size;
            self.context.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::R8 as i32,
                width as i32,
                height as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(data),
            );

            tex
        };

        match self.map.entry(idx) {
            Entry::Occupied(mut en) => unsafe {
                let old = en.insert(tex);
                if self.active == idx {
                    self.bind_texture(tex);
                }

                self.context.delete_texture(old);
            },
            Entry::Vacant(en) => {
                en.insert(tex);
            }
        }
    }

    pub fn bind(&mut self, idx: u8) {
        let use_tex = if let Some(tex) = self.map.get(&idx) {
            self.active = idx;
            self.bind_texture(*tex);
            1
        } else {
            self.active = 0;
            0
        };

        unsafe {
            self.context.uniform_1_u32(Some(&self.use_tex_loc), use_tex);
        }
    }

    fn bind_texture(&self, tex: NativeTexture) {
        unsafe { self.context.bind_texture(glow::TEXTURE_2D, Some(tex)) }
    }
}

impl Drop for Images {
    fn drop(&mut self) {
        for tex in self.map.values() {
            unsafe { self.context.delete_texture(*tex) }
        }
    }
}
