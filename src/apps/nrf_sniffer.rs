use embedded_hal_bus::spi::AtomicDevice;
use esp_hal::{delay::Delay, gpio::Output, spi::master::Spi};
use nrf24_rs::{config::DataPipe, Nrf24l01};

use super::App;

type NrfChip<'a> =
    Nrf24l01<AtomicDevice<'a, Spi<'a, esp_hal::Blocking>, Output<'a>, Delay>, Output<'a>>;

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
        let _ = self.chip.open_reading_pipe(DataPipe::DP0, b"Node1");
        let _ = self.chip.start_listening();
    }

    fn disable(&mut self) {
        self.enabled = false;
        let _ = self.chip.stop_listening();
    }

    fn is_enabled(&mut self) -> bool {
        self.enabled
    }
}
