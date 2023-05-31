use crate::utils::*;

pub struct Header {
    /// The magic number for this header. Expected values:
    /// 0xfeedface - 32 bit executable
    /// 0xfeedfacf - 64 bit executable
    /// Note that the magic number is represented in platform endianness.
    magic_number: u32,
    /// The expected processor type for this executable.
    /// Defines how `cpu_subtype` should be interpreted.
    cpu_type: CPUType,
    // TODO: enum over this for better types
    cpu_subtype: u32,
    file_type: u32,
    number_of_load_commands: u32,
    size_of_load_commands: u32,
    flags: u32,
    /// Reserved field only present in 64 bit executables
    _reserved: Option<u32>,
}

use std::convert::TryInto;

impl Header {
    /// Generates a [Header] from a type implementing [std::io::Read]. Assumes platform endinanness
    /// is little.
    pub fn from_le_reader<R: std::io::Read>(reader: &mut R) -> Result<Header, anyhow::Error> {
        let magic_number = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let cpu_type = CPUType::from_le_reader(reader)?;
        let cpu_subtype = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let file_type = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let number_of_load_commands = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let size_of_load_commands = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let flags = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let _reserved = if magic_number == 0xFEEDFACF {
            Some(u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap()))
        }
        else {
            None
        };
        Ok(Header {
            magic_number, cpu_type, cpu_subtype,
            file_type, number_of_load_commands, size_of_load_commands,
            flags, _reserved
        })
    }
}

pub enum CPUType {
    VAX = 0x00000001,
    ROMP = 0x00000002,
    NS32032 = 0x00000004,
    NS32332 = 0x00000005,
    MC680X0 = 0x00000006,
    X86 = 0x00000007,
    X86ø64 = 0x01000007,
    MIPS = 0x00000008,
    NS32352 = 0x00000009,
    MC98000 = 0x0000000A,
    HPøPA = 0x0000000B,
    ARM = 0x0000000C,
    ARM64 = 0x0100000C,
    MC88000 = 0x0000000D,
    SPARC = 0x0000000E,
    I860øBE = 0x0000000F,
    I860øLE = 0x00000010,
    RSø6000 = 0x00000011,
    POWERPC = 0x00000012,
    /// Catch-all for values not recognized by beach
    Unrecognized
}

impl CPUType {
    pub fn from_le_reader<R: std::io::Read>(reader: &mut R) -> Result<CPUType, anyhow::Error> {
        let bytes = read_n_bytes(reader, 4)?;
        return Ok(Self::from_le_bytes(bytes.try_into().unwrap()));
    }
    pub fn from_u32(value: u32) -> CPUType {
        match value {
            x if x == CPUType::VAX as u32 => CPUType::VAX,
            x if x == CPUType::ROMP as u32 => CPUType::ROMP,
            x if x == CPUType::NS32032 as u32 => CPUType::NS32032,
            // TODO: other CPU types
            x if x == CPUType::ARM as u32 => CPUType::ARM,
            x if x == CPUType::ARM64 as u32 => CPUType::ARM64,
            _ => CPUType::Unrecognized
        }

    }
    pub fn from_le_bytes(bytes: [u8; 4]) -> CPUType {
        CPUType::from_u32(u32::from_le_bytes(bytes))
    }
}
