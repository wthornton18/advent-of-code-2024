use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    And,
    Or,
    Xor,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::And => write!(f, "AND"),
            Op::Or => write!(f, "OR"),
            Op::Xor => write!(f, "XOR"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimpleGate {
    pub a: String,
    pub b: String,
    pub c: String,
    pub op: Op,
}

impl SimpleGate {
    pub fn new(a: &str, b: &str, op: Op, c: &str) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            op,
            c: c.to_string(),
        }
    }
}

impl SimpleGate {
    pub fn evaluate(&self, a: bool, b: bool) -> bool {
        match self.op {
            Op::And => a && b,
            Op::Or => a || b,
            Op::Xor => a ^ b,
        }
    }
}

impl Display for SimpleGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.a, self.op, self.b, self.c)
    }
}
