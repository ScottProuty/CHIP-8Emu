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
        let mut chip8 = Chip8 { // constructor
            mem: [0; 4096],
            stack: Vec::with_capacity(16),
            pc: 0x200,
            i: 0x200,
            v: [0; 16],
            d_timer: 0,
            s_timer: 0,
        };

        const FONT: [u8;80] = [ // fontset - 16 chars, each 5 bytes
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        
        // Fills mem 0x50 -> 0x99 (inclusive) with FONT data
        chip8.mem[0x50..0x50 + FONT.len()].copy_from_slice(&FONT); 

        chip8
    }

    pub fn mem_write(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }
    pub fn mem_read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack.push(value);
    }
    pub fn stack_pop(&mut self) -> u16 {
        self.stack.pop().unwrap_or_else(|| panic!("Stack underflow - nothing to pop"))
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

    pub fn d_timer_set(&mut self, value: u8) {
        self.d_timer = value;
    }
    pub fn d_timer_get(&self) -> u8 {
        self.d_timer
    }
    pub fn s_timer_set(&mut self, value: u8) {
        self.s_timer = value;
    }
    pub fn s_timer_get(&self) -> u8 {
        self.s_timer
    }

    
}