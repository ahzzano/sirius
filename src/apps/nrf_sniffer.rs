use embedded_hal_bus::spi::AtomicDevice;
use esp_hal::{delay::Delay, gpio::Output, spi::master::Spi};
use nrf24_rs::{config::DataPipe, Nrf24l01};
