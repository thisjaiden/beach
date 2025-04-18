use super::generic::*;

pub struct AArch64AssemblyGenerator;

impl AssemblyGenerator for AArch64AssemblyGenerator {
    const POINTER_WIDTH: u8 = 8;
    const REGISTER_WIDTH: u8 = 8;
    const INSTRUCTION_WIDTH: u8 = 4;

    fn label(label: String) -> String {
        format!("{label}:\n")
    }
    fn goto(label: HardwareData) -> String {
        match label {
            HardwareData::Label(label_name) => {
                let mut output = String::new();
                output += &format!("mov x9, [{label_name}]\n");
                output += "br x9\n";
                return output;
            }
            HardwareData::ImmediateRegister(register_name) => {
                return format!("br {register_name}\n");
            }
            _ => todo!()
        }
    }
    fn call(label: HardwareData) -> String {
        match label {
            HardwareData::Label(label_name) => {
                let mut output = String::new();
                output += "stp x29, x30, [sp, #-16]!\n";
                output += "mov x29, sp\n";
                output += &format!("adr x9, {label_name}\n");
                output += "blr x9\n";
                output += "ldp x29, x30, [sp], #16\n";
                return output;
            }
            HardwareData::ImmediateRegister(register_name) => {
                let mut output = String::new();
                output += "stp x29, x30, [sp, #-16]!\n";
                output += "mov x29, sp\n";
                output += &format!("blr {register_name}\n");
                output += "ldp x29, x30, [sp], #16\n";
                return output;
            }
            _ => todo!()
        }
    }
    fn endcall() -> String {
        String::from("ret\n")
    }
    fn data(label: String, bytes: &[u8]) -> String {
        let mut output = String::new();
        output += &format!("{label}:\n");
        output += ".byte ";
        for byte in bytes {
            output += &format!("0x{:X}, ", *byte);
        }
        output.pop();
        output.pop();
        output += "\n";
        // realigned for instructions
        output += ".align 2\n";
        return output;
    }

    fn push(data: HardwareData) -> String {
        match data {
            HardwareData::ImmediateRegister(reg) => {
                // TODO: this is inefficent (uses 16 bytes for 8 byte registers)
                return format!("str {reg}, [sp, #-16]!\n");
            }
            _ => todo!()
        }
    }

    fn pop(amount: usize, location: HardwareData) -> String {
        todo!()
    }

    fn add(value: HardwareData, to: HardwareData) -> String {
        todo!()
    }

    fn set(location: HardwareData, value: HardwareData) -> String {
        match location {
            HardwareData::ImmediateRegister(reg) => {
                match value {
                    HardwareData::ImmediateRegister(reg2) => {
                        return format!("mov {reg}, {reg2}\n");
                    }
                    HardwareData::Label(label) => {
                        return format!("adr {reg}, {label}\n");
                    }
                    HardwareData::Immediate(imm) => {
                        if imm.len() == 1 {
                            return format!("movz {reg}, #{}\n", imm[0]);
                        }
                        todo!();
                    }
                    unfinished => todo!("val: {:?}", unfinished)
                }
            }
            _ => todo!()
        }
    }
    
    fn new() -> Self {
        Self {}
    }

    // ARM conventions and C, doesn't work with C++ which passes func addr in x0
    const ARGUMENT_REGISTERS: &'static [&'static str] = &[
        "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"
    ];
    const EXTENSIONS: Vec<Extension> = vec![
        // TODO: extensions
    ];
    const EXTENSION_PERFORMANCE_ORDER: Vec<Extension> = vec![
        // TODO: ext perf order
    ];
    const EXTENSION_SIZE_ORDER: Vec<Extension> = vec![
        // TODO: ext size order
    ];
}
