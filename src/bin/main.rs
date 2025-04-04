#![no_std]
#![no_main]

use core::cell::RefCell;

use embedded_hal_bus::spi::AtomicDevice;
// use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_hal_bus::util::AtomicCell;
use esp_hal::delay::Delay;
use esp_hal::gpio::Level;
use esp_hal::gpio::Output;
use esp_hal::gpio::OutputConfig;
use esp_hal::spi::master::Config;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_println::print;
use esp_println::println;
use ieee80211::match_frames;
use ieee80211::mgmt_frame::BeaconFrame;
use log::error;
use nrf24_rs::config::DataPipe;
use nrf24_rs::config::NrfConfig;
use nrf24_rs::Nrf24l01;
use nrf24_rs::MAX_PAYLOAD_SIZE;
use sirius::apps::sniffer::WifiSniffer;
use sirius::apps::App;
use sirius::devices::wifi::WiFi;

use embassy_executor::Spawner;
use embassy_time::Duration;
use embassy_time::Timer;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use log::info;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let message = info.message();
    error!("Error: {message}");
    loop {}
}

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // let mut wifi = WiFi::new(&_init, peripherals.WIFI);
    // wifi.init();
    // let mut sniffer = WifiSniffer::new(RefCell::new(wifi));

    // sniffer.set_callback(|_packet| {
    //     // println!("{data:2x?}")
    //     let _ = match_frames!(_packet.data, beacon=BeaconFrame => {
    //         let ssid = beacon.ssid();
    //         print!("{ssid:?}");
    //         let recv = beacon.header.receiver_address;
    //         let transmitter = beacon.header.transmitter_address;
    //         print!(" {transmitter:?} {recv:?}\n")
    //     });
    // });
    // sniffer.init();
    // sniffer.enable();
    // sniffer.disable();

    // let spi = Spi::new(
    //     peripherals.SPI2,
    //     Config::default()
    //         .with_frequency(Rate::from_khz(100))
    //         .with_mode(esp_hal::spi::Mode::_0),
    // )
    // .unwrap()
    // .with_miso(peripherals.GPIO19)
    // .with_mosi(peripherals.GPIO5)
    // .with_sck(peripherals.GPIO18);

    // let mut _delays = Delay::new();

    // let cs = Output::new(peripherals.GPIO17, Level::High, OutputConfig::default());

    // let atomic_cell = AtomicCell::new(spi);
    // let nrf_dev = AtomicDevice::new(&atomic_cell, cs, _delays).unwrap();
    // let mut delays = Delay::new();

    // let ce = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());

    // let mut nrf = Nrf24l01::new(
    //     nrf_dev,
    //     ce,
    //     &mut delays,
    //     NrfConfig::default()
    //         .channel(11)
    //         .pa_level(nrf24_rs::config::PALevel::Min)
    //         .payload_size(MAX_PAYLOAD_SIZE)
    //         .addr_width(5)
    //         .crc_encoding_scheme(nrf24_rs::config::EncodingScheme::NoRedundancyCheck),
    // )
    // .unwrap();

    // let connectivity = nrf.is_connected();

    // if let Ok(connected) = connectivity {
    //     info!("NRF Status > {connected}");
    // }

    // let _ = nrf.open_reading_pipe(DataPipe::DP0, b"0000");
    // let _ = nrf.start_listening();

    // let mut buf = [0u8; MAX_PAYLOAD_SIZE as usize];
    // while nrf.data_available().is_ok() {
    //     let res = nrf.read(&mut buf);

    //     if res.is_ok() {
    //         println!("Read: {buf:02x?}");
    //     }
    // }

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        // info!("Hello world!");
        // println!("NRF Status: {nrf_dev:?}");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
