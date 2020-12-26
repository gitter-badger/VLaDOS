use thiserror::Error;

fn format_err(line: &usize, message: String) -> String {
    format!("[line {}] Error -- {}", line, message)
}

#[derive(Error, Debug, PartialEq)]
pub enum VLaDOSError {
    #[error("{}", format_err(line, format!("Invalid Token: {}", token)))]
    InvalidToken { line: usize, token: String },

    #[error("{}", format_err(line, "Unterminated string".to_string()))]
    UnterminatedString { line: usize },

    #[error("{}", format_err(line, "Unterminated float".to_string()))]
    UnterminatedFloat { line: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_err() {
        assert_eq!(
            String::from("[line 2] Error -- Invalid Token: a_bad_token"),
            VLaDOSError::InvalidToken {
                line: 2,
                token: "a_bad_token".into(),
            }
            .to_string()
        );

        assert_eq!(
            String::from("[line 42] Error -- Unterminated string"),
            VLaDOSError::UnterminatedString { line: 42 }.to_string()
        );
    }
}
