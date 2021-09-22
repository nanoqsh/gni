use glow::{Context, HasContext, NativeBuffer, NativeVertexArray};
use std::rc::Rc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
    pub col: u32,
}

pub struct DrawBuffer {
    context: Rc<Context>,
    buffer: Box<[Vertex]>,
    len: usize,
    nat: (NativeVertexArray, NativeBuffer),
}

impl DrawBuffer {
    pub fn new(context: Rc<Context>) -> Self {
        const ZERO: Vertex = Vertex {
            pos: [0., 0., 0.],
            tex: [0., 0.],
            col: 0,
        };

        let n_triangles = if cfg!(debug_assertions) {
            1 << 4
        } else {
            1 << 7
        };

        let buffer_len = n_triangles * 3;
        let buffer = vec![ZERO; buffer_len];
        let nat = unsafe {
            let array = context
                .create_vertex_array()
                .expect("Cannot create vertex array");
            context.bind_vertex_array(Some(array));

            let vertex_size = std::mem::size_of::<Vertex>() as i32;
            let size = std::mem::size_of::<f32>() as i32;
            let buffer = context.create_buffer().expect("Cannot create buffer");
            context.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            context.buffer_data_size(
                glow::ARRAY_BUFFER,
                buffer_len as i32 * vertex_size,
                glow::STREAM_DRAW,
            );

            let attributes = [(0, 3, glow::FLOAT, 0), (1, 2, glow::FLOAT, 3 * size)];
            for (loc, size, data_type, offset) in attributes {
                context.vertex_attrib_pointer_f32(loc, size, data_type, false, vertex_size, offset);
                context.enable_vertex_attrib_array(loc);
            }

            let loc = 2;
            context.vertex_attrib_pointer_i32(loc, 1, glow::UNSIGNED_INT, vertex_size, 5 * size);
            context.enable_vertex_attrib_array(loc);

            (array, buffer)
        };

        Self {
            context,
            buffer: buffer.into_boxed_slice(),
            len: 0,
            nat,
        }
    }

    pub fn add(&mut self, triangle: [Vertex; 3]) -> bool {
        let len = self.len + 3;
        if len > self.buffer.len() {
            false
        } else {
            self.buffer[self.len..len].copy_from_slice(&triangle);
            self.len = len;
            true
        }
    }

    pub fn draw(&self) {
        if self.len == 0 {
            return;
        }

        unsafe {
            let (array, buffer) = self.nat;
            self.context.bind_vertex_array(Some(array));

            let slice = &self.buffer[..self.len];
            let src = std::slice::from_raw_parts(
                slice.as_ptr().cast(),
                slice.len() * std::mem::size_of::<Vertex>(),
            );

            self.context.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            self.context
                .buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, src);
            self.context
                .draw_arrays(glow::TRIANGLES, 0, self.len as i32);
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl Drop for DrawBuffer {
    fn drop(&mut self) {
        let (array, buffer) = self.nat;
        unsafe {
            self.context.delete_vertex_array(array);
            self.context.delete_buffer(buffer);
        }
    }
}
