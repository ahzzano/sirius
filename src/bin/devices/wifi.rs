use esp_hal::peripherals;
use esp_wifi::{
    wifi::{Interfaces, Sniffer, WifiController},
    EspWifiController,
};

use super::Device;

pub struct WiFi<'a> {
    device: WifiController<'a>,
    interface: Interfaces<'a>,
    sniffer: Sniffer,
}

impl<'a> WiFi<'a> {
    pub fn new(wifi_controller: &'a EspWifiController, device: peripherals::WIFI) -> Self {
        let (device, interface) = esp_wifi::wifi::new(wifi_controller, device).unwrap();
        let sniffer = device.take_sniffer().unwrap();

        WiFi {
            device,
            interface,
            sniffer,
        }
    }

    pub fn get_sniffer(&self) -> &Sniffer {
        &self.sniffer
    }

    pub fn init() {}
}

impl<'a> Device for WiFi<'a> {}
