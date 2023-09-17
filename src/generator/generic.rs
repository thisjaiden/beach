/// Represents a type capable of generating assembly for a platform from abstract concepts.
pub trait AssemblyGenerator {
    /// The width of pointers on this platform, in bytes.
    const POINTER_WIDTH: u8;
    /// The width of registers on this platform, in bytes.
    const REGISTER_WIDTH: u8;
    /// The minimum width of instructions on this platform, in bytes.
    const INSTRUCTION_WIDTH: u8;
    /// Assembly that sets the executing location to `label`.
    fn goto(label: HardwareData) -> String;
    /// Assembly that calls a method located at `label`.
    fn call(label: HardwareData) -> String;
    /// Assembly that exits a method
    fn endcall() -> String;
    /// Assembly that stores `bytes` at a location `label`.
    fn data(label: String, bytes: &[u8]) -> String;
    /// Adds `value` to `to`, storing the result in `to`.
    fn add(value: HardwareData, to: HardwareData) -> String;
    /// Sets `location` equal to `value`.
    fn set(location: HardwareData, value: HardwareData) -> String;
    /// Creates a fresh generator.
    fn new() -> Self;
    const EXTENSIONS: Vec<Extension>;
    const EXTENSION_PERFORMANCE_ORDER: Vec<Extension>;
    const EXTENSION_SIZE_ORDER: Vec<Extension>;
}

pub enum Extension {
    /// pushes [0] to a generic stack
    StackPush(Box<dyn Fn(HardwareData) -> String>),
    /// pops [0] from a generic stack
    StackPop(Box<dyn Fn(HardwareData) -> String>),
    /// goes to [0] if [1] equals zero
    GotoIfZero(Box<dyn Fn(HardwareData, HardwareData) -> String>),
    /// adds [0] to [1], storing the result in [2]
    AddStore(Box<dyn Fn(HardwareData, HardwareData, HardwareData) -> String>),
}

/// Represents a real, tangible piece of data.
pub enum HardwareData {
    // A valid hardware register for the platform that contains some data.
    ImmediateRegister(String),
    // A valid hardware register for the platform that points to some data.
    RefrenceRegister(String),
    // A valid label that points to some data.
    Label(String),
    // Raw immediate data.
    Immediate(Vec<u8>)
}
