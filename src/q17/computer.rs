use super::op_code::Instruction;

#[derive(Debug, Clone, Copy)]
pub struct Computer {
    a: usize,
    b: usize,
    c: usize,
}

impl Computer {
    pub fn new() -> Self {
        Self { a: 0, b: 0, c: 0 }
    }
    pub fn run(&mut self, instructions: &[usize]) -> Vec<usize> {
        use Instruction::*;
        let mut out = Vec::with_capacity(17);
        let mut instruction_pointer = 0;

        while instruction_pointer < instructions.len() - 1 {
            let instruction = instructions[instruction_pointer].into();
            let operand = instructions[instruction_pointer + 1];
            match instruction {
                Adv => {
                    let combo = self.evaluate_combo_operand(operand);
                    let denominator = 1 << combo;
                    self.a /= denominator;
                }
                Bxl => {
                    self.b ^= operand;
                }
                Bst => {
                    let combo = self.evaluate_combo_operand(operand);
                    self.b = combo & 0x7;
                }
                Jnz if self.a != 0 => {
                    instruction_pointer = operand;
                    continue;
                }
                Jnz => {}
                Bxc => {
                    self.b ^= self.c;
                }
                Out => {
                    let combo = self.evaluate_combo_operand(operand);
                    out.push(combo & 0x7);
                }
                Bdv => {
                    let combo = self.evaluate_combo_operand(operand);
                    let denominator = 1 << combo;
                    self.b = self.a / denominator;
                }
                Cdv => {
                    let combo = self.evaluate_combo_operand(operand);
                    let denominator = 1 << combo;
                    self.c = self.a / denominator;
                }
            }

            instruction_pointer += 2;
        }

        out
    }

    #[inline(always)]
    pub fn evaluate_combo_operand(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid Operand"),
        }
    }

    pub fn set_register_a(&mut self, value: usize) {
        self.a = value;
    }

    pub fn set_register_b(&mut self, value: usize) {
        self.b = value;
    }

    pub fn set_register_c(&mut self, value: usize) {
        self.c = value;
    }

    pub fn new_with_registers(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }
}
