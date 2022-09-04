mod buffer_map;
mod bindings;
pub mod loading;
pub use bindings::*;
pub use buffer_map::*;
use serde::Deserialize;
use std::collections::HashMap;

pub type InputBuffer = [u8; 256];
pub type BufferPointer = u8;

/// This is where all data pertaining to a device is held.
pub type DeviceMap = HashMap<Bind, InputType>;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ButtonPointer(pub BufferPointer, pub ButtonQuery);

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Default)]
pub struct AxisPointer {
    /// The buffer index on which the fine value is stored. This combines the the coarse value to get the total axis value.
    pub fine: BufferPointer,
    /// The buffer on which the coarse value is stored. This is usually the one sequentially after the fine parameter.
    pub coarse: BufferPointer,
    /// The number of coarse 'octaves' in the buffer. Low-fidelity axis will have 4, while top-end devices can go all the way up to 256.
    pub octaves: u8,
    /// Inverting of an axis.
    pub inverted: bool,
    /// An absolute axis will only have values between 0 and 1 (once normalized).
    pub abs: bool,
}

/// Rules for collecting button data from input buffers.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum ButtonQuery {
    /// A bit query does an & opperation on the input buffer.
    Bit(u8),
    /// Eq requires the buffer to be an exact match to the provided value.
    Eq(u8),
}

/// Splits input into four components.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum InputType {
    /// A single button.
    Button(ButtonPointer),
    /// A single axis.
    Axis(AxisPointer),
    /// A collection of buttons.
    ButtonGroup(HashMap<BindId, ButtonPointer>),
    /// A collection of axes.
    AxisGroup(HashMap<BindId, AxisPointer>),
}
