use super::generic;

struct AssemblyGenerator;

impl generic::AssemblyGenerator for AssemblyGenerator {
    const POINTER_WIDTH: u8 = 8;
    const REGISTER_WIDTH: u8 = 8;
    const INSTRUCTION_WIDTH: u8 = 4;

    fn goto(label: generic::Data) -> String {
        match label {
            generic::Data::Label(label_name) => {
                let mut output = String::new();
                output += &format!("mov x9, [{label_name}]\n");
                output += "br x9\n";
                return output;
            }
            generic::Data::Register(register_name) => {
                return format!("br {register_name}\n");
            }
            _ => todo!()
        }
    }
    fn call(label: generic::Data) -> String {
        match label {
            generic::Data::Label(label_name) => {
                let mut output = String::new();
                output += "stp x29, x30, [sp, #-16]!\n";
                output += "mov x29, sp\n";
                output += &format!("mov x9, [{label_name}]\n");
                output += "blr x9\n";
                output += "ldp x29, x30, [sp], #16\n";
                return output;
            }
            generic::Data::Register(register_name) => {
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

    fn add(value: generic::Data, to: generic::Data) -> String {
        todo!()
    }

    fn set(location: generic::Data, value: generic::Data) -> String {
        todo!()
    }

    const EXTENSIONS: Vec<generic::Extension> = vec![
        // TODO: extensions
    ];
}
