
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_imports)]

use chip8emu::Chip8;

fn main() {
    let chip8 = Chip8::new();
    println!("CHIP-8 loaded. PC is at {:#X}", chip8.get_pc());
}

