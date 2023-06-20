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
    fn goto(label: String) -> String;
    /// Assembly that calls a method located at `label`.
    fn call(label: Data) -> String;
    /// Assembly that stores `bytes` at a location `label`.
    fn data(label: String, bytes: &[u8]) -> String;
    /// Adds `value` to `to`, storing the result in `to`.
    fn add(value: Data, to: Data) -> String;
    /// Sets `location` equal to `value`.
    fn set(location: Data, value: Data) -> String;
    const EXTENSIONS: Vec<Extension>;
}

pub enum Extension {
    StackPush(Box<dyn Fn(Data) -> String>),
    StackPop(Box<dyn Fn(Data) -> String>),
    GotoIfZero(Box<dyn Fn(Data, String) -> String>),
}

pub enum Data {
    Register(String),
    Label(String),
    Data(Bigint)
}