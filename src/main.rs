extern crate confy;
extern crate rumble;

use rumble::api::{Central, CentralEvent, Peripheral};
use rumble::bluez::manager::Manager;
use serde::{Deserialize, Serialize};
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
            log_level: "warn".into(),
            mttr_broker: "mttr.local".into(),
            mttr_topic: "tilt".into(),
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: Config = confy::load("tilt")?;
    eprintln!("{:?}", cfg.log_level);

    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().next().unwrap();

    eprintln!("{:?}", adapter);

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
            eprintln!("found device {:?}", addr);
            tilt::log(p.manufacturer_data);
        }
        CentralEvent::DeviceUpdated(addr) => {
            let p = clone.peripheral(addr).unwrap().properties();
            eprintln!("updated device {:?}", addr);
            tilt::log(p.manufacturer_data);
        }
        _ => {
            eprintln!("{:?}", event);
        }
    }));

    central.start_scan().unwrap();
    loop {
        thread::sleep(Duration::from_secs(3));
    }
}
