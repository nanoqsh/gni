type Context = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
type EventLoop = glutin::event_loop::EventLoop<()>;

pub struct Window {
    context: Context,
    event_loop: EventLoop,
}

impl Window {
    pub fn new<T>(title: T) -> Self
    where
        T: Into<String>,
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_resizable(true);

        let context = unsafe {
            glutin::ContextBuilder::new()
                .with_vsync(true)
                .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        Self {
            context,
            event_loop,
        }
    }

    pub fn context(&self) -> &glutin::Context<glutin::PossiblyCurrent> {
        self.context.context()
    }

    pub fn run<E>(self, mut ev: E, fps: u32) -> !
    where
        E: crate::event::Event + 'static,
    {
        use glutin::{
            event::{Event, StartCause, WindowEvent},
            event_loop::ControlFlow,
        };
        use std::time::{Duration, Instant};

        let micros = if fps == 0 { 0 } else { 1_000_000 / fps as u64 };
        let context = self.context;
        self.event_loop.run(move |event, _, flow| {
            match event {
                Event::WindowEvent { event, .. } => {
                    return match event {
                        WindowEvent::Resized(size) => {
                            context.resize(size);
                            ev.resize(size.into());
                        }
                        WindowEvent::CloseRequested => {
                            *flow = ControlFlow::Exit;
                        }
                        _ => (),
                    }
                }
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached { .. } | StartCause::Poll => {
                        ev.draw();
                        context.swap_buffers().unwrap();
                    }
                    StartCause::WaitCancelled {
                        requested_resume, ..
                    } => {
                        let instant = requested_resume.unwrap();
                        *flow = ControlFlow::WaitUntil(instant);
                        return;
                    }
                    StartCause::Init => (),
                },
                _ => return,
            }

            *flow = if micros == 0 {
                ControlFlow::Poll
            } else {
                ControlFlow::WaitUntil(Instant::now() + Duration::from_micros(micros))
            };
        })
    }
}
