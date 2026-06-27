use std::{iter::Peekable, str::Bytes};

#[derive(Debug, Clone)]
pub enum Token {
    Int(i64),
}

#[derive(Debug, Clone)]
pub enum LexError {
    UnexpectedByte(u8),
}

pub fn lexer(script: &str) -> Result<Vec<Token>, LexError> {
    let lex = Lexer::new(script);
    lex.lex()
}

struct Lexer<'a> {
    script: Peekable<Bytes<'a>>,
    result: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn new(script: &'a str) -> Self {
        Self {
            script: script.bytes().peekable(),
            result: Vec::new(),
        }
    }

    fn lex(mut self) -> Result<Vec<Token>, LexError> {
        self.root()?;
        Ok(self.result)
    }

    fn root(&mut self) -> Result<(), LexError> {
        loop {
            let b = match self.script.next() {
                Some(b) => b,
                None => break,
            };

            match b {
                b'0'..=b'9' => {
                    self.num(b)?;
                }
                _ => return Err(LexError::UnexpectedByte(b)),
            }
        }

        Ok(())
    }

    fn num(&mut self, first_char: u8) -> Result<(), LexError> {
        let mut string = String::new();
        string.push(first_char as char);

        while let Some(c) = self.script.peek() {
            match c {
                b'0'..=b'9' => string.push((*c) as char),
                b'_' => {}
                _ => break,
            }
            self.script.next();
        }

        self.result.push(Token::Int(string.parse().unwrap()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! lex_assert {
        ($input:expr, $pat:pat) => {
            std::assert_matches!(lexer($input).as_deref(), $pat)
        };
    }

    #[test]
    fn num() {
        lex_assert!("42", Ok([Token::Int(42)]));
        lex_assert!("1_4", Ok([Token::Int(14)]));
    }
}
