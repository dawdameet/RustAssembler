use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Instruction {
    mnemonic: &'static str,
    opcode: u8,
}

fn assemble(assembly_code: &str) -> Vec<u8> {
    let mut machine_code = Vec::new();
    let instruction_set: HashMap<&str, u8> = vec![
        ("MOV", 0xB8),
        ("ADD", 0x01),
        ("SUB", 0x29),
        ("JMP", 0xEB),
    ]
    .into_iter()
    .collect();
    
    for line in assembly_code.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if let Some(&opcode) = instruction_set.get(tokens[0]) {
            machine_code.push(opcode);
        }
    }
    machine_code
}

fn main() {
    let assembly_code = "MOV\nADD\nSUB\nJMP";
    let machine_code = assemble(assembly_code);
    println!("Generated Machine Code: {:?}", machine_code);
}
