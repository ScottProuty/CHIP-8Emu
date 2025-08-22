use chip8emu::*;

#[test]
fn test_set_get_pc(){
    let mut chip8 = Chip8::new();
    chip8.set_pc(0x201);
    assert_eq!(chip8.get_pc(),0x201);
}

#[test]
fn test_set_get_i(){
    let mut chip8 = Chip8::new();
    chip8.set_i(0x201);
    assert_eq!(chip8.get_i(),0x201);
}

#[test]
fn test_v_regs() {
    let mut chip8 = Chip8::new();
    chip8.set_v(0x0, 0xFF);
    chip8.set_v(0xF, 0xFF);
    assert_eq!(chip8.get_v(0x0),0xFF);
    assert_eq!(chip8.get_v(0xF),0xFF);
    chip8.set_v(0x0, 0x00);
    chip8.set_v(0xF, 0x00);
    assert_eq!(chip8.get_v(0x0),0x00);
    assert_eq!(chip8.get_v(0xF),0x00);
}
#[test]
fn test_stack_push_pop() {
    let mut chip8 = Chip8::new();
    chip8.stack_push(0x0);
    chip8.stack_push(0xFFFF);
    assert_eq!(chip8.stack_pop(), 0xFFFF);
    assert_eq!(chip8.stack_pop(), 0x0);
}
#[test]
#[should_panic]
fn test_stack_pop_when_empty_underflow() {
    let mut chip8 = Chip8::new();
    chip8.stack_push(0xB0B);
    assert_eq!(chip8.stack_pop(), 0xB0B);
    _ = chip8.stack_pop(); // Bad: Pop when nothing left in stack
}

#[test]
fn test_mem_write_read() {
    let mut chip8 = Chip8::new();
    chip8.mem_write(0x000,0xFF);
    let result: u8 = chip8.mem_read(0x0000);
    assert_eq!(result, 0xFF);
    chip8.mem_write(0xBEF,0x7A);
    let result: u8 = chip8.mem_read(0xBEF);
    assert_eq!(result, 0x7A);
}

#[test]
fn test_timers_hold_value() {
    let mut chip8 = Chip8::new();
    chip8.d_timer_set(0x50); // arbitrary value < 60
    chip8.s_timer_set(0x45);
    std::thread::sleep(std::time::Duration::from_millis(1000)); // should wait 1 sec
    let dtime: u8 = chip8.d_timer_get();
    let stime: u8 = chip8.s_timer_get();
    assert_eq!(dtime, 0x50); // will both be 0 once decrements implemented
    assert_eq!(stime, 0x45);
}

#[test]
fn test_font_exists() {
    let mut chip8 = Chip8::new();
    assert_eq!(chip8.mem_read(0x50), 0xF0); // first font byte
    assert_eq!(chip8.mem_read(0x6A), 0x80);
    assert_eq!(chip8.mem_read(0x9F), 0x80); // last font byte
}

#[test]
fn print_display_buffer(){
     let chip8 = Chip8::new();
    // for row in chip8.display() {
    //     for &pixel in row {
    //         print!("{}", if pixel { "█" } else { " " });
    //     }
    //     println!();
    // }
    chip8.display_dump();
}

    
