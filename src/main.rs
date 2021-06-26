extern crate confy;
extern crate rumble;

use log::LevelFilter;
use log::{debug, info};
use rumble::api::{Central, CentralEvent, Peripheral};
use rumble::bluez::manager::Manager;
use rumqttc::{self, Client, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

mod tilt;

#[derive(Serialize, Deserialize)]
struct Config {
    log_level: String,
    mqtt_id: String,
    mqtt_broker: String,
    mqtt_port: u16,
    mqtt_topic: String,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
            mqtt_id: "tilt-publisher".into(),
            mqtt_broker: "mttr.local".into(),
            mqtt_port: 1883_u16,
            mqtt_topic: "tilt".into(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cfg: Config = confy::load("tilt")?;

    SimpleLogger::new()
        .with_level(LevelFilter::from_str(&cfg.log_level).unwrap())
        .init()
        .unwrap();
    info!("Using config:");
    info!("log_level: {}", cfg.log_level);
    info!("mqtt_id: {}", cfg.mqtt_id);
    info!("mqtt_broker: {}", cfg.mqtt_broker);
    info!("mqtt_port: {}", cfg.mqtt_port);
    info!("mqtt_topic: {}", cfg.mqtt_topic);

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

    // setup the mqtt publisher
    let mut mqttoptions = MqttOptions::new(cfg.mqtt_id, cfg.mqtt_broker, cfg.mqtt_port);
    mqttoptions.set_keep_alive(5);
    let (mqtt, mut connection) = Client::new(mqttoptions, 10);

    let clone = central.clone();
    let topic = cfg.mqtt_topic;
    central.on_event(Box::new(move |event| match event {
        CentralEvent::DeviceDiscovered(addr) => {
            let p = clone.peripheral(addr).unwrap().properties();
            debug!("found device {:?}", addr);
            let mut mqtt = mqtt.clone();
            let topic = topic.clone();
            match tilt::process(p.manufacturer_data) {
                Ok(j) => mqtt.publish(topic, QoS::AtLeastOnce, false, j).unwrap(),
                Err(e) => debug!("{}", e),
            }
        }
        CentralEvent::DeviceUpdated(addr) => {
            let p = clone.peripheral(addr).unwrap().properties();
            debug!("updated device {:?}", addr);
            let mut mqtt = mqtt.clone();
            let topic = topic.clone();
            match tilt::process(p.manufacturer_data) {
                Ok(j) => mqtt.publish(topic, QoS::AtLeastOnce, false, j).unwrap(),
                Err(e) => debug!("{}", e),
            }
        }
        _ => {
            debug!("{:?}", event);
        }
    }));

    central.start_scan().unwrap();
    loop {
        thread::sleep(Duration::from_secs(1));
        for notification in connection.iter() {
            debug!("MQTT = {:?}", notification);
        }
    }
}
