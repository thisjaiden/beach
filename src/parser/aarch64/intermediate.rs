#[derive(Debug)]
pub enum Instruction {
    /// ### {B}ranch to a signed offset.
    /// 
    /// ## Encoding
    /// - `b #[offset]`
    /// - `b label`
    /// 
    /// ## Effect
    /// PC = PC + offset
    /// 
    /// ## Notes
    /// Only 28 bits of the offset are used. All other bits are ignored. Neither
    /// of the lowest two bits should be set to align the result with
    /// instruction borders.
    /// 
    /// If using a label instead of an offset, the compiler will calculate the
    /// relative offset of the label. If out of range, an error will be thrown.
    /// 
    /// This hints to the processor that you are not jumping to a subroutine and
    /// not returning from one either. Use other instructions for those cases to
    /// avoid predictive misses.
    B { offset: u32 },
    /// ### {B}ranch to a signed offset on a condition {XX}.
    /// 
    /// ## Encoding
    /// - `b.[condition] #[offset]`
    /// - `b.[condition] label`
    /// 
    /// ## Effect
    /// if (condition) {
    ///     PC = PC + offset
    /// }
    /// 
    /// ## Notes
    /// Only 21 bits of the offset are used. All other bits are ignored. Neither
    /// of the lowest two bits for the offset should be set to align the result
    /// with instruction borders.
    /// 
    /// If using a label instead of an offset, the compiler will calculate the
    /// relative offset of the label. If out of range, an error will be thrown.
    /// 
    /// This hints to the processor that you are not jumping to a subroutine and
    /// not returning from one either. Use other instructions for those cases to
    /// avoid predictive misses.
    BDotXX { condition: Condition, offset: u32 },
    /// ### {M}{O}{V}es the value of a register to another register.
    /// 
    /// ## Encoding
    /// - `mov [destination], [source]`
    /// 
    /// ## Effect
    /// destination = source
    MOV { destination: Register, source: Register },
    /// ### {M}{O}{V}es a 16 bit immediate value into a register after {Z}eroing
    /// said register. Optionally shifts the value.
    /// 
    /// ## Encoding
    /// - `movz [destination], #[value]`
    /// - `movz [destination], #[value], lsl #[shift]`
    /// 
    /// If the instruction does not have a shift, it is sometimes represented as
    /// - `mov [destination], #[value]`
    /// 
    /// instead. This is only perfered if [value] is not 0.
    /// 
    /// ## Effect
    /// destination = value << shift
    /// 
    /// ## Notes
    /// All other shift sizes besides 16, 32, or 48 bits are invalid. Setting
    /// `half` to true makes this operate on the destination register as a 32
    /// bit register.
    MOVZ { destination: Register, value: u16, shift: u8, half: bool },
}

#[derive(Debug)]
pub enum Condition {
    /// ### {E}{Q}ual
    /// 
    /// true if the zero flag (z) is set.
    /// 
    /// After a `cmp` or `subs` instruction this condition is true if the values
    /// were equal.
    EQ,
    /// ### {N}ot {E}qual
    /// 
    /// true if the zero flag (z) is not set.
    /// 
    /// After a `cmp` or `subs` instruction this condition is true if the values
    /// were not equal.
    NE,
    // TODO: doc the rest!
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    /// ### Signed {G}reater {Than}
    /// 
    /// true if the zero flag (z) is set, **and** the signed carry flag (v) is
    /// equal to the negative flag (n).
    /// 
    /// After a `cmp` or `subs` instruction this ondition is true if the first
    /// value was greater than the second value, given both are handled as
    /// signed values.
    GT,
    /// ### Signed {L}ess than or {E}qual
    /// 
    /// true if the zero flag (z) is set, **or** the signed carry flag (v) is
    /// not in the same state as the negative flag (n).
    /// 
    /// After a `cmp` or `subs` instruction this condition is true if first
    /// value was less than or equal to the second value, given both are handled
    /// as signed values.
    LE,
    /// ### {A}{L}ways
    /// 
    /// always true. Not specifiying a flag is the same thing as specifying this
    /// flag.
    AL,
    /// ### {N}o idea what this stands {V}or.
    /// 
    /// always true. Not specifiying a flag is the same thing as specifying this
    /// flag. Perfer [Condition::AL]. This flag only exists to fill out the
    /// binary space, as stated by the ARM refrence manual.
    NV
}

impl Condition {
    /// Used for bytecode generation. Only the low 4 bits are used.
    pub fn to_bits(&self) -> u8 {
        match self {
            Self::EQ => 0b0000,
            Self::NE => 0b0001,
            Self::CS => 0b0010,
            Self::CC => 0b0011,
            Self::MI => 0b0100,
            Self::PL => 0b0101,
            Self::VS => 0b0110,
            Self::VC => 0b0111,
            Self::HI => 0b1000,
            Self::LS => 0b1001,
            Self::GE => 0b1010,
            Self::LT => 0b1011,
            Self::GT => 0b1100,
            Self::LE => 0b1101,
            Self::AL => 0b1110,
            Self::NV => 0b1111
        }
    }
}

#[derive(Debug)]
pub enum Register {
    /// General purpose registers with argument and return values conventions.
    R0, R1, R2, R3, R4, R5, R6, R7,
    /// Indirect return value address. (??)
    R8,
    /// General purpose registers that must be preserved by the caller, if used.
    R9, R10, R11, R12, R13, R14, R15,
    /// Intra-procedure scratch registers.
    R16, R17,
    /// Platform defined.
    R18,
    /// General purpose registers that must be preserved by the callee, if used.
    R19, R20, R21, R22, R23, R24, R25, R26, R27, R28,
    /// Frame pointer: the value of SP for the callee.
    R29,
    /// Current return address.
    R30,
    /// {Z}ero {R}egister
    /// 
    /// Acts as the value 0. Not defined as a real register.
    ZR,
    /// {P}rogram {C}ounter
    /// 
    /// Points to the current executing location in memory.
    PC,
    /// {S}tack {P}ointer
    /// 
    /// Points to the current bottom of the stack.
    SP
}

impl Register {
    /// Used for bytecode generation. Only the low 5 bits are used.
    pub fn to_5_bits(&self) -> u8 {
        // I know this sucks. Oh well! It's Fast Enough:tm: and not so horrible
        // I'd bother with a macro or library.
        match self {
            Self::R0  => 0b00000,
            Self::R1  => 0b00001,
            Self::R2  => 0b00010,
            Self::R3  => 0b00011,
            Self::R4  => 0b00100,
            Self::R5  => 0b00101,
            Self::R6  => 0b00110,
            Self::R7  => 0b00111,
            Self::R8  => 0b01000,
            Self::R9  => 0b01001,
            Self::R10 => 0b01010,
            Self::R11 => 0b01011,
            Self::R12 => 0b01100,
            Self::R13 => 0b01101,
            Self::R14 => 0b01110,
            Self::R15 => 0b01111,
            Self::R16 => 0b10000,
            Self::R17 => 0b10001,
            Self::R18 => 0b10010,
            Self::R19 => 0b10011,
            Self::R20 => 0b10100,
            Self::R21 => 0b10101,
            Self::R22 => 0b10110,
            Self::R23 => 0b10111,
            Self::R24 => 0b11000,
            Self::R25 => 0b11001,
            Self::R26 => 0b11010,
            Self::R27 => 0b11011,
            Self::R28 => 0b11100,
            Self::R29 => 0b11101,
            Self::R30 => 0b11110,
            Self::ZR  => 0b11111,
            _ => panic!("This register is not valid in a 5 bit encoding context!")
        }
    }
}
