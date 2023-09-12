use crate::generator::generic::AssemblyGenerator;

/// Represents a platform and the things it supports.
pub struct Platform<G: AssemblyGenerator> {
    /// A valid generator for assembly for this platform.
    pub assembly_generator: G,
    /// The width of pointers on this platform, in bytes.
    pub pointer_width: u8,
    /// The minimum memory size on this platform, in bytes.
    pub memory_size_minimum: u128,
    /// The maximum memory size on this platform, in bytes.
    pub memory_size_maximum: u128,
    /// A name for this platform your grandparents could use.
    pub friendly_name: &'static str,
    /// A name for this platform that makes it distinct from others.
    pub technical_name: &'static str,
    /// A list of features supported by this platform.
    pub features: Vec<&'static str>,
}
