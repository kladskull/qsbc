use std::collections::HashMap;

struct InstructionInfo {
    counter: u32,
    params_expected: u32,
}

fn load_instructions() -> HashMap<String, InstructionInfo> {
    // This data could also be loaded from a file
    let mut instructions = HashMap::new();
    instructions.insert("ADD".to_string(), InstructionInfo { counter: 1, params_expected: 0 });
    instructions.insert("SUB".to_string(), InstructionInfo { counter: 2, params_expected: 0 });
    instructions.insert("MUL".to_string(), InstructionInfo { counter: 3, params_expected: 0 });
    instructions.insert("DIV".to_string(), InstructionInfo { counter: 4, params_expected: 0 });
    instructions.insert("AND".to_string(), InstructionInfo { counter: 5, params_expected: 0 });
    instructions.insert("OR".to_string(), InstructionInfo { counter: 6, params_expected: 0 });
    instructions.insert("ORR".to_string(), InstructionInfo { counter: 7, params_expected: 0 });
    instructions.insert("EOR".to_string(), InstructionInfo { counter: 8, params_expected: 0 });
    instructions.insert("BIC".to_string(), InstructionInfo { counter: 9, params_expected: 0 });
    instructions.insert("CMP".to_string(), InstructionInfo { counter: 10, params_expected: 0 });
    instructions.insert("CMN".to_string(), InstructionInfo { counter: 11, params_expected: 0 });
    instructions.insert("TST".to_string(), InstructionInfo { counter: 12, params_expected: 0 });
    instructions.insert("TEQ".to_string(), InstructionInfo { counter: 13, params_expected: 0 });
    instructions.insert("MOV".to_string(), InstructionInfo { counter: 14, params_expected: 0 });
    instructions.insert("MVN".to_string(), InstructionInfo { counter: 15, params_expected: 0 });
    instructions.insert("B".to_string(), InstructionInfo { counter: 16, params_expected: 0 });
    instructions.insert("BL".to_string(), InstructionInfo { counter: 17, params_expected: 0 });
    instructions.insert("BX".to_string(), InstructionInfo { counter: 18, params_expected: 0 });
    instructions.insert("BLX".to_string(), InstructionInfo { counter: 19, params_expected: 0 });
    instructions.insert("BEQ".to_string(), InstructionInfo { counter: 20, params_expected: 0 });
    instructions.insert("BNE".to_string(), InstructionInfo { counter: 21, params_expected: 0 });
    instructions.insert("BGT".to_string(), InstructionInfo { counter: 22, params_expected: 0 });
    instructions.insert("BLT".to_string(), InstructionInfo { counter: 23, params_expected: 0 });
    instructions.insert("LDR".to_string(), InstructionInfo { counter: 24, params_expected: 0 });
    instructions.insert("LDRB".to_string(), InstructionInfo { counter: 25, params_expected: 0 });
    instructions.insert("LDRH".to_string(), InstructionInfo { counter: 26, params_expected: 0 });
    instructions.insert("STR".to_string(), InstructionInfo { counter: 27, params_expected: 0 });
    instructions.insert("STRB".to_string(), InstructionInfo { counter: 28, params_expected: 0 });
    instructions.insert("STRH".to_string(), InstructionInfo { counter: 29, params_expected: 0 });
    instructions.insert("SWI".to_string(), InstructionInfo { counter: 30, params_expected: 0 });
    instructions.insert("NOP".to_string(), InstructionInfo { counter: 31, params_expected: 0 });
    instructions.insert("LDM".to_string(), InstructionInfo { counter: 32, params_expected: 0 });
    instructions.insert("STM".to_string(), InstructionInfo { counter: 33, params_expected: 0 });
    instructions.insert("VADD".to_string(), InstructionInfo { counter: 34, params_expected: 0 });
    instructions.insert("VSUB".to_string(), InstructionInfo { counter: 35, params_expected: 0 });
    instructions.insert("VMUL".to_string(), InstructionInfo { counter: 36, params_expected: 0 });
    instructions.insert("VDIV".to_string(), InstructionInfo { counter: 37, params_expected: 0 });
    instructions.insert("VADD_I32".to_string(), InstructionInfo { counter: 38, params_expected: 0 });
    instructions.insert("VSUB_I32".to_string(), InstructionInfo { counter: 39, params_expected: 0 });
    instructions.insert("SVC".to_string(), InstructionInfo { counter: 40, params_expected: 0 });
    instructions.insert("BKPT".to_string(), InstructionInfo { counter: 41, params_expected: 0 });

    // Add other instructions
    instructions
}


