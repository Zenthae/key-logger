pub trait Threaded {
    fn new() -> Self;

    fn init(&mut self);

    fn run(&mut self);

    fn stop(&mut self);
}
