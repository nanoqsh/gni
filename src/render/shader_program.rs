use glow::{Context, HasContext, NativeProgram, NativeUniformLocation};
use std::rc::Rc;

pub struct Program {
    context: Rc<Context>,
    program: NativeProgram,
}

impl Program {
    const VERTEX_SHADER: &'static str = r#"
        #version 330 core
        layout (location = 0) in vec3 pos;
        layout (location = 1) in vec2 tex;
        layout (location = 2) in uint col;
        
        uniform vec3 palette[16u];
        
        out vec2 fs_tex;
        out vec3 fs_col;
        void main() {
            fs_tex = tex;
            if (col >= 16u) {
                fs_col = vec3(1.0, 0.0, 0.0);
            } else {
                fs_col = palette[col];
            }
            
            gl_Position = vec4(pos, 1.0);
        }"#;

    const FRAGMENT_SHADER: &'static str = r#"
        #version 330 core
        uniform usampler2D tex;
        uniform vec3 palette[16u];
        uniform bool use_tex;
        
        in vec2 fs_tex;
        in vec3 fs_col;
        out vec4 color;
        void main() {
            vec3 tex_col;
            if (use_tex) {
                uint i = texture(tex, fs_tex).r;
                tex_col = palette[i];
            } else {
                tex_col = vec3(1.0);
            }
            
            color = vec4(tex_col * fs_col, 1.0);
        }"#;

    pub fn new(context: Rc<Context>) -> Self {
        unsafe {
            let program = context.create_program().expect("Cannot create program");
            let shader_sources = [
                (glow::VERTEX_SHADER, Self::VERTEX_SHADER),
                (glow::FRAGMENT_SHADER, Self::FRAGMENT_SHADER),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());
            for (shader_type, shader_source) in shader_sources {
                let shader = context
                    .create_shader(shader_type)
                    .expect("Cannot create shader");

                context.shader_source(shader, shader_source);
                context.compile_shader(shader);
                if !context.get_shader_compile_status(shader) {
                    panic!("{}", context.get_shader_info_log(shader));
                }

                context.attach_shader(program, shader);
                shaders.push(shader);
            }

            context.link_program(program);
            if !context.get_program_link_status(program) {
                panic!("{}", context.get_program_info_log(program));
            }

            for shader in shaders {
                context.detach_shader(program, shader);
                context.delete_shader(shader);
            }

            context.use_program(Some(program));

            Self { context, program }
        }
    }

    pub fn tex_loc(&self) -> NativeUniformLocation {
        self.loc("tex")
    }

    pub fn palette_loc(&self) -> NativeUniformLocation {
        self.loc("palette")
    }

    pub fn use_tex_loc(&self) -> NativeUniformLocation {
        self.loc("use_tex")
    }

    fn loc(&self, name: &str) -> NativeUniformLocation {
        unsafe {
            self.context
                .get_uniform_location(self.program, name)
                .unwrap()
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.context.delete_program(self.program) }
    }
}
