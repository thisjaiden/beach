enum Instruction {
    /// {B}ranch to an offset.
    /// ## Effect
    /// PC = PC + offset
    /// 
    /// Only 28 bits of the offset are used. All other bits are ignored.
    B { offset: u32 },
    /// {B}ranch to an offset on a condition {XX}.
    /// Acutal instructions may look like:
    /// 
    /// `b.eq #offset`
    /// ## Effect
    /// if (condition) {
    ///     PC = PC + offset
    /// }
    /// 
    /// Only 21 bits of the offset are used. All other bits are ignored.
    BDotXX { condition: Condition, offset: u32 }

}

enum Condition {
    EQ,
    NE,
    // TODO: the rest of the fucking owl
}

enum Register {
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