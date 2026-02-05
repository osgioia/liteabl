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
    Dot,
    Colon,
    
    Eof,
}