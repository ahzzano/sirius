pub trait App {
    fn enable(&mut self);
    fn run(&mut self);
    fn disable(&mut self);
}

pub mod sniffer;
