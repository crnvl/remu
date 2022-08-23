pub mod cpu;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::cpu;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = cpu::CPU::new();
        cpu.interpret(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = cpu::CPU::new();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}