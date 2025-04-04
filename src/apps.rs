pub mod nrf_sniffer;
pub mod wifi_sniffer;

pub struct Sync;
pub struct Async;

pub trait SyncApp {
    fn enable(&mut self);
    fn disable(&mut self);
    fn is_enabled(&mut self) -> bool;
}

pub trait AsyncApp {
    fn task(&mut self); // the thing that gets sent to Embassy
}
