pub trait Event {
    fn resize(&mut self, size: (u32, u32));

    fn draw(&mut self);
}
