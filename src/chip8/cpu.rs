// number of registers
const NUM_REG: usize = 16;

#[derive(Debug)]
pub struct Cpu {
    pub(super) pc: u16,
    pub(super) i: u16,
    pub(super) registers: [u8; NUM_REG],
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            pc: 0x200,
            i: 0,
            registers: [0; NUM_REG],
        }
    }
}

impl Cpu {
    // return the next word and increment the PC
    pub fn fetch<B: AsRef<Vec<u8>>>(&mut self, buffer: B) -> Option<u16> {
        // check if the pc exceed the ram limit
        if self.pc as usize > super::RAM_SIZE {
            return None;
        }

        let buffer = buffer.as_ref();

        let word = (buffer[self.pc as usize] as u16) << 8 | buffer[self.pc as usize + 1] as u16;
        self.pc += 2;

        Some(word)
    }
}
