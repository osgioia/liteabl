#[derive(Debug, Clone)]
pub enum Token {
    ForEach,
    FindFirst,
    Where,
    Create,
    Delete,
    Display,
    
    Identifier(String),
    StringLit(String),
    Number(i64),
    
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    And,
    Or,
    Dot,
    Colon,
    LParen,
    RParen,
    Float(f64),
    
    Eof,
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut chars = source.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '/' => {
                chars.next();
                if let Some('*') = chars.peek() {
                    chars.next();
                    while let Some(c2) = chars.next() {
                        if c2 == '*' {
                            if let Some('/') = chars.peek() {
                                chars.next();
                                break;
                            }
                        }
                    }
                } else {
                    buf.push('/');
                }
            }
            '(' => {
                push_token_from_buf(&mut tokens, &mut buf);
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                push_token_from_buf(&mut tokens, &mut buf);
                tokens.push(Token::RParen);
                chars.next();
            }
            ' ' | '\n' | '\r' | '\t' | ',' => {
                push_token_from_buf(&mut tokens, &mut buf);
                chars.next();
            }
            '.' | ':' | '=' | '<' | '>' => {
                push_token_from_buf(&mut tokens, &mut buf);
                let mut op = c.to_string();
                chars.next();
                if let Some(&next_c) = chars.peek() {
                    if (c == '<' && (next_c == '>' || next_c == '=')) || (c == '>' && next_c == '=') {
                        op.push(next_c);
                        chars.next();
                    }
                }
                match op.as_str() {
                    "." => tokens.push(Token::Dot),
                    ":" => tokens.push(Token::Colon),
                    "=" => tokens.push(Token::Equals),
                    "<>" => tokens.push(Token::NotEquals),
                    "<" => tokens.push(Token::LessThan),
                    ">" => tokens.push(Token::GreaterThan),
                    "<=" => tokens.push(Token::LessOrEqual),
                    ">=" => tokens.push(Token::GreaterOrEqual),
                    _ => {}
                }
            }
            '"' | '\'' => {
                let quote = c;
                chars.next();
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2 == quote {
                        chars.next();
                        break;
                    }
                    s.push(c2);
                    chars.next();
                }
                tokens.push(Token::StringLit(s));
            }
            _ => {
                buf.push(c);
                chars.next();
            }
        }
    }
    push_token_from_buf(&mut tokens, &mut buf);
    tokens.push(Token::Eof);
    tokens
}

fn push_token_from_buf(tokens: &mut Vec<Token>, buf: &mut String) {
    let word = buf.trim();
    if word.is_empty() {
        return;
    }

    let upper = word.to_uppercase();
    match upper.as_str() {
        "FOR EACH" => tokens.push(Token::ForEach),
        "FOR" => {
            *buf = "FOR ".to_string();
            return;
        }
        "FIND FIRST" => tokens.push(Token::FindFirst),
        "FIND" => {
            *buf = "FIND ".to_string();
            return;
        }
        "FIRST" if buf.starts_with("FIND ") => {
            tokens.push(Token::FindFirst);
        }
        "WHERE" => tokens.push(Token::Where),
        "DISPLAY" => tokens.push(Token::Display),
        "CREATE" => tokens.push(Token::Create),
        "DELETE" => tokens.push(Token::Delete),
        "AND" => tokens.push(Token::And),
        "OR" => tokens.push(Token::Or),
        "EQ" => tokens.push(Token::Equals),
        "NE" => tokens.push(Token::NotEquals),
        "LT" => tokens.push(Token::LessThan),
        "GT" => tokens.push(Token::GreaterThan),
        "LE" => tokens.push(Token::LessOrEqual),
        "GE" => tokens.push(Token::GreaterOrEqual),
        _ => {
            if let Ok(n) = word.parse::<i64>() {
                tokens.push(Token::Number(n));
            } else if let Ok(f) = word.parse::<f64>() {
                tokens.push(Token::Float(f));
            } else {
                tokens.push(Token::Identifier(word.to_string()));
            }
        }
    }
    buf.clear();
}