use core::cell::RefCell;

use esp_wifi::wifi::PromiscuousPkt;

use crate::devices::wifi::WiFi;

use super::App;

pub struct WifiSniffer<'a> {
    wifi: RefCell<WiFi<'a>>,
    enabled: bool,
}

#[allow(clippy::needless_lifetimes)]
impl<'a> WifiSniffer<'a> {
    pub fn new(wifi: RefCell<WiFi<'a>>) -> Self {
        WifiSniffer {
            wifi,
            enabled: false,
        }
    }

    pub fn set_callback(&mut self, callback: fn(PromiscuousPkt)) {
        self.wifi.borrow_mut().set_sniffer_callback(callback);
    }

    pub fn init(&mut self) {
        self.wifi.borrow_mut().init();
    }
}

impl App for WifiSniffer<'_> {
    fn enable(&mut self) {
        self.enabled = true;
        self.wifi.borrow_mut().set_promiscuous_mode(true);
    }

    // fn run(&mut self) {}

    fn disable(&mut self) {
        self.wifi.borrow_mut().set_promiscuous_mode(false);
        self.enabled = false;
    }

    fn is_enabled(&mut self) -> bool {
        self.enabled
    }
}
