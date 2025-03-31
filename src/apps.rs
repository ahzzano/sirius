pub mod sniffer;

pub trait App {
    fn enable(&mut self);
    // fn run(&mut self);
    fn disable(&mut self);
    fn is_enabled(&mut self) -> bool;
}
