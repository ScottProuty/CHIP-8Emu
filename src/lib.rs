#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_imports)]

pub struct Chip8 { // definition of CHIP-8
    mem: [u8; 4096],
    display: [[bool; 64]; 32], // 64x32 bool array
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
            display: [[false; 64]; 32],
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

    pub fn get_display(&self) -> &[[bool; 64]; 32] {
        &self.display
    }
    pub fn display_dump(&self) {
        for row in self.get_display() {
            for &pixel in row {
                print!("{}", if pixel { "█" } else { " " });
            }
            println!();
        }
    }
    pub fn clear_display(&mut self) {
        self.display = [[false; 64]; 32];
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

    // helper functions for extracting nibbles:
    // Extract first nibble (highest 4 bits of opcode)
    pub fn opcode_nibble(&self, opcode: u16) -> u16 {
        (opcode & 0xF000) >> 12
    }
    // Extract second nibble (X register index)
    pub fn opcode_x(&self, opcode: u16) -> usize {
        ((opcode & 0x0F00) >> 8) as usize
    }
    // Extract third nibble (Y register index)
    pub fn opcode_y(&self, opcode: u16) -> usize {
        ((opcode & 0x00F0) >> 4) as usize
    }
    // Extract fourth nibble (lowest 4 bits)
    pub fn opcode_n(&self, opcode: u16) -> u8 {
        ((opcode & 0x000F)) as u8
    }
    // Extract lowest byte (NN)
    pub fn opcode_nn(&self, opcode: u16) -> u8 {
        ((opcode & 0x00FF)) as u8
    }
    // Extract lowest 12 bits (NNN)
    pub fn opcode_nnn(&self, opcode: u16) -> u16 {
        ((opcode & 0x0FFF)) as u16
    }

    fn fetch (&mut self) -> u16 { // Fetch next opcode (2 bytes) from mem at PC
        let high_byte = self.mem[self.pc as usize] as u16;
        let low_byte = self.mem[(self.pc + 1) as usize] as u16;
        let opcode = (high_byte << 8) | low_byte;
        self.pc += 2;
        opcode
    }

    fn execute (&mut self, opcode: u16) {
        match opcode & 0xF000 { // first byte of opcode
            0x0000 => {
                match opcode {
                    0x00E0 => { // clear screen
                        self.clear_display();
                    }
                    0x00EE => { // return from subroutine
                        self.return_from_subroutine();
                    }
                    _ => {
                        panic!("Unknown 0x0NNN opcode: {:#X", opcode);
                    }
                } 
                
            } 
            0x1000 => { // 0x1NNN: jump to address NNN
                let addr = opcode & 0x0FFF;
                self.pc = addr;
            } 
            0x2000 => {

            }
            0x3000 => {}
            0x4000 => {}
            0x5000 => {}
            0x6000 => {
                self.v[opcode_x(opcode)] = opcode_nn(opcode);

            }
            0x7000 => {}
            0x8000 => {}
            0x9000 => {}
            0xA000 => {}
            0xB000 => {}
            0xC000 => {}
            0xD000 => {}
            0xE000 => {}
            0xF000 => {}
            _ => {println!("Unknown opcode: {:04X}", opcode);}
        }
    }
    
}