use core::{cell::RefCell, marker::PhantomData};

use esp_wifi::wifi::PromiscuousPkt;
use log::info;

use crate::devices::wifi::WiFi;

use super::{Async, AsyncApp, Sync, SyncApp};

pub struct WifiSniffer<'a, Kind = Sync> {
    wifi: RefCell<WiFi<'a>>,
    enabled: bool,
    Kind: PhantomData<Kind>,
}

impl<'a> WifiSniffer<'a> {
    pub fn new(wifi: RefCell<WiFi<'a>>) -> Self {
        WifiSniffer {
            wifi,
            enabled: true,
            Kind: Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.wifi.borrow_mut().set_sniffer_callback(|_packet| {
            let data = _packet.data;
            info!("{data:?}");
        });
    }
}

impl<'a> WifiSniffer<'a, Sync> {
    pub fn to_async(self) -> WifiSniffer<'a, Async> {
        WifiSniffer {
            wifi: self.wifi,
            enabled: self.enabled,
            Kind: PhantomData::<Async>,
        }
    }
}

impl<'a> SyncApp for WifiSniffer<'a, Sync> {
    fn enable(&mut self) {
        self.enabled = true;
        self.wifi.borrow_mut().set_promiscuous_mode(true);
    }

    fn disable(&mut self) {
        self.enabled = false;
        self.wifi.borrow_mut().set_promiscuous_mode(false);
    }

    fn is_enabled(&mut self) -> bool {
        self.enabled
    }
}

impl<'a> AsyncApp for WifiSniffer<'a, Async> {
    fn task(&mut self) {
        todo!()
    }
}

// #[allow(clippy::needless_lifetimes)]
// impl<'a> WifiSniffer<'a> {
//     pub fn new(wifi: RefCell<WiFi<'a>>) -> Self {
//         WifiSniffer {
//             wifi,
//             enabled: false,
//         }
//     }

//     pub fn set_callback(&mut self, callback: fn(PromiscuousPkt)) {
//         self.wifi.borrow_mut().set_sniffer_callback(callback);
//     }

//     pub fn init(&mut self) {
//         self.wifi.borrow_mut().init();
//     }
// }

// impl App for WifiSniffer<'_> {
//     fn enable(&mut self) {
//         self.enabled = true;
//         self.wifi.borrow_mut().set_promiscuous_mode(true);
//     }

//     // fn run(&mut self) {}

//     fn disable(&mut self) {
//         self.wifi.borrow_mut().set_promiscuous_mode(false);
//         self.enabled = false;
//     }

//     fn is_enabled(&mut self) -> bool {
//         self.enabled
//     }
// }
