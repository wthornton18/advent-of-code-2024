use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stone(usize);

impl FromStr for Stone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|_| ())?))
    }
}

impl Stone {
    pub fn blink(&self) -> (Stone, Option<Stone>) {
        match self.0 {
            0 => (Stone(1), None),
            n => {
                let digit_count = n.ilog10() as usize + 1;
                if digit_count % 2 == 0 {
                    let half = digit_count / 2;
                    let divisor = 10_usize.pow(half as u32);

                    let first_part = n / divisor;
                    let second_part = n % divisor;

                    (Stone(first_part), Some(Stone(second_part)))
                } else {
                    (Stone(n * 2024), None)
                }
            }
        }
    }
}
