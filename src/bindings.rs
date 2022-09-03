use serde::Deserialize;

/// Used to represent just about any action on a device. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum BindingType {
    /// Points to a specific button action.
    Button(ButtonType),
    /// Points to a button within a button group.
    ButtonGroup(ButtonGroupType, ButtonType),
    /// Represents an axis and the sub-axis index.
    Axis(AxisType, u8),
}

/// A nice collection of axis types that can be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum AxisType {
    // The standard directional axis are provided here, but should be discouraged from being used.
    // The rationale is that if you have one, you probably have at least one other, so you should be using a device group instead.
    // I cant stop you using them, but I will be very sad if you do.
    X,
    Y,
    Z,
    RZ,
    // Device groups are far more intuitive to use and make it very clear how devices are laid out.
    // These are the more conventional groups you will encounter.
    LeftStick,
    RightStick,
    LeftTrigger,
    RightTrigger,
    FlightStick,
    FLightStickTwist,
    Wheel,
    // Misc identifiers incase you have a really crazy device on your hands.
    Throttle(u8),
    Stick(u8),
    Trigger(u8),
    Slider(u8),
    Rotor(u8),
    Other(u8),
}

/// Spesific button actions that are used in groups. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum ButtonType {
    Button(u8),
    Trigger(u8),
    Function(u8),
    // D-pad/ hat switch types. use these in a group rather than 
    // as a button- that way you can use them multiple times!
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Press,

    // Triggers
    Fire,
    HalfDepress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum ButtonGroupType {
    DualTrigger,
    DPad(u8),
    HatSwitch(u8),
}
