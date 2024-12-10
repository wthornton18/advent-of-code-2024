use crate::tokenizer::Tokenizer;

use super::tokenizer::{Token, TokenType};

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Mul(i32, i32),
    Do,
    Dont,
}

pub struct Compiler<'a> {
    tokens: &'a [Token],
    source: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn new(tokens: &'a [Token], source: &'a str) -> Self {
        Self { tokens, source }
    }
}

impl Tokenizer for Compiler<'_> {
    type K = Token;

    fn get_source(&self) -> &[Self::K] {
        self.tokens
    }

    fn advance(&mut self, n: usize) {
        self.tokens = &self.tokens[n..];
    }
}

impl Compiler<'_> {
    fn parse_do(&mut self) -> Option<Expr> {
        if self.eof_at(2) {
            return None;
        }
        let valid_sequence = [TokenType::Do, TokenType::LeftParen, TokenType::RightParen];

        let mut valid = true;

        for (token, expected) in self.tokens.iter().zip(valid_sequence.iter()) {
            if token.token_type != *expected {
                valid = false;
                break;
            }
        }

        if !valid {
            return None;
        }

        self.advance(3);

        Some(Expr::Do)
    }

    fn parse_dont(&mut self) -> Option<Expr> {
        if self.eof_at(2) {
            return None;
        }

        let valid_sequence = [TokenType::Dont, TokenType::LeftParen, TokenType::RightParen];

        let mut valid = true;

        for (token, expected) in self.tokens.iter().zip(valid_sequence.iter()) {
            if token.token_type != *expected {
                valid = false;
                break;
            }
        }

        if !valid {
            return None;
        }

        self.advance(3);

        Some(Expr::Dont)
    }

    fn parse_mul(&mut self) -> Option<(i32, i32)> {
        let valid_sequence = [
            TokenType::Mul,
            TokenType::LeftParen,
            TokenType::NumericLiteral,
            TokenType::Comma,
            TokenType::NumericLiteral,
            TokenType::RightParen,
        ];

        let mut valid = true;

        for (token, expected) in self.tokens.iter().zip(valid_sequence.iter()) {
            if token.token_type != *expected {
                valid = false;
                break;
            }
        }

        if !valid {
            return None;
        }

        let a_token = self.peek_at(2);

        let a = self.source[a_token.pos..a_token.pos + a_token.length]
            .parse::<i32>()
            .unwrap();

        let b_token = self.peek_at(4);
        let b = self.source[b_token.pos..b_token.pos + b_token.length]
            .parse::<i32>()
            .unwrap();

        self.advance(6);

        Some((a, b))
    }

    fn next_expr(&mut self) -> Option<Expr> {
        self.advance_while(|c| c.token_type == TokenType::Other);
        if self.eof() {
            return None;
        }

        if let Some(do_) = self.parse_do() {
            return Some(do_);
        }

        if let Some(dont) = self.parse_dont() {
            return Some(dont);
        }

        if let Some(mul) = self.parse_mul() {
            return Some(Expr::Mul(mul.0, mul.1));
        }

        self.advance(1);
        self.next_expr()
    }
}

impl Iterator for Compiler<'_> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expr()
    }
}

#[cfg(test)]
mod tests {
    use super::super::tokenizer::Token;
    use super::*;

    #[test]
    fn test_parse_mul() {
        let input = "mul(2,4)";
        let chars = input.chars().collect::<Vec<char>>();

        let tokenizer = super::super::tokenizer::MulTokenizer::new(&chars);

        let tokens = tokenizer.collect::<Vec<Token>>();

        println!("{:?}", tokens);

        let mut compiler = Compiler {
            tokens: &tokens,
            source: input,
        };

        let result = compiler.parse_mul();

        assert_eq!(result, Some((2, 4)));
    }

    #[test]
    fn test_next_expr() {
        let input = "mul(2,4)%&do()mul[1,2]don't()";
        let chars = input.chars().collect::<Vec<char>>();

        let tokenizer = super::super::tokenizer::MulTokenizer::new(&chars);

        let tokens = tokenizer.collect::<Vec<Token>>();

        let mut compiler = Compiler {
            tokens: &tokens,
            source: input,
        };

        let result = compiler.next_expr();
        println!("{:?}", result);
        assert_eq!(result, Some(Expr::Mul(2, 4)));

        let result = compiler.next_expr();

        assert_eq!(result, Some(Expr::Do));

        let result = compiler.next_expr();

        assert_eq!(result, Some(Expr::Dont));
    }
}
