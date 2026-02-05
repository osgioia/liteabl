
use rusqlite::{Connection, Result};
use std::error::Error;
use crate::lexer::Token;
use crate::parser::Parser;
use crate::sqlgen::statement_to_sql;

pub struct Runtime {
    conn: Connection,
}

impl Runtime {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(db_path)?;
        Ok(Runtime { conn })
    }
    
    pub fn execute_query(&self, sql: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(sql)?;
        let column_count = stmt.column_count();
        
        let rows = stmt.query_map([], |row| {
            let mut values = Vec::new();
            for i in 0..column_count {
                let value: Result<String, _> = row.get(i);
                values.push(value.unwrap_or_else(|_| "NULL".to_string()));
            }
            Ok(values)
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        
        Ok(results)
    }
    
    pub fn execute_update(&self, sql: &str) -> Result<usize, Box<dyn Error>> {
        let affected = self.conn.execute(sql, [])?;
        Ok(affected)
    }
}

pub fn execute(db_path: &str, source: &str) -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new(db_path)?;

    let tokens = simple_lexer(source);
    let mut parser = Parser::new(tokens);
    let statements = parser.parse_statements();

    for stmt in &statements {
        if let Some(sql) = statement_to_sql(stmt) {
            println!("[SQL] {}", sql);
            if sql.trim_start().to_uppercase().starts_with("SELECT") {
                let results = runtime.execute_query(&sql)?;
                for row in results {
                    println!("{:?}", row);
                }
            } else {
                let affected = runtime.execute_update(&sql)?;
                println!("[{} rows affected]", affected);
            }
        }
    }
    Ok(())
}

fn simple_lexer(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut chars = source.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            '.' => {
                chars.next();
                if buf.trim().to_uppercase() == "END" {
                    buf.clear();
                    tokens.push(Token::Identifier("END".to_string()));
                    tokens.push(Token::Dot);
                } else {
                    tokens.push(Token::Dot);
                }
            }
            '=' => {
                chars.next();
                tokens.push(Token::Equals);
            }
            '\"' => {
                chars.next();
                let mut s = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc == '\"' {
                        chars.next();
                        break;
                    }
                    s.push(nc);
                    chars.next();
                }
                tokens.push(Token::StringLit(s));
            }
            c if c.is_whitespace() => {
                if !buf.is_empty() {
                    push_token_from_buf(&mut tokens, &mut buf);
                }
                chars.next();
            }
            _ => {
                buf.push(c);
                chars.next();
            }
        }
    }
    if !buf.is_empty() {
        push_token_from_buf(&mut tokens, &mut buf);
    }
    tokens.push(Token::Eof);
    tokens
}

fn push_token_from_buf(tokens: &mut Vec<Token>, buf: &mut String) {
    let word = buf.trim();
    if word.is_empty() { buf.clear(); return; }
    match word.to_uppercase().as_str() {
        "FOREACH" => tokens.push(Token::ForEach),
        "FIND FIRST" => tokens.push(Token::FindFirst),
        "WHERE" => tokens.push(Token::Where),
        "CREATE" => tokens.push(Token::Create),
        "DELETE" => tokens.push(Token::Delete),
        "DISPLAY" => tokens.push(Token::Display),
        "END" => tokens.push(Token::Identifier("END".to_string())),
        _ => {
            if let Ok(n) = word.parse::<i64>() {
                tokens.push(Token::Number(n));
            } else {
                tokens.push(Token::Identifier(word.to_string()));
            }
        }
    }
    buf.clear();
}