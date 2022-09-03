pub mod bindings;
pub mod maps;
pub mod utils;
mod errors;

pub use errors::Error;
use maps::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

/// A unique identifier set by the manufacturer of a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct DeviceType {
    pub vendor_id: u16,
    pub product_id: u16,
}

/// The main data structure representation of a human-interface-device.
#[derive(Debug, Clone, Deserialize)]
pub struct HumanInterfaceDevice {
    pub name: String,
    pub device_type: DeviceType,
    pub map: HidMap,
}

impl PartialEq for HumanInterfaceDevice {
    fn eq(&self, other: &Self) -> bool {
        self.device_type == other.device_type
    }
}

impl HumanInterfaceDevice {
    /// Creates a new human interface device from a file, relative to the manifest directory.
    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let input_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let f = match File::open(&input_path) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(format!("failed to open stick config: {}", e)));
            }
        };

        let stick: Self = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                return Err(Error::new(format!("failed to parse stick config: {}", e)));
            }
        };
        Ok(stick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::*;

    #[test]
    fn test_from_file() {
        let input_path = "devices/nxt_gladiator.ron";
        let joystick = HumanInterfaceDevice::from_file(&input_path).unwrap();
        assert_eq!(joystick.device_type.product_id, 512);
        assert_eq!(joystick.device_type.vendor_id, 8989);
    }

    #[test]
    fn test_utils() {
        let input_path = "devices/nxt_gladiator.ron";
        let joystick = HumanInterfaceDevice::from_file(&input_path).unwrap();

        let used = get_used_buffer_indices(&joystick.map);
        assert_eq!(used, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 17, 18, 19, 33, 48, 49]);

        let inverted_map = build_buffer_map(&joystick.map);
        assert_eq!(used.len(), inverted_map.len());
        println!("{:?}", inverted_map)
    }
}
