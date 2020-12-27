use crate::lexer::token::Token;
use std::fmt::{Display, Formatter};

pub(crate) trait Visitor<T> {
    fn visit(&self, expr: &Expr) -> T;
}

pub(crate) enum Expr {
    Literal(Token),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl Expr {
    pub(crate) fn walk<T>(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit(self)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut return_val = format!("({}", name);
        for expr in exprs {
            return_val = format!("{} {}", return_val, expr.walk(self));
        }
        return_val += ")";
        return_val
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use Expr::*;
        write!(
            f,
            "{}",
            match self {
                Literal(token) => token.lexeme.to_string(),
                Unary(operator, expr) => self.parenthesize(&operator.lexeme, vec![expr]),
                Binary(lhs, operator, rhs) => self.parenthesize(&operator.lexeme, vec![lhs, rhs]),
                Grouping(expr) => self.parenthesize("group", vec![expr]),
            }
        )
    }
}

impl Visitor<String> for Expr {
    fn visit(&self, expr: &Expr) -> String {
        format!("{}", expr)
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::*;

    fn token(lexeme: &str) -> Token {
        Token::build(lexeme, 1).unwrap()
    }

    #[test]
    fn display_binary_of_two_plus_three() {
        let binary_expr = Binary(
            Box::new(Literal(token("2"))),
            token("+"),
            Box::new(Literal(token("3"))),
        );
        assert_eq!("(+ 2 3)".to_string(), binary_expr.to_string())
    }

    #[test]
    fn display_grouping_of_two_binaries() {
        let binary_lhs = Binary(
            Box::new(Literal(Token::build("5", 1).unwrap())),
            Token::build("*", 1).unwrap(),
            Box::new(Literal(Token::build("4", 1).unwrap())),
        );
        let binary_rhs = Binary(
            Box::new(Literal(Token::build("10", 1).unwrap())),
            Token::build("/", 1).unwrap(),
            Box::new(Literal(Token::build("217.3", 1).unwrap())),
        );
        let grouping_expr = Grouping(Box::new(Binary(
            Box::new(binary_lhs),
            token("&"),
            Box::new(binary_rhs),
        )));
        assert_eq!(
            "(group (& (* 5 4) (/ 10 217.3)))".to_string(),
            grouping_expr.to_string()
        );
    }

    #[test]
    fn display_unary() {
        assert_eq!(
            "(! 1)",
            Unary(token("!"), Box::new(Literal(token("1")))).to_string()
        );
        assert_eq!(
            "(! (^ 0 1))",
            Unary(
                token("!"),
                Box::new(Binary(
                    Box::new(Literal(token("0"))),
                    token("^"),
                    Box::new(Literal(token("1")))
                ))
            )
            .to_string()
        );
    }
}
