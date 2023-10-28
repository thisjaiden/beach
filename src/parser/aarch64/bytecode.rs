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
            ins |= (offset & mask(28)) >> 2;
            return ins;
        }
        Instruction::MOVZ { destination, value, shift, half } => {
            let mut ins: u32 = 0;
            if half {
                // mark as half width register
                ins |= 0b1 << 31;
                assert!(
                    !(shift > 16) || !(shift % 16 == 0),
                    "Move and zero instruction (MOVZ) contained invalid shift."
                );
            }
            else {
                assert!(
                    !(shift % 16 == 0) || (shift > 48),
                    "Move and zero instruction (MOVZ) contained invalid shift."
                );
            }
            // opcode
            ins |= 0b10100101 << 23;
            // value shift
            ins |= ((shift / 16) as u32) << 21;
            // immediate value
            ins |= (value as u32) << 5;
            // destination register
            ins |= destination.to_5_bits() as u32;

            return ins;
        }
        _ => todo!()
    }
}
