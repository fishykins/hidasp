use ron::de::from_reader;
use std::{
    fs::{read_dir, File, ReadDir},
    path::PathBuf,
};

use crate::DeviceMap;

/// Loads a config from the default hid_mappings.
pub fn load_device(vendor_id: u16, device_id: u16, mapping: &str) -> Option<DeviceMap> {
    let temp_vendor_path = format!("{}/hid_mappings/vendors", env!("CARGO_MANIFEST_DIR"));
    let vendors = read_dir(temp_vendor_path).expect("vendors directory not found");

    let vendor_path = match_file_uid(vendors, "vid", vendor_id);

    if vendor_path.is_none() {
        panic!("no vendor found");
    }

    let devices = read_dir(vendor_path.unwrap()).expect("vendors directory not found");

    if let Some(device_path) = match_file_uid(devices, "pid", device_id) {
        // We are in the device directory
        println!("{:?}", device_path);

        let files = read_dir(device_path).expect("device file not found");

        for file in files {
            let file = file.expect("failed to read file");
            if file.file_type().unwrap().is_file() {
                let file_string = file.file_name().into_string().unwrap();
                if let Some(extension) = file_string.split(".").last() {
                    if extension == "ron" {
                        let file_name = &file_string[0..file_string.len() - 4];
                        if file_name == mapping {
                            // done!
                            let f = File::open(&file.path()).expect("failed to open device map");
                            let cfg: DeviceMap =
                                from_reader(f).expect("failed to parse device map");
                            return Some(cfg);
                        }
                    }
                }
            }
        }
    }
    None
}

fn match_file_uid(root_dir: ReadDir, ext: &str, id: u16) -> Option<PathBuf> {
    for path in root_dir {
        let dir = path.expect("failed to open directory");
        let temp_path = dir.path();

        if temp_path.is_file() {
            continue;
        }

        let files =
            read_dir(&temp_path).expect(&format!("failed to read files in path: {:?}", temp_path));

        for file in files {
            let file = file.expect("failed to read file");
            if file.file_type().unwrap().is_file() {
                // get file name and extension
                let file_string = file.file_name().into_string().unwrap();
                if let Some(extension) = file_string.split(".").last() {
                    if extension == ext {
                        let file_name = &file_string[0..file_string.len() - 4];
                        if let Ok(file_id) = file_name.parse::<u16>() {
                            if file_id == id {
                                // This is the correct vendor!
                                return Some(temp_path);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_device() {
        load_device(8989, 512, "map_1").unwrap();
    }
}
