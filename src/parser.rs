use crate::{
    ast::{self},
    lexer::Token,
};

pub fn parse(tokens: &[Token]) -> Result<ast::Root, ParseError> {
    let parser = Parser::new(tokens);
    parser.parse()
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken,
    ExpectedToken,
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

    fn peek(&mut self, index: usize) -> Option<&Token> {
        self.tokens.get(self.pos + index)
    }

    fn parse(mut self) -> Result<ast::Root, ParseError> {
        let mut exprs = Vec::new();
        while let Ok(expr) = self.expr() {
            exprs.push(expr);
        }
        Ok(ast::Root { expr: exprs })
    }

    fn expr(&mut self) -> Result<ast::Expr, ParseError> {
        self.expr_inner(0)
    }

    fn expr_inner(&mut self, min_bp: usize) -> Result<ast::Expr, ParseError> {
        let mut leading = match self.next() {
            Some(Token::Int(i)) => ast::Expr::Int(*i),
            Some(_) => return Err(ParseError::UnexpectedToken),
            None => return Err(ParseError::ExpectedToken),
        };

        loop {
            match self.peek(0) {
                Some(Token::Plus) => {
                    const FOWARD_BP: usize = 50;
                    const BACKWARD_BP: usize = 51;

                    if min_bp >= FOWARD_BP {
                        return Ok(leading);
                    }

                    self.next();
                    let following = self.expr_inner(BACKWARD_BP)?;
                    leading = ast::Expr::Plus(Box::new(leading), Box::new(following));
                }
                _ => return Ok(leading),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse_assert_eq {
        ($input:expr, $result:expr) => {
            assert_eq!(parse(&crate::lexer::lexer($input).unwrap()), $result)
        };
    }

    #[test]
    fn num() {
        parse_assert_eq!(
            "14",
            Ok(ast::Root {
                expr: vec![ast::Expr::Int(14)]
            })
        )
    }

    #[test]
    fn plus_expr() {
        parse_assert_eq!(
            "14 + 24",
            Ok(ast::Root {
                expr: vec![ast::Expr::Plus(
                    Box::new(ast::Expr::Int(14)),
                    Box::new(ast::Expr::Int(24))
                )]
            })
        )
    }
}
