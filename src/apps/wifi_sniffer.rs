use core::{cell::RefCell, marker::PhantomData};

use log::info;

use crate::devices::wifi::WiFi;

use super::{Async, AsyncApp, Sync, SyncApp};

pub struct WifiSniffer<'a, Kind = Sync> {
    wifi: RefCell<WiFi<'a>>,
    enabled: bool,
    kind: PhantomData<Kind>,
}

impl<'a> WifiSniffer<'a> {
    pub fn new(wifi: RefCell<WiFi<'a>>) -> Self {
        WifiSniffer {
            wifi,
            enabled: true,
            kind: Default::default(),
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
            kind: PhantomData::<Async>,
        }
    }
}

impl SyncApp for WifiSniffer<'_, Sync> {
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

impl AsyncApp for WifiSniffer<'_, Async> {
    fn task(&mut self) {
        todo!()
    }
}
