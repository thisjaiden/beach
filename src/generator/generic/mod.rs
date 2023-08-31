use crate::utils::*;

/// Represents a type capable of generating assembly for a platform from abstract concepts.
pub trait AssemblyGenerator {
    /// The width of pointers on this platform, in bytes.
    const POINTER_WIDTH: u8;
    /// The width of registers on this platform, in bytes.
    const REGISTER_WIDTH: u8;
    /// The minimum width of instructions on this platform, in bytes.
    const INSTRUCTION_WIDTH: u8;
    /// Assembly that sets the executing location to `label`.
    fn goto(label: Data) -> String;
    /// Assembly that calls a method located at `label`.
    fn call(label: Data) -> String;
    /// Assembly that stores `bytes` at a location `label`.
    fn data(label: String, bytes: &[u8]) -> String;
    /// Adds `value` to `to`, storing the result in `to`.
    fn add(value: Data, to: Data) -> String;
    /// Sets `location` equal to `value`.
    fn set(location: Data, value: Data) -> String;
    const EXTENSIONS: Vec<Extension>;
    const EXTENSION_PERFORMANCE_ORDER: Vec<Extension>;
    const EXTENSION_SIZE_ORDER: Vec<Extension>;
}

pub enum Extension {
    /// pushes [0] to a generic stack
    StackPush(Box<dyn Fn(Data) -> String>),
    /// pops [0] from a generic stack
    StackPop(Box<dyn Fn(Data) -> String>),
    /// goes to [0] if [1] equals zero
    GotoIfZero(Box<dyn Fn(Data, Data) -> String>),
    /// adds [0] to [1], storing the result in [2]
    AddStore(Box<dyn Fn(Data, Data, Data) -> String>),
}

pub enum Data {
    Register(String),
    Label(String),
    Data(Bigint)
}