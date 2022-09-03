use crate::{
    bindings::BindingType,
    maps::{BufferMap, HidMap},
};
use std::collections::HashSet;

/// A simple function that runs through a map and returns all the buffer indices that are mapped.
/// This is useful for when you want to avoid iterating over a whole lot of noise!
pub fn get_used_buffer_indices(map: &HidMap) -> Vec<u8> {
    let mut used = HashSet::new();

    for (_, bm) in map.buttons.iter() {
        used.insert(bm.buf);
    }

    for (_, bg) in map.button_groups.iter() {
        for (_, bm) in bg.buttons.iter() {
            used.insert(bm.buf);
        }
    }

    for (_, ag) in map.analog_groups.iter() {
        for m in ag.as_vec() {
            used.insert(m.fine);
            used.insert(m.coarse);
        }
    }

    let mut sorted = used.into_iter().collect::<Vec<u8>>();
    sorted.sort();
    sorted
}

/// Builds an inverted hid_map, indexing by the buffer rather than by action.
/// This makes it easy to quickly sift through buffers and isolate important data changes.
pub fn build_buffer_map(hid_map: &HidMap) -> BufferMap {
    let mut buf_map = BufferMap::default();

    for (button_type, bm) in hid_map.buttons.iter() {
        let i = bm.buf;
        match buf_map.get(&i) {
            Some(v) => {
                let mut v = v.clone();
                v.push(BindingType::Button(*button_type));
                buf_map.insert(i, v);
            }
            None => {
                buf_map.insert(i, vec![BindingType::Button(*button_type)]);
            }
        }
    }

    for (group_type, bg) in hid_map.button_groups.iter() {
        for (button_type, bm) in bg.buttons.iter() {
            let i = bm.buf;
            match buf_map.get(&i) {
                Some(v) => {
                    let mut v = v.clone();
                    v.push(BindingType::ButtonGroup(*group_type, *button_type));
                    buf_map.insert(i, v);
                }
                None => {
                    buf_map.insert(i, vec![BindingType::ButtonGroup(*group_type, *button_type)]);
                }
            }
        }
    }

    for (axis_type, ag) in hid_map.analog_groups.iter() {
        for (i,m) in ag.as_vec().iter().enumerate() {
            let a = m.fine;
            let b = m.coarse;
            // seeing as axis tend to hog the entire index, we can just slap these values right on in.
            buf_map.insert(a, vec![BindingType::Axis(*axis_type, i as u8)]);
            buf_map.insert(b, vec![BindingType::Axis(*axis_type, i as u8)]);
        }
    }
    buf_map
}
