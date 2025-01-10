use std::str::FromStr;

use hashbrown::HashMap;

use super::op::{Op, SimpleGate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Numeric {
    N,
    Nm1,
}

#[derive(Debug, Clone)]
pub struct AliasDescriptor {
    pub prefix: String,
    pub numeric: Numeric,
}

impl FromStr for AliasDescriptor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (prefix, numeric) = s.split_once('(').ok_or(())?;

        let numeric = match numeric {
            "N)" => Numeric::N,
            "N-1)" => Numeric::Nm1,
            _ => return Err(()),
        };

        Ok(Self {
            prefix: prefix.to_string(),
            numeric,
        })
    }
}

impl From<&str> for AliasDescriptor {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl AliasDescriptor {
    fn applies(&self, name: &str) -> Option<usize> {
        name.strip_prefix(&self.prefix)
            .and_then(|s| s.parse().ok())
            .map(
                |n: usize| {
                    if self.numeric == Numeric::N {
                        n
                    } else {
                        n + 1
                    }
                },
            )
    }

    fn generate(&self, n: usize) -> String {
        let n = if self.numeric == Numeric::N { n } else { n - 1 };
        format!("{}{:0>2}", self.prefix, n)
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    alias_1: AliasDescriptor,
    op: Op,
    alias_2: AliasDescriptor,
    alias_generator: AliasDescriptor,
}

impl Alias {
    pub fn new<A1, A2, A3>(alias_1: A1, op: Op, alias_2: A2, alias_generator: A3) -> Self
    where
        A1: Into<AliasDescriptor>,
        A2: Into<AliasDescriptor>,
        A3: Into<AliasDescriptor>,
    {
        Self {
            alias_1: alias_1.into(),
            op,
            alias_2: alias_2.into(),
            alias_generator: alias_generator.into(),
        }
    }
}

impl Alias {
    pub fn try_generate_alias(&self, gate: &SimpleGate) -> Option<String> {
        if gate.op != self.op {
            return None;
        }

        let a_1 = self.alias_1.applies(&gate.a);
        let a_2 = self.alias_2.applies(&gate.a);

        let b_1 = self.alias_1.applies(&gate.b);
        let b_2 = self.alias_2.applies(&gate.b);

        if !((a_1.is_some() && b_2.is_some()) || (a_2.is_some() && b_1.is_some())) {
            return None;
        }

        let a = if a_1.is_some() { a_1 } else { a_2 }.unwrap();
        let b = if b_1.is_some() { b_1 } else { b_2 }.unwrap();

        if a != b {
            return None;
        }

        Some(self.alias_generator.generate(a))
    }

    pub fn alias(&self, gates: &mut [SimpleGate], renamed_wires: &mut HashMap<String, String>) {
        for g in gates.iter() {
            if let Some(new_name) = self.try_generate_alias(g) {
                renamed_wires.insert(g.c.clone(), new_name.clone());
            }
        }

        for g in gates {
            if let Some(new_a) = renamed_wires.get(&g.a) {
                g.a = new_a.clone();
            }
            if let Some(new_b) = renamed_wires.get(&g.b) {
                g.b = new_b.clone();
            }

            if let Some(new_c) = renamed_wires.get(&g.c) {
                g.c = new_c.clone();
            }
        }
    }
}
