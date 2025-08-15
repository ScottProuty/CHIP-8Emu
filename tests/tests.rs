use CHIP-8Emu::*;

#[cfg(test)]
mod tests {
    use super::chip8::Chip8;

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
    fn test_stack_pop_when_empty() {
        let mut chip8 = Chip8::new();
        chip8.stack_push(0xB0B);
        assert_eq!(chip8.stack_pop(), 0xB0B);
        _ = chip8.stack_pop(); // Bad: Pop when nothing left in stack
    }
    
}
