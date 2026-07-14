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
        let mut stmts = Vec::new();
        while let Some(stmt) = self.stmt()? {
            stmts.push(stmt);
        }

        Ok(ast::Root { stmts })
    }

    fn stmt(&mut self) -> Result<Option<ast::Stmt>, ParseError> {
        match self.peek(0) {
            Some(Token::Let) => {
                self.next();
                let ident = match self.next() {
                    Some(Token::Ident(i)) => i.clone(),
                    _ => return Err(ParseError::UnexpectedToken),
                };

                match self.next() {
                    Some(Token::Equal) => {}
                    _ => return Err(ParseError::UnexpectedToken),
                }

                let expr = match self.expr()? {
                    Some(expr) => expr,
                    None => return Err(ParseError::UnexpectedToken),
                };

                Ok(Some(ast::Stmt::Let(ident, expr)))
            }
            Some(Token::Return) => {
                self.next();
                let expr = match self.expr()? {
                    Some(expr) => expr,
                    None => return Err(ParseError::UnexpectedToken),
                };
                Ok(Some(ast::Stmt::Return(expr)))
            }
            Some(_) => match self.expr()? {
                Some(expr) => Ok(Some(ast::Stmt::Expr(expr))),
                None => Err(ParseError::UnexpectedToken),
            },
            None => Ok(None),
        }
    }

    fn expr(&mut self) -> Result<Option<ast::Expr>, ParseError> {
        self.expr_inner(0)
    }

    fn expr_inner(&mut self, min_bp: usize) -> Result<Option<ast::Expr>, ParseError> {
        let mut leading = match self.next() {
            Some(Token::Int(i)) => ast::Expr::Int(*i),
            Some(Token::Ident(i)) => ast::Expr::Var(i.clone()),
            Some(_) | None => return Ok(None),
        };

        loop {
            match self.peek(0) {
                Some(Token::Plus) => {
                    const FOWARD_BP: usize = 50;
                    const BACKWARD_BP: usize = 51;

                    if min_bp >= FOWARD_BP {
                        return Ok(Some(leading));
                    }

                    self.next();
                    let following = match self.expr_inner(BACKWARD_BP)? {
                        Some(f) => f,
                        None => return Err(ParseError::ExpectedToken),
                    };
                    leading = ast::Expr::Plus(Box::new(leading), Box::new(following));
                }
                _ => return Ok(Some(leading)),
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
                stmts: vec![ast::Stmt::Expr(ast::Expr::Int(14))]
            })
        )
    }

    #[test]
    fn plus_expr() {
        parse_assert_eq!(
            "14 + 24",
            Ok(ast::Root {
                stmts: vec![ast::Stmt::Expr(ast::Expr::Plus(
                    Box::new(ast::Expr::Int(14)),
                    Box::new(ast::Expr::Int(24))
                ))]
            })
        )
    }
}
