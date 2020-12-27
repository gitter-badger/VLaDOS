use crate::error::VLaDOSError;
use crate::lexer::token::token_type::TokenType;
use crate::lexer::token::Token;
use crate::utils::substring;

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            source: String::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        let mut scanner = Scanner::default();
        scanner.source = source;
        scanner
    }

    pub(crate) fn scan_tokens(&mut self) -> Result<Vec<Token>, VLaDOSError> {
        let mut tokens: Vec<Token> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }
        }
        tokens.push(Token::new(TokenType::Eof, String::new(), self.line));
        Ok(tokens)
    }

    pub(crate) fn scan_token(&mut self) -> Result<Option<Token>, VLaDOSError> {
        let c = self.pop();
        match c {
            '(' | ')' | '{' | '}' | ',' | '.' | '-' | '+' | ';' | '*' => {
                Ok(Some(Token::build(c.to_string().as_str(), self.line)?))
            }
            '!' | '=' | '<' | '>' => match self.pop_if_exp_is_next('=') {
                Some(next_c) => Ok(Some(Token::build(
                    format!("{}{}", c, next_c).as_str(),
                    self.line,
                )?)),
                None => Ok(Some(Token::build(c.to_string().as_str(), self.line)?)),
            },
            '/' => match self.pop_if_exp_is_next('/') {
                Some(_) => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.pop();
                    }
                    Ok(None)
                }
                None => Ok(Some(Token::build(c.to_string().as_str(), self.line)?)),
            },
            '&' | '|' => match self.pop_if_exp_is_next(c) {
                Some(next_c) => Ok(Some(Token::build(
                    format!("{}{}", c, next_c).as_str(),
                    self.line,
                )?)),
                None => Ok(Some(Token::build(c.to_string().as_str(), self.line)?)),
            },
            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            '"' => Ok(Some(Token::new(
                TokenType::StringLiteral,
                self.parse_string()?,
                self.line,
            ))),
            _ => {
                if c.is_digit(10) {
                    Ok(Some(Token::new(
                        TokenType::NumberLiteral,
                        self.parse_number()?,
                        self.line,
                    )))
                } else if c.is_alphabetic() {
                    Ok(Some(Token::build(&self.parse_identifier()?, self.line)?))
                } else {
                    Err(VLaDOSError::InvalidToken {
                        line: self.line,
                        token: c.to_string(),
                    })
                }
            }
        }
    }

    /// Look ahead at the next character, but do not advance the current pointer
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_at(self.current)
        }
    }

    fn peek_next(&mut self) -> Result<char, VLaDOSError> {
        if self.current + 1 >= self.source.len() {
            Err(VLaDOSError::UnterminatedFloat { line: self.line })
        } else {
            Ok(self.source_at(self.current + 1))
        }
    }

    /// Advance the current pointer by one and get the next character in the source
    fn pop(&mut self) -> char {
        self.current += 1;
        self.source_at(self.current - 1)
    }

    /// Advance the current pointer by one if `exp` is the next character
    fn pop_if_exp_is_next(&mut self, exp: char) -> Option<char> {
        if self.is_at_end() || self.source_at(self.current) != exp {
            return None;
        }
        self.current += 1;
        Some(exp)
    }

    fn source_at(&self, i: usize) -> char {
        self.source.as_bytes()[i] as char
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn parse_string(&mut self) -> Result<String, VLaDOSError> {
        while self.peek() != '"' {
            if self.is_at_end() {
                return Err(VLaDOSError::UnterminatedString { line: self.line });
            }

            if self.peek() == '\n' {
                self.line += 1;
            }

            let _ = self.pop();
        }

        // Advance past the closing "
        self.pop();

        // +1 and -1 for dropping " characters in the lexeme
        Ok(substring(&self.source, self.start + 1, self.current - 1))
    }

    fn parse_number(&mut self) -> Result<String, VLaDOSError> {
        while self.peek().is_digit(10) {
            self.pop();
        }
        if self.peek() == '.' && self.peek_next()?.is_digit(10) {
            self.pop(); // Consume the "."
            while self.peek().is_digit(10) {
                self.pop();
            }
        }
        Ok(substring(&self.source, self.start, self.current))
    }

    fn parse_identifier(&mut self) -> Result<String, VLaDOSError> {
        while self.peek().is_alphanumeric() {
            self.pop();
        }
        Ok(substring(&self.source, self.start, self.current))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::token_type::TokenType;

    #[test]
    fn scan_empty() {
        let exp = vec![Token::new(TokenType::Eof, "".into(), 1)];
        let act = Scanner::new("".into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_plus() {
        let exp = vec![
            Token::new(TokenType::Plus, "+".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"+"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_minus() {
        let exp = vec![
            Token::new(TokenType::Minus, "-".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"-"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_multiple_operators() {
        let exp = vec![
            Token::new(TokenType::Plus, "+".into(), 1),
            Token::new(TokenType::Minus, "-".into(), 1),
            Token::new(TokenType::Bang, "!".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"+-!"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_multi_char_operators() {
        let exp = vec![
            Token::new(TokenType::GreaterEqual, ">=".into(), 1),
            Token::new(TokenType::LessEqual, "<=".into(), 1),
            Token::new(TokenType::LogicalEqual, "==".into(), 1),
            Token::new(TokenType::LogicalUnequal, "!=".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#">=<===!="#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_divide() {
        let exp = vec![
            Token::new(TokenType::ForwardSlash, "/".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"/"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_comment() {
        let exp = vec![Token::new(TokenType::Eof, "".into(), 1)];
        let act = Scanner::new(r#"// Ignore this line."#.into())
            .scan_tokens()
            .unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_string() {
        let exp = vec![
            Token::new(TokenType::StringLiteral, r#"My own string"#.into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#""My own string""#.into())
            .scan_tokens()
            .unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_string_across_newlines() {
        let exp = vec![
            Token::new(TokenType::StringLiteral, "My own\nstring".into(), 2),
            Token::new(TokenType::Eof, "".into(), 2),
        ];
        let act = Scanner::new(
            r#""My own
string""#
                .into(),
        )
        .scan_tokens()
        .unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_strings() {
        let exp = vec![
            Token::new(TokenType::StringLiteral, r#"aaaa"#.into(), 1),
            Token::new(TokenType::StringLiteral, r#"bbbb"#.into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#""aaaa" "bbbb""#.into())
            .scan_tokens()
            .unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_unterminated_string_throws_error() {
        let exp = Some(VLaDOSError::UnterminatedString { line: 1 });
        let act = Scanner::new(r#""aaaa"#.into()).scan_tokens().err();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_string_plus_operator() {
        let exp = vec![
            Token::new(TokenType::StringLiteral, r#"My own string"#.into(), 1),
            Token::new(TokenType::Plus, "+".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#""My own string" +"#.into())
            .scan_tokens()
            .unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_number() {
        let exp = vec![
            Token::new(TokenType::NumberLiteral, "2".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"2"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_multi_digit_number() {
        let exp = vec![
            Token::new(TokenType::NumberLiteral, "42".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"42"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_float() {
        let exp = vec![
            Token::new(TokenType::NumberLiteral, "6.9".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"6.9"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_unterminated_float_throws_error() {
        let exp = Some(VLaDOSError::UnterminatedFloat { line: 1 });
        let act = Scanner::new(r#"6."#.into()).scan_tokens().err();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_numbers() {
        let exp = vec![
            Token::new(TokenType::NumberLiteral, "2".into(), 1),
            Token::new(TokenType::NumberLiteral, "3".into(), 1),
            Token::new(TokenType::NumberLiteral, "4".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new(r#"2 3 4"#.into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_identifier() {
        let exp = vec![
            Token::new(TokenType::BooleanAnd, "&&".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new("&&".into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_identifiers() {
        let exp = vec![
            Token::new(TokenType::BooleanOr, "||".into(), 1),
            Token::new(TokenType::Function, "function".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        let act = Scanner::new("|| function".into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_newlines() {
        let exp = vec![Token::new(TokenType::Eof, "".into(), 4)];
        let act = Scanner::new("\n\n\n".into()).scan_tokens().unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn scan_invalid_token_throws_error() {
        let token = "\0".to_string();
        let exp = Some(VLaDOSError::InvalidToken {
            line: 1,
            token: token.clone(),
        });
        let act = Scanner::new(token).scan_tokens().err();
        assert_eq!(exp, act);
    }
}
