use intel_8008_emulator::{Intel8008, Register};

#[test]
fn LrM_works() {
    let mut cpu = Intel8008::new();

    cpu[Register::H] = 0x12;
    cpu[Register::L] = 0x12;

    cpu.set_memory(0x1212, 0x42);

    // load register with data from memory
    // from address made up of combined H, L register to A register.
    cpu.LrM(Register::A);
    assert_eq!(cpu[Register::A], cpu.get_memory(0x1212));
}
