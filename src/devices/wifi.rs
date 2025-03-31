use esp_hal::peripherals;
use esp_wifi::{
    wifi::{Interfaces, PromiscuousPkt, Sniffer, WifiController},
    EspWifiController,
};

use super::Device;

pub struct WiFi<'a> {
    device: WifiController<'a>,
    _interface: Interfaces<'a>,
    sniffer: Sniffer,
}

impl<'a> WiFi<'a> {
    pub fn new(wifi_controller: &'a EspWifiController, device: peripherals::WIFI) -> Self {
        let (device, interface) = esp_wifi::wifi::new(wifi_controller, device).unwrap();
        let sniffer = device.take_sniffer().unwrap();

        WiFi {
            device,
            _interface: interface,
            sniffer,
        }
    }

    pub fn set_sniffer_callback(&mut self, callback: fn(PromiscuousPkt)) {
        self.sniffer.set_receive_cb(callback);
    }

    pub fn set_promiscuous_mode(&mut self, enabled: bool) {
        self.sniffer.set_promiscuous_mode(enabled).unwrap();
    }

    pub fn init(&mut self) {
        let _ = self.device.start();
    }
}

impl<'a> Device for WiFi<'a> {}
