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

pub enum Register {
    /// General purpose registers with argument and return values conventions.
    X0, X1, X2, X3, X4, X5, X6, X7,
    // TODO X8-X18
    /// General purpose registers with preservation conventions.
    X19, X20, X21, X22, X23, X24, X25, X26, X27, X28,
    /// Frame pointer: the value of SP for the callee.
    X29,
    /// Current return address.
    X30,
    // TODO XZR
    /// {P}rogram {C}ounter
    /// 
    /// Points to the current executing location in memory.
    PC,
    /// {S}tack {P}ointer
    /// 
    /// Points to the current bottom of the stack.
    SP
}
