/// Implementations described in Chapter 1
use std::cmp;
use std::collections::HashMap;

/// Variant for binary operators
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Calculate the result of the binary operator given two inputs
fn calc_bin_op<T>(left: T, op: &BinOp, right: T) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    match op {
        BinOp::Add => return left + right,
        BinOp::Sub => return left - right,
        BinOp::Mul => return left * right,
        BinOp::Div => return left / right,
    }
}

/// A statement in the program. Statements do not return anything
enum Statement {
    Compound(Box<Statement>, Box<Statement>),
    Assignment { id: String, expr: Box<Expression> },
    Print(Vec<Expression>),
}

/// An expression in the program. Expressions currently always return an integer
enum Expression {
    Id(String),
    Num(i32),
    Op(Box<Expression>, BinOp, Box<Expression>),
    Eseq(Statement, Box<Expression>),
}

/// Returns the maximum number of arguments of any print statement within any subexpression of a given statement
fn max_args(stm: &Statement) -> i32 {
    match stm {
        Statement::Compound(l, r) => return cmp::max(max_args(l), max_args(r)),
        Statement::Assignment { id: _, expr } => return max_args_expr(expr),
        Statement::Print(exprs) => {
            let mut current_max = exprs.len() as i32;
            for expr in exprs {
                current_max = cmp::max(current_max, max_args_expr(expr));
            }
            return current_max;
        }
    }
}

/// Helper function for max_args to do the same functionality on an expression
fn max_args_expr(eseq: &Expression) -> i32 {
    match eseq {
        Expression::Id(_) => return 0,
        Expression::Num(_) => return 0,
        Expression::Op(l, _, r) => return cmp::max(max_args_expr(l), max_args_expr(r)),
        Expression::Eseq(s, e) => return cmp::max(max_args(s), max_args_expr(e)),
    }
}

/// Interperets a statement
fn interp(stm: &Statement) {
    let mut context = HashMap::new();
    interp_stm(stm, &mut context);
}

/// Helper function for interp() which interperets a statement
fn interp_stm(stm: &Statement, context: &mut HashMap<String, i32>) {
    match stm {
        Statement::Compound(l, r) => {
            interp_stm(l, context);
            interp_stm(r, context);
        }
        Statement::Assignment { id: s, expr } => {
            let expr_val = interp_expr(expr, context);
            context.insert(s.clone(), expr_val);
        }
        Statement::Print(exprs) => {
            for expr in exprs {
                println!("{}", interp_expr(expr, context));
            }
        }
    }
}

/// Helper function for interp() which interperets an expression
fn interp_expr(expr: &Expression, context: &mut HashMap<String, i32>) -> i32 {
    match expr {
        Expression::Id(s) => return context[s],
        Expression::Num(i) => return *i,
        Expression::Op(l, bin_op, r) => {
            return calc_bin_op(interp_expr(l, context), bin_op, interp_expr(r, context))
        }
        Expression::Eseq(s, e) => {
            interp_stm(s, context);
            return interp_expr(e, context);
        }
    }
}

pub fn test_exercise_1() {
    let prog = Statement::Compound(
        Box::new(Statement::Assignment {
            id: "a".to_string(),
            expr: Box::new(Expression::Op(
                Box::new(Expression::Num(5)),
                BinOp::Add,
                Box::new(Expression::Num(3)),
            )),
        }),
        Box::new(Statement::Compound(
            Box::new(Statement::Assignment {
                id: "b".to_string(),
                expr: Box::new(Expression::Eseq(
                    Statement::Print(vec![
                        Expression::Id("a".to_string()),
                        Expression::Op(
                            Box::new(Expression::Id("a".to_string())),
                            BinOp::Sub,
                            Box::new(Expression::Num(1)),
                        ),
                    ]),
                    Box::new(Expression::Op(
                        Box::new(Expression::Num(10)),
                        BinOp::Mul,
                        Box::new(Expression::Id("a".to_string())),
                    )),
                )),
            }),
            Box::new(Statement::Print(vec![Expression::Id("b".to_string())])),
        )),
    );

    println!("Max args in prog = {}", max_args(&prog));
    interp(&prog);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc_bin_op() {
        assert_eq!(calc_bin_op(5, &BinOp::Add, 5), 10);
        assert_eq!(calc_bin_op(5, &BinOp::Sub, 5), 0);
        assert_eq!(calc_bin_op(5, &BinOp::Mul, 5), 25);
        assert_eq!(calc_bin_op(5, &BinOp::Div, 5), 1);
    }

    #[test]
    fn test_max_args_basic() {
        let stm = Statement::Print(vec![
            Expression::Num(1),
            Expression::Num(2),
            Expression::Num(3),
        ]);

        assert_eq!(max_args(&stm), 3);
    }

    #[test]
    fn test_max_args_nested() {
        let nested_print = Expression::Eseq(
            Statement::Print(vec![
                Expression::Num(1),
                Expression::Num(2),
                Expression::Num(3),
                Expression::Num(4),
            ]),
            Box::new(Expression::Id("z".to_string())),
        );

        let stm = Statement::Print(vec![
            Expression::Num(1),
            Expression::Num(2),
            Expression::Num(3),
            nested_print,
        ]);

        assert_eq!(max_args(&stm), 4);
    }

    #[test]
    fn test_interp_assignment_stm() {
        let mut context = HashMap::new();

        let stm = Statement::Assignment {
            id: "a".to_string(),
            expr: Box::new(Expression::Num(5)),
        };
        interp_stm(&stm, &mut context);
        assert_eq!(context["a"], 5);
    }

    #[test]
    fn test_interp_compound_stm() {
        let mut context = HashMap::new();

        let stm = Statement::Compound(
            Box::new(Statement::Assignment {
                id: "a".to_string(),
                expr: Box::new(Expression::Num(6)),
            }),
            Box::new(Statement::Assignment {
                id: "b".to_string(),
                expr: Box::new(Expression::Num(7)),
            }),
        );
        interp_stm(&stm, &mut context);
        assert_eq!(context["a"], 6);
        assert_eq!(context["b"], 7);
    }

    #[test]
    fn test_interp_print_stm() {
        let mut context = HashMap::new();
        let original_context = context.clone();

        let stm = Statement::Print(vec![Expression::Num(5), Expression::Num(6)]);
        interp_stm(&stm, &mut context);
        assert_eq!(context, original_context);
    }

    #[test]
    fn test_interp_id_expr() {
        let mut context = HashMap::new();
        context.insert("a".to_string(), 10);

        let expr = Expression::Id("a".to_string());
        assert_eq!(interp_expr(&expr, &mut context), 10);
    }

    #[test]
    fn test_interp_num_expr() {
        let mut context = HashMap::new();

        assert_eq!(interp_expr(&Expression::Num(5), &mut context), 5);
    }

    #[test]
    fn test_interp_bin_op_expr() {
        let mut context = HashMap::new();
        context.insert("a".to_string(), 10);

        assert_eq!(
            interp_expr(
                &Expression::Op(
                    Box::new(Expression::Id("a".to_string())),
                    BinOp::Add,
                    Box::new(Expression::Num(20))
                ),
                &mut context
            ),
            30
        );
    }

    #[test]
    fn test_interp_eseq_expr() {
        let mut context = HashMap::new();

        let eseq = Expression::Eseq(
            Statement::Assignment {
                id: "a".to_string(),
                expr: Box::new(Expression::Num(10)),
            },
            Box::new(Expression::Id("a".to_string())),
        );

        assert_eq!(interp_expr(&eseq, &mut context), 10);
    }
}
