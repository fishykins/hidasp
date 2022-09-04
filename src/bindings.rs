use serde::Deserialize;

/// The main identifier of an input type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct Bind(BindType, BindId);

/// Binding types are purely for the benefit of making mappings more human-readable.
/// Most systems should be fairly agnostic as to what it used, with the exception of
/// transocders that might be translating to other controller mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum BindType {
    /// Standard button, either numerical or alphabetical.
    Button,
    /// A trigger device, such as L and R on a gamepad. Could be a button, a button group or even an axis.
    Trigger,
    /// A function button (i.e. start, select, mode, etc).
    Function,
    /// A dual-axis thumbstick. Left and Right are standard on a gamepad.
    Thumbstick,
    /// A flightstick/joystick. Some may have twist action, but most will probably have dual axis. 
    Joystick,
    /// A rudder control.
    Rudder,
    /// A throttle axis device (plane goes zoom).
    Throttle,
    /// Pedals could refere to car pedals, a single-axis rudder pedal etc.
    Pedal,
    /// A four-way button pad, sometimes with a central push option.
    DPad,
    /// An eight-way selector hat, usually found on flight sticks.
    HatSwitch,
    /// Mouse wheel, scroll wheel, steering wheel, or axis wheel on a throttle.
    Wheel,
    /// A single-axis slider.
    Slider,
    /// A Potentiometer device, featuring a single axis (usually).
    Rotor,
    /// A misc bind for if you're feeling woke.
    Other,
}

/// Identifiers help differentiate between bindings of the same type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum BindId {
    /// Denotes the "main" device of this type.
    Main,

    // Directional identifiers
    Left,
    Right,
    Up,
    Down,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
    Center,

    // Stage based identifiers
    Half,
    Full,

    // Mode identifiers
    Start,
    Select,
    Mode,

    // Generic identifiers
    Id(u8),
    Axis(char),
}
