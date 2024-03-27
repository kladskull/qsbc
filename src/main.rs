use std::ptr::null;
use crate::instructions::Instructions;

mod virtual_machine;
mod stack;
mod instructions;

fn main() {
    println!("Hello, world!");

    let mut vm = virtual_machine::VirtualMachine::new();
    let mut program: Vec<String> = Vec::new();
    /**
    .global _start

    .section .text
    _start:
        MOV R0, #1      @ Initialize counter with 1

    loop:
        CMP R0, #10     @ Compare counter with 10
        ADD R0, R0, #1  @ Increment counter
        BLE loop        @ Loop if counter is less than or equal to 10

        @ Prepare to exit
        MOV R0, #0      @ Use 0 as the exit status
        MOV R7, #1      @ syscall number for sys_exit
        SWI 0           @ Make a system call to exit
     */

    program.push(String::from(".global _start"));
    program.push(String::from(".section .text"));
    program.push(String::from("loop:"));
    program.push(String::from("CMP R0, #10     @ Compare counter with 10"));
    program.push(String::from("ADD R0, R0, #1  @ Increment counter"));
    program.push(String::from("BLE loop        @ Loop if counter is less than or equal to 10"));
    program.push(String::from(""));
    program.push(String::from("@ Prepare to exit"));
    program.push(String::from("MOV R0, #0      @ Use 0 as the exit status"));
    program.push(String::from("MOV R7, #1      @ syscall number for sys_exit"));
    program.push(String::from("SWI 0           @ Make a system call to exit"));

    let chars_to_remove = [','];

    for line in program {

        // Remove the characters you don't want
        let cleaned_line: String = line.chars()
            .filter(|&c| !chars_to_remove.contains(&c))
            .collect();

        // Split at '@' and take the part before it, trimming any trailing whitespace
        let final_line = cleaned_line.splitn(2, '@').next().unwrap_or("").trim();

        // Check if the final line is not empty before printing
        if !final_line.is_empty() {
            println!("{}", final_line);
        }
    }

    vm.program.push(vec![Instructions::MOV.into()]); // Adds a new Vec<i32> containing 1, 2, 3


}
