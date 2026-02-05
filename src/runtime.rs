use rusqlite::{Connection, Result};
use std::error::Error;
use crate::lexer;
use crate::parser::Parser;
use crate::sqlgen::statement_to_sql;
use crate::tui::display_results;

pub struct Runtime {
    conn: Connection,
}

impl Runtime {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(db_path)?;
        Ok(Runtime { conn })
    }
    
    pub fn execute_query(&self, sql: &str) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
        let mut stmt = self.conn.prepare(sql)?;
        let column_count = stmt.column_count();
        let column_names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();
        
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
        
        Ok((column_names, results))
    }
    
    pub fn execute_update(&self, sql: &str) -> Result<usize, Box<dyn Error>> {
        let affected = self.conn.execute(sql, [])?;
        Ok(affected)
    }
}

pub fn execute(db_path: &str, source: &str, verbose: bool) -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new(db_path)?;

    let tokens = lexer::lex(source);
    if verbose {
        println!("Tokens: {:?}", tokens);
    }
    let mut parser = Parser::new(tokens);
    let statements = parser.parse_statements();
    if verbose {
        println!("Statements: {} found", statements.len());
    }

    for stmt in &statements {
        if let Some(sql) = statement_to_sql(stmt) {
            if verbose {
                println!("[SQL] {}", sql);
            }
            if sql.trim_start().to_uppercase().starts_with("SELECT") {
                let (cols, results) = runtime.execute_query(&sql)?;
                display_results(&cols, &results)?;
            } else {
                let affected = runtime.execute_update(&sql)?;
                if verbose {
                    println!("[{} rows affected]", affected);
                }
            }
        }
    }
    Ok(())
}