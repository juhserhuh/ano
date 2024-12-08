#[derive(Debug, PartialEq)]
pub enum Token {
    Input(String),
    Output(String),
    Identifier(String),
    Keyword(String),
    EOF,
    // Add more token types as necessary
}

pub fn lex(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = 0;

    while current < source_code.len() {
        let ch = source_code[current..].chars().next().unwrap();

        if ch.is_whitespace() {
            current += ch.len_utf8();
            continue;
        }

        // Match input/output keywords and identifiers
        if ch == 'i' && source_code[current..].starts_with("input:") {
            let input_end = source_code[current..].find('\n').unwrap_or(source_code.len());
            tokens.push(Token::Input(source_code[current..input_end].to_string()));
            current = input_end;
            continue;
        }
        if ch == 'o' && source_code[current..].starts_with("output:") {
            let output_end = source_code[current..].find('\n').unwrap_or(source_code.len());
            tokens.push(Token::Output(source_code[current..output_end].to_string()));
            current = output_end;
            continue;
        }

        // Default case for identifiers and unknown tokens
        tokens.push(Token::Identifier(ch.to_string()));
        current += ch.len_utf8();
    }

    tokens.push(Token::EOF); // End of File token
    tokens
}
