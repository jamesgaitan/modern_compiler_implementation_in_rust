#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Id(String),
    Int(i32),
    Float(f32),
    Boolean(bool),
    Char(char),
    String(String),
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Variant for binary operators
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinOpExpr {
    pub left: Expr,
    pub op: BinOp,
    pub right: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    FunctionCall(FunctionCall),
    Atom(Atom),
    BinOpExpr(Box<BinOpExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}
