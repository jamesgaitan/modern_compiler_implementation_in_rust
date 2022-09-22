enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

enum Statement {
    CompoundStatement(Box<Statement>, Box<Statement>),
    Assignment { id: String, expr: Box<Expression> },
    Print(Vec<Expression>),
}

enum Expression {
    Id(String),
    Num(i32),
    Op(Box<Expression>, BinOp, Box<Expression>),
    Eseq(Statement, Box<Expression>)
}

fn main() {
    println!("Hello, world!");
}
