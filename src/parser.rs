use crate::{ast, lexer::Token};

pub fn parse(tokens: &[Token]) -> Result<ast::Root, ParseError> {
    let parser = Parser::new(tokens);
    parser.parse()
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn parse(mut self) -> Result<ast::Root, ParseError> {
        let int = match self.next() {
            Some(Token::Int(i)) => *i,
            _ => return Err(ParseError::UnexpectedToken),
        };
        Ok(ast::Root { int })
    }
}
