use crate::generator::generic::AssemblyGenerator;
use crate::generator::aarch64::AArch64AssemblyGenerator;
use super::generic::Platform;

pub fn get_this() -> Platform<impl AssemblyGenerator> {
    Platform {
        assembly_generator: AArch64AssemblyGenerator::new(),
        pointer_width: 8,
        memory_size_minimum: 1024 * 1024 * 1024 * 8,
        memory_size_maximum: 1024 * 1024 * 1024 * 192,
        friendly_name: "MacOS",
        technical_name: "AArch64-based MacOS",
        features: vec![
            (
                "stdout",
                "\
stdout:
mov x2, x1 // move arguments up one
mov x0, x1
mov X0, #1 // 1 = fd for stdout
mov X16, #4 // unix write system call
// x1 = ptr to str, x2 = len of str
svc #0 // Call kernel to output the string
ret"
            )
        ],
    }
}
