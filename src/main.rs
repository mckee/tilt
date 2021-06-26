extern crate confy;
extern crate rumble;

use log::LevelFilter;
use log::{debug, info};
use rumble::api::{Central, CentralEvent, Peripheral};
use rumble::bluez::manager::Manager;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

mod tilt;

#[derive(Serialize, Deserialize)]
struct Config {
    log_level: String,
    mttr_broker: String,
    mttr_topic: String,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
            mttr_broker: "mttr.local".into(),
            mttr_topic: "tilt".into(),
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: Config = confy::load("tilt")?;

    SimpleLogger::new()
        .with_level(LevelFilter::from_str(&cfg.log_level).unwrap())
        .init()
        .unwrap();
    info!("Using config:");
    info!("log_level: {}", cfg.log_level);
    info!("mttr_broker: {}", cfg.mttr_broker);
    info!("mttr_topic: {}", cfg.mttr_topic);

    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().next().unwrap();

    debug!("{:?}", adapter);

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();
    central.active(false);
    central.filter_duplicates(false);

    let clone = central.clone();
    central.on_event(Box::new(move |event| match event {
        CentralEvent::DeviceDiscovered(addr) => {
            let p = clone.peripheral(addr).unwrap().properties();
            debug!("found device {:?}", addr);
            tilt::send(p.manufacturer_data);
        }
        CentralEvent::DeviceUpdated(addr) => {
            let p = clone.peripheral(addr).unwrap().properties();
            debug!("updated device {:?}", addr);
            tilt::send(p.manufacturer_data);
        }
        _ => {
            debug!("{:?}", event);
        }
    }));

    central.start_scan().unwrap();
    loop {
        thread::sleep(Duration::from_secs(3));
    }
}
