use std::fmt::Debug;

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

    // Iterate through each line of the program text.
    for line in program {
        // Skip empty or whitespace-only lines early to reduce nesting.
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        // Clean the line by removing specified unwanted characters and then tokenize it.
        // Splitting at '@', taking the part before it, trimming, and splitting into words.
        let tokens: Vec<String> = trimmed_line.chars()
            // Remove characters specified in `chars_to_remove`.
            .filter(|&c| !chars_to_remove.contains(&c))
            .collect::<String>()
            .splitn(2, '@')
            .next()
            .unwrap_or("")
            .trim()
            // Split the cleaned string into whitespace-separated tokens.
            .split_ascii_whitespace()
            .map(ToString::to_string) // Convert each token to a String for consistency.
            .collect();

        // At this point, `tokens` contains a vector of the cleaned and tokenized parts of the line before the '@' character.
        // Further processing can be done on `tokens` as needed.
        println!("{:#?}", tokens)
    }

    //vm.program.push(vec![Instructions::MOV.into()]); // Adds a new Vec<i32> containing 1, 2, 3
    //vm.program.push(vec![Instructions::MOV.into()]); // Adds a new Vec<i32> containing 1, 2, 3
}
