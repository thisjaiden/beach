use super::intermediate::*;

// TODO: move to utils
const fn mask(bits: u8) -> u32 {
    return u32::MAX >> (32 - bits);
}

pub fn convert_instruction(instruction: Instruction) -> u32 {
    match instruction {
        Instruction::B { offset } => {
            let mut ins: u32 = 0b0001_01 << 26;
            assert_eq!(
                offset,
                offset & mask(28),
                "Branch instruction (B) contained offset larger than 28 bits."
            );
            assert_eq!(
                offset,
                (offset >> 2) << 2,
                "Branch instruction (B) contained misaligned offset."
            );
            ins |= (offset & mask(26)) >> 2;
            return ins;
        }
        _ => todo!()
    }
}
