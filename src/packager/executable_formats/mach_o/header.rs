use crate::utils::*;
use std::convert::TryInto;

pub struct Header {
    // TODO: Universal (multi) binary support, has a unique magic number
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
    // TODO: switch to [FileType] enum
    file_type: u32,
    number_of_load_commands: u32,
    size_of_load_commands: u32,
    // TODO: proper flag enum
    flags: u32,
    /// Reserved field only present in 64 bit executables
    _reserved: Option<u32>,
}

impl Header {
    /// Generates a [Header] from a type implementing [std::io::Read]. Assumes platform endianness
    /// is little.
    pub fn from_le_reader<R: std::io::Read>(reader: &mut R) -> Result<Header, anyhow::Error> {
        // Read the header values
        let magic_number = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let cpu_type = CPUType::from_le_reader(reader)?;
        let cpu_subtype = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let file_type = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let number_of_load_commands = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let size_of_load_commands = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        let flags = u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap());
        // This value is only present if this is a 64-bit executable
        let _reserved = if magic_number == 0xFEEDFACF {
            Some(u32::from_le_bytes(read_n_bytes(reader, 4)?.try_into().unwrap()))
        }
        else {
            None
        };

        // Construct the Header type
        Ok(Header {
            magic_number, cpu_type, cpu_subtype,
            file_type, number_of_load_commands, size_of_load_commands,
            flags, _reserved
        })
    }
    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), anyhow::Error> {
        match self.cpu_type.endianness() {
            Endian::Little => {
                return self.write_le(writer);
            }
            Endian::Big => {
                return self.write_be(writer);
            }
            _ => todo!()
        }
    }
    fn write_le<W: std::io::Write>(&self, writer: &mut W) -> Result<(), anyhow::Error> {
        writer.write_all(&self.magic_number.to_le_bytes())?;
        writer.write_all(&(self.cpu_type as u32).to_le_bytes())?;
        writer.write_all(&self.cpu_subtype.to_le_bytes())?;
        writer.write_all(&self.file_type.to_le_bytes())?;
        writer.write_all(&self.number_of_load_commands.to_le_bytes())?;
        writer.write_all(&self.size_of_load_commands.to_le_bytes())?;
        writer.write_all(&self.flags.to_le_bytes())?;
        if self._reserved.is_some() {
            writer.write_all(&[0x00, 0x00, 0x00, 0x00])?;
        }
        writer.flush()?;
        Ok(())
    }
    fn write_be<W: std::io::Write>(&self, writer: &mut W) -> Result<(), anyhow::Error> {
        writer.write_all(&self.magic_number.to_be_bytes())?;
        writer.write_all(&(self.cpu_type as u32).to_be_bytes())?;
        writer.write_all(&self.cpu_subtype.to_be_bytes())?;
        writer.write_all(&self.file_type.to_be_bytes())?;
        writer.write_all(&self.number_of_load_commands.to_be_bytes())?;
        writer.write_all(&self.size_of_load_commands.to_be_bytes())?;
        writer.write_all(&self.flags.to_be_bytes())?;
        if self._reserved.is_some() {
            writer.write_all(&[0x00, 0x00, 0x00, 0x00])?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
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
    pub fn from_be_reader<R: std::io::Read>(reader: &mut R) -> Result<CPUType, anyhow::Error> {
        let bytes = read_n_bytes(reader, 4)?;
        return Ok(Self::from_be_bytes(bytes.try_into().unwrap()));
    }
    pub fn from_le_bytes(bytes: [u8; 4]) -> CPUType {
        CPUType::from_u32(u32::from_le_bytes(bytes))
    }
    pub fn from_be_bytes(bytes: [u8; 4]) -> CPUType {
        CPUType::from_u32(u32::from_be_bytes(bytes))
    }
    pub fn from_u32(value: u32) -> CPUType {
        match value {
            x if x == CPUType::VAX as u32 => CPUType::VAX,
            x if x == CPUType::ROMP as u32 => CPUType::ROMP,
            x if x == CPUType::NS32032 as u32 => CPUType::NS32032,
            x if x == CPUType::NS32332 as u32 => CPUType::NS32332,
            x if x == CPUType::MC680X0 as u32 => CPUType::MC680X0,
            x if x == CPUType::X86 as u32 => CPUType::X86,
            x if x == CPUType::X86ø64 as u32 => CPUType::X86ø64,
            x if x == CPUType::MIPS as u32 => CPUType::MIPS,
            x if x == CPUType::NS32352 as u32 => CPUType::NS32352,
            x if x == CPUType::MC98000 as u32 => CPUType::MC98000,
            x if x == CPUType::HPøPA as u32 => CPUType::HPøPA,
            x if x == CPUType::ARM as u32 => CPUType::ARM,
            x if x == CPUType::ARM64 as u32 => CPUType::ARM64,
            x if x == CPUType::MC88000 as u32 => CPUType::MC88000,
            x if x == CPUType::SPARC as u32 => CPUType::SPARC,
            x if x == CPUType::I860øBE as u32 => CPUType::SPARC,
            x if x == CPUType::I860øLE as u32 => CPUType::I860øLE,
            x if x == CPUType::RSø6000 as u32 => CPUType::RSø6000,
            x if x == CPUType::POWERPC as u32 => CPUType::POWERPC,
            _ => CPUType::Unrecognized
        }
    }
    pub fn endianness(&self) -> Endian {
        match self {
            CPUType::ARM => Endian::Either,
            CPUType::ARM64 => Endian::Little,
            CPUType::X86 => Endian::Little,
            CPUType::X86ø64 => Endian::Little,
            CPUType::I860øLE => Endian::Little,
            CPUType::I860øBE => Endian::Big,
            // TODO: rest of cpu endians
            _ => todo!()
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum FileType {
    RelocatableObjects = 0x00000001,
    DemandPagedExecutable = 0x00000002,
    FixedLibrary = 0x00000003,
    CoreFile = 0x00000004,
    PreloadedExecutable = 0x00000005,
    DynamicLibrary = 0x00000006,
    DynamicLinkEditor = 0x00000007,
    DynamicallyBoundBundle = 0x00000008,
    SharedLibraryStub = 0x00000009,
    DebugCompanion = 0x0000000A,
    X86ø64Kexts = 0x0000000B,
    MultiMachO = 0x0000000C,
    Unrecognized
}

enum Endian {
    Big,
    Little,
    Either
}
