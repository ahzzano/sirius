use esp_wifi::wifi::PromiscuousPkt;

use crate::devices::wifi::WiFi;

use super::App;

pub struct WifiSniffer<'a> {
    wifi: WiFi<'a>,
    enabled: bool,
}

#[allow(clippy::needless_lifetimes)]
impl<'a> WifiSniffer<'a> {
    pub fn new(wifi: WiFi<'a>) -> Self {
        WifiSniffer {
            wifi,
            enabled: false,
        }
    }

    pub fn set_callback(&mut self, callback: fn(PromiscuousPkt)) {
        self.wifi.set_sniffer_callback(callback);
    }

    pub fn init(&mut self) {
        self.wifi.init();
    }
}

impl App for WifiSniffer<'_> {
    fn enable(&mut self) {
        self.enabled = true;
        self.wifi.set_promiscuous_mode(true);
    }

    // fn run(&mut self) {}

    fn disable(&mut self) {
        self.wifi.set_promiscuous_mode(false);
        self.enabled = false;
    }

    fn is_enabled(&mut self) -> bool {
        self.enabled
    }
}
