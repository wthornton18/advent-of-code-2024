use compiler::Compiler;
use tokenizer::Token;

pub mod compiler;
pub mod tokenizer;

fn compute_multiplication(input: &str) -> Vec<i32> {
    let chars = input.chars().collect::<Vec<char>>();

    let tokenizer = tokenizer::MulTokenizer::new(&chars);

    let tokens = tokenizer.collect::<Vec<Token>>();
    let compiler = Compiler::new(&tokens, input);

    compiler
        .into_iter()
        .filter_map(|expr| match expr {
            compiler::Expr::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .collect()
}

pub fn compute_multiplication_sum(input: &str) -> i32 {
    compute_multiplication(input).iter().sum()
}

pub fn compute_multiplication_sum_op_aware(input: &str) -> i32 {
    let chars = input.chars().collect::<Vec<char>>();
    let tokenizer = tokenizer::MulTokenizer::new(&chars);

    let tokens = tokenizer.collect::<Vec<Token>>();

    let compiler = Compiler::new(&tokens, input);

    let mut mul_enabled = true;

    let mut result = 0;

    for expr in compiler {
        match expr {
            compiler::Expr::Mul(a, b) => {
                if mul_enabled {
                    result += a * b;
                }
            }
            compiler::Expr::Do => {
                mul_enabled = true;
            }
            compiler::Expr::Dont => {
                mul_enabled = false;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compute_multiplication() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = compute_multiplication(input);

        assert_eq!(result, vec![8, 25, 88, 40]);
    }

    #[test]
    fn test_compute_multiplication_sum_op_aware() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(compute_multiplication_sum_op_aware(input), 48);
    }
}
