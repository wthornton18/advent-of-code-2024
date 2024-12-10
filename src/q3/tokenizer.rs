use crate::tokenizer::Tokenizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Mul,
    LeftParen,
    RightParen,
    NumericLiteral,
    Comma,
    Other,
    Do,
    Dont,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: usize,
    pub length: usize,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: TokenType::Other,
            pos: 0,
            length: 0,
        }
    }
}

pub struct MulTokenizer<'a> {
    source: &'a [char],
    removed_chars: usize,
}

impl Tokenizer for MulTokenizer<'_> {
    type K = char;

    fn get_source(&self) -> &[Self::K] {
        self.source
    }

    fn advance(&mut self, n: usize) {
        self.removed_chars += n;
        self.source = &self.source[n..];
    }
}

impl MulTokenizer<'_> {
    fn single_char_token(&mut self) -> Option<Token> {
        let token = match self.peek() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            ',' => TokenType::Comma,
            _ => return None,
        };

        let token = Some(self.make_token(token, 1));

        self.advance(1);

        token
    }

    fn numeric_literal(&mut self) -> Option<Token> {
        let mut n = 0;

        while !self.eof_at(n) && self.peek_at(n).is_ascii_digit() {
            n += 1;
        }

        if n == 0 {
            return None;
        }

        let token = Some(self.make_token(TokenType::NumericLiteral, n));

        self.advance(n);

        token
    }

    fn identifier(&mut self) -> Option<Token> {
        let mut n = 0;

        while !self.eof_at(n) && self.peek_at(n).is_ascii_alphabetic() {
            n += 1;
        }

        match &self.source[..n] {
            ['m', 'u', 'l'] => {
                let token = Some(self.make_token(TokenType::Mul, n));
                self.advance(n);
                return token;
            }
            ['d', 'o'] => {
                let token = Some(self.make_token(TokenType::Do, n));
                self.advance(n);
                return token;
            }
            ['d', 'o', 'n'] => {
                if let (Some('\''), Some('t')) = (self.maybe_peek_at(n), self.maybe_peek_at(n + 1))
                {
                    let token = Some(self.make_token(TokenType::Dont, n + 2));
                    self.advance(n + 2);
                    return token;
                }
            }
            _ => {}
        }

        None
    }

    fn make_token(&self, token_type: TokenType, length: usize) -> Token {
        Token {
            token_type,
            pos: self.removed_chars,
            length,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.advance_while(|c| c.is_whitespace());

        if self.eof() {
            return None;
        }

        if let Some(token) = self.single_char_token() {
            return Some(token);
        }

        if let Some(token) = self.numeric_literal() {
            return Some(token);
        }

        if let Some(token) = self.identifier() {
            return Some(token);
        }

        let other = Some(self.make_token(TokenType::Other, 1));
        self.advance(1);
        other
    }
}

impl MulTokenizer<'_> {
    pub fn new(source: &[char]) -> MulTokenizer {
        MulTokenizer {
            source,
            removed_chars: 0,
        }
    }
}

impl Iterator for MulTokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_input() {
        let input = "xmul(2,4)%&mul[3,7]do()don't()";

        let filtered_tokens: Vec<Token> = MulTokenizer::new(&input.chars().collect::<Vec<char>>())
            .filter(|t| t.token_type != TokenType::Other)
            .collect();

        let expected_tokens = vec![
            Token {
                token_type: TokenType::Mul,
                pos: 1,
                length: 3,
            },
            Token {
                token_type: TokenType::LeftParen,
                pos: 4,
                length: 1,
            },
            Token {
                token_type: TokenType::NumericLiteral,
                pos: 5,
                length: 1,
            },
            Token {
                token_type: TokenType::Comma,
                pos: 6,
                length: 1,
            },
            Token {
                token_type: TokenType::NumericLiteral,
                pos: 7,
                length: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                pos: 8,
                length: 1,
            },
            Token {
                token_type: TokenType::Mul,
                pos: 11,
                length: 3,
            },
            Token {
                token_type: TokenType::NumericLiteral,
                pos: 15,
                length: 1,
            },
            Token {
                token_type: TokenType::Comma,
                pos: 16,
                length: 1,
            },
            Token {
                token_type: TokenType::NumericLiteral,
                pos: 17,
                length: 1,
            },
            Token {
                token_type: TokenType::Do,
                pos: 19,
                length: 2,
            },
            Token {
                token_type: TokenType::LeftParen,
                pos: 21,
                length: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                pos: 22,
                length: 1,
            },
            Token {
                token_type: TokenType::Dont,
                pos: 23,
                length: 5,
            },
            Token {
                token_type: TokenType::LeftParen,
                pos: 28,
                length: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                pos: 29,
                length: 1,
            },
        ];

        assert_eq!(filtered_tokens, expected_tokens);
    }
}
