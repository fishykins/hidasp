use crate::bindings::Bind;
use crate::DeviceMap;
use std::collections::{HashMap, HashSet};

/// A buffer map is essentially an inverted DeviceMap which indexes by buffer indices rather than action types.
/// This is not a good way to store data, but is very useful for quickly parsing device buffers.
pub type BufferMap = HashMap<u8, Vec<Bind>>;

/// This is not a particularly attractive function, but it does the job.
/// Run it once, cache the result, make money. 
pub fn build_buffer_map(device_map: &DeviceMap) -> BufferMap {
    let mut buffer_map: HashMap<u8, HashSet<Bind>> = HashMap::new();
    
    // lets just get right to it
    for (bind, input) in device_map.iter() {
        match input {
            crate::InputType::Button(button) => {
                if let Some(buf) = buffer_map.get_mut(&button.0) {
                    buf.insert(bind.clone());
                } else {
                    let mut hashset = HashSet::new();
                    hashset.insert(bind.clone());
                    buffer_map.insert(button.0, hashset);
                }
            }
            crate::InputType::Axis(axis) => {
                if let Some(buf) = buffer_map.get_mut(&axis.fine) {
                    buf.insert(bind.clone());
                } else {
                    let mut hashset = HashSet::new();
                    hashset.insert(bind.clone());
                    buffer_map.insert(axis.fine, hashset);
                }
                if let Some(buf) = buffer_map.get_mut(&axis.coarse) {
                    buf.insert(bind.clone());
                } else {
                    let mut hashset = HashSet::new();
                    hashset.insert(bind.clone());
                    buffer_map.insert(axis.coarse, hashset);
                }
            }
            crate::InputType::ButtonGroup(group) => {
                for (_, button) in group {
                    if let Some(buf) = buffer_map.get_mut(&button.0) {
                        buf.insert(bind.clone());
                    } else {
                        let mut hashset = HashSet::new();
                        hashset.insert(bind.clone());
                        buffer_map.insert(button.0, hashset);
                    }
                }
            }
            crate::InputType::AxisGroup(group) => {
                for (_, axis) in group {
                    if let Some(buf) = buffer_map.get_mut(&axis.fine) {
                        buf.insert(bind.clone());
                    } else {
                        let mut hashset = HashSet::new();
                        hashset.insert(bind.clone());
                        buffer_map.insert(axis.fine, hashset);
                    }
                    if let Some(buf) = buffer_map.get_mut(&axis.coarse) {
                        buf.insert(bind.clone());
                    } else {
                        let mut hashset = HashSet::new();
                        hashset.insert(bind.clone());
                        buffer_map.insert(axis.coarse, hashset);
                    }
                }
            }
        }
    }
    buffer_map.iter().map(|(buf, v)| (*buf, Vec::from_iter(v.clone()))).collect()
}

#[cfg(test)]
mod tests {
    use crate::loading::load_device;
    use super::*;

    #[test]
    fn test_load_device() {
        let device_map = load_device(8989, 512, "map_1").unwrap();
        let buffer_map = build_buffer_map(&device_map);
        assert_eq!(buffer_map.len(), 18);
    }
}
