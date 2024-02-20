use std::fs::File;
use std::io::{BufRead, BufReader};

// 命令コード
const MOV: u16 = 0b0000;
const ADD: u16 = 0b0001;
const SUB: u16 = 0b0010;
const AND: u16 = 0b0011;
const OR: u16 = 0b0100;
const SL: u16 = 0b0101;
const SR: u16 = 0b0110;
const SRA: u16 = 0b0111;
const LDL: u16 = 0b1000;
const LDH: u16 = 0b1001;
const CMP: u16 = 0b1010;
const JE: u16 = 0b1011;
const JMP: u16 = 0b1100;
const LD: u16 = 0b1101;
const ST: u16 = 0b1110;
const HLT: u16 = 0b1111;

// 汎用レジスタを指定するオペランド
const REG0: u16 = 0b000;
const REG1: u16 = 0b001;
const REG2: u16 = 0b010;
const REG3: u16 = 0b011;
const REG4: u16 = 0b100;
const REG5: u16 = 0b101;
const REG6: u16 = 0b110;
const REG7: u16 = 0b111;

pub fn assembler(file_path: String) -> Vec<u16> {
    let mut program: Vec<u16> = Vec::new();

    let file = File::open(file_path).unwrap();
    let buffer = BufReader::new(file);

    for (index, line) in buffer.lines().enumerate() {
        let index = index + 1;
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<&str>>();

        match line[0] {
            "mov" => {
                let r1 = get_register(index, line[1]);
                let r2 = get_register(index, line[2]);
                program.push(mov(r1, r2));
            }

            "add" => {
                let r1 = get_register(index, line[1]);
                let r2 = get_register(index, line[2]);
                program.push(add(r1, r2));
            }

            "sub" => {
                let r1 = get_register(index, line[1]);
                let r2 = get_register(index, line[2]);
                program.push(sub(r1, r2));
            }

            "and" => {
                let r1 = get_register(index, line[1]);
                let r2 = get_register(index, line[2]);
                program.push(and(r1, r2));
            }

            "or" => {
                let r1 = get_register(index, line[1]);
                let r2 = get_register(index, line[2]);
                program.push(or(r1, r2));
            }

            "sl" => {
                let r1 = get_register(index, line[1]);
                program.push(sl(r1));
            }

            "sr" => {
                let r1 = get_register(index, line[1]);
                program.push(sr(r1));
            }

            "sra" => {
                let r1 = get_register(index, line[1]);
                program.push(sra(r1));
            }

            "ldl" => {
                let r1 = get_register(index, line[1]);
                let data = get_data(index, line[2]);
                program.push(ldl(r1, data));
            }

            "ldh" => {
                let r1 = get_register(index, line[1]);
                let data = get_data(index, line[2]);
                program.push(ldh(r1, data));
            }

            "cmp" => {
                let r1 = get_register(index, line[1]);
                let r2 = get_register(index, line[2]);
                program.push(cmp(r1, r2));
            }

            "je" => {
                let addr = get_addr(index, line[1]);
                program.push(je(addr));
            }

            "jmp" => {
                let addr = get_addr(index, line[1]);
                program.push(jmp(addr));
            }

            "ld" => {
                let r1 = get_register(index, line[1]);
                let addr = get_addr(index, line[2]);
                program.push(ld(r1, addr));
            }

            "st" => {
                let r1 = get_register(index, line[1]);
                let addr = get_addr(index, line[2]);
                program.push(st(r1, addr));
            }

            "hlt" => {
                program.push(hlt());
            }

            _ => {
                println!("In line {}, Invalid instruction: {}", index, line[0]);
                std::process::exit(1);
            }
        }
    }

    program
}

fn get_register(index: usize, operand: &str) -> u16 {
    let reg = match operand {
        "reg0" => REG0,
        "reg1" => REG1,
        "reg2" => REG2,
        "reg3" => REG3,
        "reg4" => REG4,
        "reg5" => REG5,
        "reg6" => REG6,
        "reg7" => REG7,
        _ => {
            println!("Invalid Input Error");
            println!(
                "line {}, you must specify a register as an operand, but got \"{}\"",
                index, operand
            );
            std::process::exit(1);
        }
    };
    reg
}

fn get_data(index: usize, operand: &str) -> u16 {
    let data = match operand.parse::<u16>() {
        Ok(data) => {
            if data < 256 {
                data
            } else {
                println!("Invalid Input Error");
                println!(
                    "line {}, you must specify 8 bit data as an operand, but got {}",
                    index, operand
                );
                std::process::exit(1);
            }
        }
        Err(_) => {
            println!("Invalid Input Error");
            println!(
                "line {}, you must specify 8 bit data as an operand, but got \"{}\"",
                index, operand
            );
            std::process::exit(1);
        }
    };
    data
}

fn get_addr(index: usize, operand: &str) -> u16 {
    let addr = match operand.parse::<u16>() {
        Ok(addr) => {
            if addr < 256 {
                addr
            } else {
                println!("Invalid Input Error");
                println!(
                    "line {}, you must specify 8 bit address as an operand, but got {}",
                    index, operand
                );
                std::process::exit(1);
            }
        }
        Err(_) => {
            println!("Invalid Input Error");
            println!(
                "line {}, you must specify 8 bit address as an operand, but got \"{}\"",
                index, operand
            );
            std::process::exit(1);
        }
    };
    addr
}

fn mov(r1: u16, r2: u16) -> u16 {
    (MOV << 11) | (r1 << 8) | (r2 << 5)
}

fn add(r1: u16, r2: u16) -> u16 {
    (ADD << 11) | (r1 << 8) | (r2 << 5)
}

fn sub(r1: u16, r2: u16) -> u16 {
    (SUB << 11) | (r1 << 8) | (r2 << 5)
}

fn and(r1: u16, r2: u16) -> u16 {
    (AND << 11) | (r1 << 8) | (r2 << 5)
}

fn or(r1: u16, r2: u16) -> u16 {
    (OR << 11) | (r1 << 8) | (r2 << 5)
}

fn sl(r1: u16) -> u16 {
    SL << 11 | r1 << 8
}

fn sr(r1: u16) -> u16 {
    SR << 11 | r1 << 8
}

fn sra(r1: u16) -> u16 {
    SRA << 11 | r1 << 8
}

fn ldl(r1: u16, data: u16) -> u16 {
    LDL << 11 | r1 << 8 | data
}

fn ldh(r1: u16, data: u16) -> u16 {
    LDH << 11 | r1 << 8 | data
}

fn cmp(r1: u16, r2: u16) -> u16 {
    CMP << 11 | r1 << 8 | r2 << 5
}

fn je(addr: u16) -> u16 {
    JE << 11 | addr
}

fn jmp(addr: u16) -> u16 {
    JMP << 11 | addr
}

fn ld(r1: u16, addr: u16) -> u16 {
    LD << 11 | r1 << 8 | addr
}

fn st(r1: u16, addr: u16) -> u16 {
    ST << 11 | r1 << 8 | addr
}

fn hlt() -> u16 {
    HLT << 11
}
