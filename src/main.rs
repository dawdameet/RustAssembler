use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Instruction {
    mnemonic: &'static str,
    opcode: u8,
    operand_size: usize,
}

fn assemble(assembly_code: &str) -> Vec<u8> {
    let mut machine_code = Vec::new();
    let instruction_set: HashMap<&str, Instruction> = vec![
        ("MOV", Instruction { mnemonic: "MOV", opcode: 0xB8, operand_size: 2 }),
        ("ADD", Instruction { mnemonic: "ADD", opcode: 0x01, operand_size: 2 }),
        ("SUB", Instruction { mnemonic: "SUB", opcode: 0x29, operand_size: 2 }),
        ("JMP", Instruction { mnemonic: "JMP", opcode: 0xEB, operand_size: 1 }),
    ]
    .into_iter()
    .collect();
    
    for line in assembly_code.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if let Some(instr) = instruction_set.get(tokens[0]) {
            machine_code.push(instr.opcode);
            if instr.operand_size > 1 {
                machine_code.extend(vec![0x00; instr.operand_size - 1]); // Placeholder for operand
            }
        }
    }
    machine_code
}

fn disassemble(machine_code: &[u8]) -> String {
    let mut assembly_code = String::new();
    let opcode_map: HashMap<u8, &str> = vec![
        (0xB8, "MOV"),
        (0x01, "ADD"),
        (0x29, "SUB"),
        (0xEB, "JMP"),
    ]
    .into_iter()
    .collect();

    let mut i = 0;
    while i < machine_code.len() {
        if let Some(&mnemonic) = opcode_map.get(&machine_code[i]) {
            assembly_code.push_str(mnemonic);
            assembly_code.push('\n');
        }
        i += 1;
    }
    assembly_code
}

fn main() {
    let assembly_code = "MOV\nADD\nSUB\nJMP";
    let machine_code = assemble(assembly_code);
    println!("Generated Machine Code: {:?}", machine_code);
    
    let disassembled_code = disassemble(&machine_code);
    println!("Disassembled Code:\n{}", disassembled_code);
}
