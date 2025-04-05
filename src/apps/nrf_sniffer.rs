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
