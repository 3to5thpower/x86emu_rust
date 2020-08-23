struct Emulator {
    regs: [u32; 32],
    eflags: u32,
    memory: Vec<u32>,
    eip: u32
}

impl Emulator {
    fn new(size: u32, eip: u32, esp: u32) -> Self {
        Emulator {
            regs: [0; 32],
            eflags: 0,
            memory: vec![0; size],
            eip
        }
    }
}

fn main() {
    println!("Hello, world!");
}
