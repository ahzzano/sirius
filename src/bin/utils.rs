#![no_std]
#![no_main]

use esp_hal::time::{Duration, Instant};

pub fn wait(duration: Duration) {
    let t = Instant::now() + duration;
    while Instant::now() < t {}
}
