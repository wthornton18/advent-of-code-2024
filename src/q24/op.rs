#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Equation<'a> {
    pub w_1: &'a str,
    pub w_2: &'a str,
    pub out: &'a str,
    pub op: Op,
}

impl<'a> Equation<'a> {
    pub fn new(w_1: &'a str, w_2: &'a str, op: Op, out: &'a str) -> Self {
        Self { w_1, w_2, op, out }
    }
}

impl Equation<'_> {
    pub fn evaluate(&self, w_1: bool, w_2: bool) -> bool {
        match self.op {
            Op::And => w_1 && w_2,
            Op::Or => w_1 || w_2,
            Op::Xor => w_1 ^ w_2,
        }
    }
}
