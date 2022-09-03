use serde::Deserialize;

use crate::bindings::*;
use std::collections::HashMap;

/// a pointer to an index in the device buffer.
pub type BufIndex = u8;

/// A collection of mapping values for a human-interface-device.
#[derive(Debug, Clone, Deserialize)]
pub struct HidMap {
    /// The button map.
    pub buttons: HashMap<ButtonType, ButtonMap>,
    pub button_groups: HashMap<ButtonGroupType, ButtonGroup>,
    pub analog_groups: HashMap<AxisType, AxisGroup>,
}

/// An inverted map of buffer indices that point to their corresponding actions.
pub type BufferMap = HashMap<u8, Vec<BindingType>>;

/// The mapping data for an axis within the device buffer.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AxisMap {
    /// Fine axis values between 0 and 255.
    pub fine: BufIndex,
    /// Cooarse axis value, also referred to as an octave.
    pub coarse: BufIndex,
    /// The total number of octaves this axis is mapped with. Typically, its either 8 or 16.
    pub octaves: u8,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ButtonMap {
    pub buf: BufIndex,
    pub query: Buttonquery,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Buttonquery {
    Bit(u8),
    Eq(u8),
}

/// Defines a collection of axis maps.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum AxisGroup {
    /// A single axis.
    Single(AxisMap),
    /// Two axis.
    Dual(AxisMap, AxisMap),
    /// Three axis. This is rare, but cool.
    Triple(AxisMap, AxisMap, AxisMap),
}

impl AxisGroup {
    pub fn as_vec(&self) -> Vec<&AxisMap> {
        match self {
            AxisGroup::Single(x) => vec![x],
            AxisGroup::Dual(x, y) => vec![x, y],
            AxisGroup::Triple(x, y, z) => vec![x, y, z],
        }
    }
}

/// Defines a collection of button maps.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ButtonGroup {
    /// Defines whether this group supports multiple button presses.
    pub mono: bool,
    /// The buttons within this group.
    pub buttons: HashMap<ButtonType, ButtonMap>,
}
