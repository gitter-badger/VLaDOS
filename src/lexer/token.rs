pub(crate) mod token_type;

use crate::error::VLaDOSError;
use std::str::FromStr;
use token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    type_: TokenType,
    lexeme: String,
    line: usize,
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
        let type_ = match TokenType::from_str(lexeme) {
            Ok(type_) => type_,
            Err(_) => {
                return Err(VLaDOSError::InvalidToken {
                    line,
                    token: lexeme.into(),
                })
            }
        };
        Ok(Token::new(type_, lexeme.to_string(), line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_new() {
        assert_eq!(
            Token {
                type_: TokenType::Always,
                lexeme: String::from("always"),
                line: 3
            },
            Token::new(TokenType::Always, String::from("always"), 3)
        );
        assert_eq!(
            Token {
                type_: TokenType::EndModule,
                lexeme: String::from("endmodule"),
                line: 27
            },
            Token::new(TokenType::EndModule, String::from("endmodule"), 27)
        );
    }

    #[test]
    fn token_build() {
        assert_eq!(
            Token {
                type_: TokenType::Always,
                lexeme: String::from("always"),
                line: 3
            },
            Token::build("always", 3).unwrap()
        );
        assert_eq!(
            Token {
                type_: TokenType::EndModule,
                lexeme: String::from("endmodule"),
                line: 27
            },
            Token::build("endmodule", 27).unwrap()
        );
    }
}
