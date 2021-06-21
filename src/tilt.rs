use chrono::Utc;

pub const RED:    [u8; 16] = [0xa4, 0x95, 0xbb, 0x10, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];
pub const GREEN:  [u8; 16] = [0xa4, 0x95, 0xbb, 0x20, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];
pub const BLACK:  [u8; 16] = [0xa4, 0x95, 0xbb, 0x30, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];
pub const PURPLE: [u8; 16] = [0xa4, 0x95, 0xbb, 0x40, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];
pub const ORANGE: [u8; 16] = [0xa4, 0x95, 0xbb, 0x50, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];
pub const BLUE:   [u8; 16] = [0xa4, 0x95, 0xbb, 0x60, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];
pub const YELLOW: [u8; 16] = [0xa4, 0x95, 0xbb, 0x70, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];
pub const PINK:   [u8; 16] = [0xa4, 0x95, 0xbb, 0x80, 0xc5, 0xb1, 0x4b, 0x44, 0xb5, 0x12, 0x13, 0x70, 0xf0, 0x2d, 0x74, 0xde];

struct Beacon {
    color: String,
    temperature: u16,
    gravity: u16
}

fn color_by_uuid(uuid: [u8; 16]) -> Result<String, String> {
    match uuid {
        RED => Ok(String::from("red")),
        GREEN => Ok(String::from("green")),
        BLACK => Ok(String::from("black")),
        PURPLE => Ok(String::from("purple")),
        ORANGE => Ok(String::from("orange")),
        BLUE => Ok(String::from("blue")),
        YELLOW => Ok(String::from("yellow")),
        PINK => Ok(String::from("pink")),
        _ => Err(String::from("iBeacon uuid is not a tilt")),
    }
}

fn parse_beacon(v: Vec<u8>) -> Result<Beacon, String> {
    if v.len() == 25 {
        let mut mfg_id = [0; 4];
        mfg_id.copy_from_slice(&v[0..=3]);

        let mut uuid = [0; 16];
        uuid.copy_from_slice(&v[4..=19]);

        let mut temperature = [0; 2];
        temperature.copy_from_slice(&v[20..=21]);

        let mut gravity = [0; 2];
        gravity.copy_from_slice(&v[22..=23]);

        eprintln!("{:02x?}", v);
        if mfg_id == [0x4c, 0x00, 0x02, 0x15] {
            match color_by_uuid(uuid) {
                Ok(c) => {
                    Ok( Beacon {
                        color: c,
                        temperature: u16::from_be_bytes(temperature),
                        gravity: u16::from_be_bytes(gravity),
                    })
                }
                Err(e) => Err(format!("parse error: {}", e))
            }
        } else {
            Err(String::from("parse error: not iBeacon"))
        }
    } else {
        Err(String::from("parse error: ble announcement has wrong length"))
    }
}

pub fn log(v: Option<Vec<u8>>) {
    if let Some(v) = v {
        match parse_beacon(v) {
            Ok(tilt) => {
                let sg = f64::from(tilt.gravity) / 1000.0;
                println!("{} - {}: {}\u{b0}F, SG{:.3}", Utc::now(), tilt.color.to_uppercase(), tilt.temperature, sg);
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
