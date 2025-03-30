use esp_wifi::wifi::PromiscuousPkt;

use crate::devices::wifi::WiFi;

use super::App;

pub struct Sniffer<'a> {
    wifi: &'a mut WiFi<'a>,
}

impl<'a> Sniffer<'a> {
    pub fn new(wifi: &'a mut WiFi<'a>) -> Self {
        Sniffer { wifi }
    }

    pub fn set_callback(&mut self, callback: fn(PromiscuousPkt)) {
        self.wifi.set_sniffer_callback(callback);
    }

    pub fn init(&mut self) {
        /*
        insert callback here
        */
        self.wifi.init();
    }
}

impl<'a> App for Sniffer<'a> {
    fn enable(&mut self) {
        let _ = self.wifi.set_promiscuous_mode(true);
    }

    // fn run(&mut self) {}

    fn disable(&mut self) {
        let _ = self.wifi.set_promiscuous_mode(false);
    }
}
