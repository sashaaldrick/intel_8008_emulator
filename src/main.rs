#![allow(non_snake_case)]
use std::ops::{Index, IndexMut};

// you can think of this as the CPU itself
// physical space will be mapped here i.e.
// registers, memory, the program counter etc.
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    H = 5,
    L = 6,
}

pub struct Intel8008 {
    registers: [u8; 7],  // A, B, C, D, E, H, L
    memory: [u8; 16384], // 16K memory or 2^14
}

// These are the methods that belong to the Intel8008 struct
impl Intel8008 {
    pub fn new() -> Self {
        Intel8008 {
            registers: [0; 7],
            memory: [0; 16384],
        }
    }

    pub fn set_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn get_memory(&mut self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    // Indirect Addressing:
    // To access the full 14-bit address space using 8-bit registers,
    // the 8008 uses H and L together to form a complete address:
    // The result is a 16-bit address where H forms the high byte
    // and L forms the low byte.
    pub fn LrM(&mut self, register: Register) {
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
    // let mut cpu = Intel8008::new();

    // cpu[Register::H] = 0b0001_0010;
    // cpu[Register::L] = 0x12;
    // cpu.memory[0x1212] = 0x42;

    // cpu.LrM(Register::A);

    // assert_eq!(cpu[Register::A], 66);
    // println!("seems we legit bro");
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn LrM_works() {
//         let mut cpu = Intel8008::new();

//         cpu[Register::H] = 0x12;
//         cpu[Register::L] = 0x12;
//         cpu.memory[0x1212] = 0x42;

//         // load memory from address made up of combined H, L register to A register.
//         cpu.LrM(Register::A);
//         assert_eq!(cpu[Register::A], cpu.memory[0x1212]);

//     }
// }
