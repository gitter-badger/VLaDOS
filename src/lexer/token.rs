pub(crate) mod token_type;

use crate::error::VLaDOSError;
use std::str::FromStr;
use token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            type_: TokenType::Eof,
            lexeme: String::new(),
            line: 0,
        }
    }
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            type_,
            lexeme,
            line,
        }
    }

    pub fn build(lexeme: &str, line: usize) -> Result<Self, VLaDOSError> {
        let type_ = if Token::is_number_token(lexeme) {
            TokenType::NumberLiteral
        } else if lexeme.starts_with('"') {
            TokenType::StringLiteral
        } else {
            match TokenType::from_str(lexeme) {
                Ok(type_) => type_,
                Err(_) => {
                    return Err(VLaDOSError::InvalidToken {
                        line,
                        token: lexeme.into(),
                    })
                }
            }
        };
        Ok(Token::new(type_, lexeme.to_string(), line))
    }

    fn is_number_token(lexeme: &str) -> bool {
        lexeme.parse::<i32>().is_ok() || lexeme.parse::<f64>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_new() {
        assert_eq!(
            Token {
                type_: TokenType::Endmodule,
                lexeme: String::from("endmodule"),
                line: 27
            },
            Token::new(TokenType::Endmodule, String::from("endmodule"), 27)
        );
        assert_eq!(
            Token {
                type_: TokenType::JoinAny,
                lexeme: String::from("join_any"),
                line: 25
            },
            Token::new(TokenType::JoinAny, String::from("join_any"), 25)
        );
    }

    #[test]
    fn token_build() {
        assert_eq!(
            Token {
                type_: TokenType::Endmodule,
                lexeme: String::from("endmodule"),
                line: 27
            },
            Token::build("endmodule", 27).unwrap()
        );
        assert_eq!(
            Token {
                type_: TokenType::JoinAny,
                lexeme: String::from("join_any"),
                line: 25
            },
            Token::build("join_any", 25).unwrap()
        );
    }

    #[test]
    fn token_build_literals() {
        assert_eq!(
            Token {
                type_: TokenType::NumberLiteral,
                lexeme: String::from("23"),
                line: 2
            },
            Token::build("23", 2).unwrap()
        );
        assert_eq!(
            Token {
                type_: TokenType::NumberLiteral,
                lexeme: String::from("4.20"),
                line: 1
            },
            Token::build("4.20", 1).unwrap()
        );
        assert_eq!(
            Token {
                type_: TokenType::StringLiteral,
                lexeme: String::from(r#""This is a string literal""#),
                line: 3
            },
            Token::build(r#""This is a string literal""#, 3).unwrap()
        );
    }
}
