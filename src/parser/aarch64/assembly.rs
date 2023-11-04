use super::intermediate::*;

pub fn line_to_intermediate(line: &str) -> Instruction {
    let cleaned = line.trim();
    if cleaned.ends_with(":") {
        // TODO: label
    }
    let (opcode, args) = cleaned
        .split_once(" ")
        .unwrap_or_else(|| {return (cleaned, "")});

    match opcode.to_lowercase().as_ref() {
        "b" => {
            if args.starts_with("#") {
                if let Ok(mut val) = args.trim_start_matches("#").parse::<i32>() {
                    if val.is_negative() {
                        // move the sign bit to the proper ending
                        val *= -1;
                        val |= 0b1 << 27;
                    }
                    return Instruction::B { offset: val as u32 };

                }
                else {
                    panic!("Unable to parse numeric value in B opcode!");
                }
            }
            else {
                // TODO: probably a label
                todo!()
            }
        }
        _ => todo!()
    }
}

#[test]
fn test_branch_instructions() {
    assert_eq!(
        super::bytecode::convert_instruction(
            line_to_intermediate("b #16020")
        ),
        0x14000FA5
    );
    assert_eq!(
        super::bytecode::convert_instruction(
            // TODO: This fails bc HEX SCARRY AHAHAGHHAGHA
            line_to_intermediate("b #0x3e94")
        ),
        0x14000FA5
    );
    assert_eq!(
        super::bytecode::convert_instruction(
            line_to_intermediate("b #20")
        ),
        0x14000005
    );
}