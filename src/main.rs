mod event;
mod executor;
mod render;
mod window;

use event::Event;
use executor::Executor;
use gni::output::parse_command;
use render::Render;
use window::Window;

struct App {
    exe: Executor,
}

impl Event for App {
    fn resize(&mut self, (width, height): (u32, u32)) {
        println!("{}, {}", width, height);
    }

    fn draw(&mut self) {
        let mut line = String::new();
        loop {
            line.clear();
            std::io::stdin().read_line(&mut line).unwrap();

            let mut input = line.bytes();
            match parse_command(&mut input, &mut self.exe) {
                Ok(true) => (),
                Ok(false) => return,
                Err(err) => panic!("{:?}", err),
            }
        }
    }
}

fn main() {
    let window = Window::new("gni");
    let render = Render::new(&window);
    render.check_error();

    let exe = Executor::new(render);
    window.run(App { exe }, 60);
}
