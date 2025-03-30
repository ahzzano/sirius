use crate::devices::wifi::WiFi;

use super::App;

pub struct Sniffer<'a> {
    sniffer: &'a esp_wifi::wifi::Sniffer,
}

impl<'a> Sniffer<'a> {
    pub fn new(wifi: &'a WiFi) -> Self {
        Sniffer {
            sniffer: wifi.get_sniffer(),
        }
    }
}

impl<'a> App for Sniffer<'a> {
    fn enable(&mut self) {
        self.sniffer.set_promiscuous_mode(true);
    }

    fn run(&mut self) {}

    fn disable(&mut self) {
        self.sniffer.set_promiscuous_mode(false);
    }
}
