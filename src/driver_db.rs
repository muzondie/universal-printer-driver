use super::*;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref DRIVER_MAP: HashMap<(u16, u16), PrinterProfile> = {
        let mut m = HashMap::new();
        m.insert((0x03F0, 0x2A4A), PrinterProfile {
            vendor_id: 0x03F0,
            product_id: 0x2A4A,
            driver_path: "hp/universal_v5".to_string(),
            checksum: [0; 32],
        });
        m
    };
}

pub fn find_driver(vendor_id: u16, product_id: u16) -> Option<&'static PrinterProfile> {
    DRIVER_MAP.get(&(vendor_id, product_id))
}