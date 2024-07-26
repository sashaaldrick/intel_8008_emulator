use std::ops::{Index, IndexMut};

// you can think of this as the CPU itself
// physical space will be mapped here i.e.
// registers, memory, the program counter etc.
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    H = 5,
    L = 6,
}

struct Intel8008 {
    registers: [u8; 7],  // A, B, C, D, E, H, L
    memory: [u8; 16384], // 16K memory or 2^14
}

// These are the methods that belong to the Intel8008 struct
impl Intel8008 {
    fn new() -> Self {
        Intel8008 {
            registers: [0; 7],
            memory: [0; 16384],
        }
    }

    // fn lrm(&mut self, register: Register) { 
    // }

    fn lrm(&mut self, register: Register) {
        let address = (self.registers[Register::H as usize] as u16) << 8 
                    | self.registers[Register::L as usize] as u16;
        self.registers[register as usize] = self.memory[address as usize];
    }
}

impl Index<Register> for Intel8008 {
    type Output = u8;

    fn index(&self, register: Register) -> &Self::Output {
        &self.registers[register as usize]
    }
}

impl IndexMut<Register> for Intel8008 {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        &mut self.registers[register as usize]
    }
}

fn main() {
    let mut cpu = Intel8008::new();

    cpu[Register::H] = 0b0001_0010;
    cpu.registers[6] = 0x12;
    cpu.memory[0x1234] = 0x42;

    assert_eq!(cpu.registers[5], cpu.registers[6]);
    println!("seems we legit bro");
}