#[derive(Debug, Clone)]
pub enum Op {
    Eq,
}

pub enum Statement {
    ForEach {
        table: String,
        where_clause: Option<Expr>,
        body: Vec<Statement>,
    },
    FindFirst {
        table: String,
        where_clause: Option<Expr>,
    },
    Create { table: String },
    Delete { table: String },
    Display { fields: Vec<String> },
}

pub enum Expr {
    Identifier(String),
    String(String),
    Number(i64),
    BinOp { left: Box<Expr>, op: Op, right: Box<Expr> },
}