#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum Instructions {
    ADD = 1,
    SUB = 2,
    MUL = 3,
    DIV = 4,
    AND = 5,
    OR = 6,
    ORR = 7,
    EOR = 8,
    BIC = 9,
    CMP = 10,
    CMN = 11,
    TST = 12,
    TEQ = 13,
    MOV = 14,
    MVN = 15,
    B = 16,
    BL = 17,
    BX = 18,
    BLX = 19,
    BEQ = 20,
    BNE = 21,
    BGT = 22,
    BLT = 23,
    LDR = 24,
    LDRB = 25,
    LDRH = 26,
    STR = 27,
    STRB = 28,
    STRH = 29,
    SWI = 30,    // system call
    LDM = 31,
    STM = 32,
    VADD = 33,
    VSUB = 34,
    VMUL = 35,
    VDIV = 36,
    VADD_I32 = 37,
    VSUB_I32 = 38,
    SVC = 39,
    BKPT = 40,
    NOP = 0,
}

// Implement `From<Instructions>` for `i32`
impl From<Instructions> for i32 {
    fn from(instr: Instructions) -> Self {
        instr as i32
    }
}
