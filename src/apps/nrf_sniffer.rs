use embassy_executor::Spawner;
use embedded_hal_bus::spi::AtomicDevice;
use esp_hal::{
    delay::Delay,
    gpio::{Output, OutputPin},
    peripherals::SPI2,
    spi::master::Spi,
};
use esp_println::println;
use nrf24_rs::{config::DataPipe, Nrf24l01, MAX_PAYLOAD_SIZE};

use super::App;

type NrfChip<'a> =
    Nrf24l01<AtomicDevice<'a, Spi<'a, esp_hal::Blocking>, Output<'a>, Delay>, Output<'a>>;

#[embassy_executor::task]
async fn read_chip(nrf: &mut NrfChip<'static>, check: fn() -> bool) {
    let mut buf = [0u8; MAX_PAYLOAD_SIZE as usize];
    while nrf.data_available().is_ok() && check() {
        let res = nrf.read(&mut buf);
        if res.is_ok() {
            println!("Read")
        }
    }
}

pub struct NRFSniffer<'a> {
    chip: NrfChip<'a>,
    enabled: bool,
}

impl<'a> NRFSniffer<'a> {
    pub fn new(chip: NrfChip<'a>) -> Self {
        NRFSniffer {
            enabled: true,
            chip,
        }
    }

    pub fn init(&mut self) {}
}

impl<'a> App for NRFSniffer<'a> {
    fn enable(&mut self) {
        self.enabled = true;
        self.chip.open_reading_pipe(DataPipe::DP0, b"Node1");
        self.chip.start_listening();
        read_chip(&mut self.chip, || self.is_enabled());
    }

    fn disable(&mut self) {
        self.enabled = false;
        self.chip.stop_listening();
    }

    fn is_enabled(&mut self) -> bool {
        self.enabled
    }
}
