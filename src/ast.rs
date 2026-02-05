#[derive(Debug, Clone)]
pub enum Op {
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    String(String),
    Number(i64),
    Float(f64),
    Group(Box<Expr>),
    BinOp { left: Box<Expr>, op: Op, right: Box<Expr> },
}