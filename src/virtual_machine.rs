use crate::stack::Stack;
use bitvec::prelude::*;

/**
 # = immediate
 r0-r12 = registers
 [r1] [3] = memory address
 */

// Define a struct with some fields
pub struct VirtualMachine {
    pc: i32,
    // program counter
    sp: i32,
    // stack pointer
    flags: BitVec,
    // flags [negative,zero,carry,overflow]
    lr: i32,
    // link register / return address
    r0: i32,
    r1: i32,
    r2: i32,
    r3: i32,
    r4: i32,
    r5: f32,
    r6: f32,
    r7: f32,
    r8: f32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    memory: Vec<i32>,
    stack: Stack<i32>,
    pub(crate) program: Vec<Vec<i32>>,
}

impl VirtualMachine {
    // Public method to create a new instance of MyObject
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            sp: 0,
            flags: BitVec::new(),
            lr: 0,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0.0,
            r6: 0.0,
            r7: 0.0,
            r8: 0.0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            memory: Vec::new(),
            stack: Stack::new(),
            program: Vec::new(),
        }
    }
}
