#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_imports)]

pub struct Chip8 { // definition of CHIP-8
    mem: [u8; 4096],
    stack: Vec<u16>,
    pc: u16,
    i: u16,
    v: [u8; 16], // general-purpose registers V0 (v[0]) through VF (v[15])
    d_timer: u8,
    s_timer: u8
}

impl Chip8 {  // Behavior implementation
    pub fn new() -> Self { // this fn returns "Chip8", which is "Self" in this case
        Self { // This "Self" is a constructor shorthand
            mem: [0; 4096],
            stack: Vec::new(),
            pc: 0x200,
            i: 0x200,
            v: [0; 16],
            d_timer: 0,
            s_timer: 0,
        }
    }

    pub fn set_pc(&mut self, addr: u16) {
        self.pc = addr;
    }
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_i(&mut self, addr: u16) {
        self.i = addr;
    }
    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn set_v(&mut self, index: usize, value: u8) {
        assert!(index < 16, "Reg index out of range (0-15)");
        self.v[index] = value;
    }
    pub fn get_v(&self, index: usize) -> u8 {
        assert!(index < 16, "Reg index out of range (0-15)");
        self.v[index]
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack.push(value);
    }
    pub fn stack_pop(&mut self) -> Option<u16> {
        self.stack.pop()
    }
}