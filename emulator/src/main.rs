/*
命令長は15ビット
したがって機械語を格納するインストラクションレジスタやメインメモリのプログラム領域におけるデータ長も15ビット

データ表現は16ビット
汎用レジスタやメインメモリのデータ領域におけるデータ長は16ビット

15ビットの命令長の内訳
- 4ビット: 命令コード
- 11ビット: レジスタ1、レジスタ2、データ、アドレス
ただしレジスタは3ビット、データは8ビット、アドレスは8ビット
したがって命令の数は2^4=16通り、汎用レジスタの数は2^3=8通り、絶対アドレスの数は2^8=256通り

メモリとレジスタのデータ転送は
- プログラム領域とインストラクションレジスタ間
- データ領域と汎用レジスタ間
*/

#![allow(dead_code)]

mod assembler;

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

fn main() {
    let mut emu = Emulator::new("sample.txt");
    emu.run();
    println!("{:?}", emu.ram[64]);
}

struct Emulator {
    // 汎用レジスタ
    reg: Vec<u16>,
    // メインメモリのプログラム領域
    rom: Vec<u16>,
    // メインメモリのデータ領域
    ram: Vec<u16>,
    // プログラムカウンタ
    pc: usize,
    // インストラクションレジスタ
    ir: u16,
    // フラグ
    flag: u16,
}

impl Emulator {
    fn new(file_path: &str) -> Emulator {
        let program = assembler::assembler(file_path.to_string());
        Emulator {
            reg: vec![0; 8],
            rom: program,
            ram: vec![0; 256],
            pc: 0,
            ir: 0,
            flag: 0,
        }
    }

    fn run(&mut self) {
        while op_code(self.rom[self.pc]) != HLT {
            self.ir = self.rom[self.pc];
            match op_code(self.rom[self.pc]) {
                MOV => {
                    let op1 = reg1(self.ir);
                    let op2 = reg2(self.ir);
                    self.reg[op1 as usize] = self.reg[op2 as usize];
                }

                ADD => {
                    let op1 = reg1(self.ir);
                    let op2 = reg2(self.ir);
                    self.reg[op1 as usize] += self.reg[op2 as usize];
                }

                SUB => {
                    let op1 = reg1(self.ir);
                    let op2 = reg2(self.ir);
                    self.reg[op1 as usize] -= self.reg[op2 as usize];
                }

                AND => {
                    let op1 = reg1(self.ir);
                    let op2 = reg2(self.ir);
                    self.reg[op1 as usize] &= self.reg[op2 as usize];
                }

                OR => {
                    let op1 = reg1(self.ir);
                    let op2 = reg2(self.ir);
                    self.reg[op1 as usize] |= self.reg[op2 as usize];
                }

                SL => {
                    let op1 = reg1(self.ir);
                    self.reg[op1 as usize] <<= 1;
                }

                SR => {
                    let op1 = reg1(self.ir);
                    self.reg[op1 as usize] >>= 1;
                }

                SRA => {
                    let op1 = reg1(self.ir);
                    self.reg[op1 as usize] =
                        (self.reg[op1 as usize] >> 1) | (self.reg[op1 as usize] & 0x8000);
                }

                LDL => {
                    let op1 = reg1(self.ir);
                    let data = data(self.ir);
                    self.reg[op1 as usize] = (self.reg[op1 as usize] & 0xff00) | data;
                }

                LDH => {
                    let op1 = reg1(self.ir);
                    let data = data(self.ir);
                    self.reg[op1 as usize] = (self.reg[op1 as usize] & 0x00ff) | (data << 8);
                }

                CMP => {
                    let op1 = reg1(self.ir);
                    let op2 = reg2(self.ir);
                    if self.reg[op1 as usize] == self.reg[op2 as usize] {
                        self.flag = 1;
                    } else {
                        self.flag = 0;
                    }
                }

                JE => {
                    let addr = address(self.ir);
                    if self.flag == 1 {
                        self.pc = addr as usize;
                    }
                }

                JMP => {
                    let addr = address(self.ir);
                    self.pc = addr as usize;
                }

                LD => {
                    let op1 = reg1(self.ir);
                    let addr = address(self.ir);
                    self.reg[op1 as usize] = self.ram[addr as usize];
                }

                ST => {
                    let op1 = reg1(self.ir);
                    let addr = address(self.ir);
                    self.ram[addr as usize] = self.reg[op1 as usize];
                }

                _ => {
                    println!("Invalid instruction: {}", op_code(self.rom[self.pc]));
                    std::process::exit(1);
                }
            }
            self.pc += 1;
        }
    }
}

fn op_code(code: u16) -> u16 {
    code >> 11
}

fn reg1(code: u16) -> u16 {
    (code >> 8) & 0b111
}

fn reg2(code: u16) -> u16 {
    (code >> 5) & 0b111
}

fn data(code: u16) -> u16 {
    code & 0b11111111
}

fn address(code: u16) -> u16 {
    code & 0b11111111
}
