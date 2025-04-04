#![no_std]
#![no_main]

use core::cell::RefCell;

use log::error;
#[allow(unused_imports)]
use sirius::apps::wifi_sniffer::WifiSniffer;
use sirius::apps::AsyncApp;
use sirius::apps::SyncApp;
#[allow(unused_imports)]
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

    let wifi = WiFi::new(&_init, peripherals.WIFI);
    let mut sniff_sync = WifiSniffer::new(RefCell::new(wifi));

    sniff_sync.init();
    sniff_sync.enable();

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
