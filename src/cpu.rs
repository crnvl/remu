pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        todo!("")
    }
}