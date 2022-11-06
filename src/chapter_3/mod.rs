extern crate lalrpop;

mod ast;
mod parser;

pub fn test_chapter_3() {
    println!("\n----Test Chapter 3----\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num() {
        let src = "64";
        let should_be = ast::Atom::Int(64);

        let got = parser::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_float() {
        let src = "3.14";
        let should_be = ast::Atom::Float(3.14);

        let got = parser::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_boolean() {
        let src = "True";
        let should_be = ast::Atom::Boolean(true);

        let got = parser::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);

        let src = "False";
        let should_be = ast::Atom::Boolean(false);

        let got = parser::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_identifier() {
        let src = "x";
        let should_be = ast::Atom::Id(String::from(src));

        let got = parser::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_multiple_parens() {
        let src = "(22)";
        let should_be = ast::Expr::Atom(ast::Atom::Int(22));

        let got = parser::ExprParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);

        let src = "(((22)))";
        let should_be = ast::Expr::Atom(ast::Atom::Int(22));

        let got = parser::ExprParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_function_call() {
        let src = "function_name(arg1, arg2, arg3)";
        let should_be = ast::Expr::FunctionCall(ast::FunctionCall {
            name: String::from("function_name"),
            arguments: vec![
                ast::Expr::Atom(ast::Atom::Id(String::from("arg1"))),
                ast::Expr::Atom(ast::Atom::Id(String::from("arg2"))),
                ast::Expr::Atom(ast::Atom::Id(String::from("arg3"))),
            ],
        });

        let got = parser::ExprParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_mult_expr() {
        let src = "64*x";
        let should_be = ast::Expr::BinOpExpr(Box::new(ast::BinOpExpr {
            left: ast::Expr::Atom(ast::Atom::Int(64)),
            op: ast::BinOp::Mul,
            right: ast::Expr::Atom(ast::Atom::Id(String::from("x"))),
        }));

        let got = parser::FactorParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_add_expr() {
        let src = "64+x";
        let should_be = ast::Expr::BinOpExpr(Box::new(ast::BinOpExpr {
            left: ast::Expr::Atom(ast::Atom::Int(64)),
            op: ast::BinOp::Add,
            right: ast::Expr::Atom(ast::Atom::Id(String::from("x"))),
        }));

        let got = parser::TermParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_if_block() {
        let src = "if (x) { function_call(x); function_call(x); } else { function_call(z); }";
        let should_be = ast::Statement::IfElseBlock(
            ast::Expr::Atom(ast::Atom::Id("x".to_string())),
            vec![
                ast::Statement::Expr(ast::Expr::FunctionCall(ast::FunctionCall {
                    name: "function_call".to_string(),
                    arguments: vec![ast::Expr::Atom(ast::Atom::Id("x".to_string()))],
                })),
                ast::Statement::Expr(ast::Expr::FunctionCall(ast::FunctionCall {
                    name: "function_call".to_string(),
                    arguments: vec![ast::Expr::Atom(ast::Atom::Id("x".to_string()))],
                })),
            ],
            vec![ast::Statement::Expr(ast::Expr::FunctionCall(
                ast::FunctionCall {
                    name: "function_call".to_string(),
                    arguments: vec![ast::Expr::Atom(ast::Atom::Id("z".to_string()))],
                },
            ))],
        );

        let got = parser::IfBlockParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_for_loop() {
        let src = "for x; y; z { function_call(x, y, z); }";
        let should_be = ast::Statement::ForLoop(
            ast::Expr::Atom(ast::Atom::Id("x".to_string())),
            ast::Expr::Atom(ast::Atom::Id("y".to_string())),
            ast::Expr::Atom(ast::Atom::Id("z".to_string())),
            vec![ast::Statement::Expr(ast::Expr::FunctionCall(
                ast::FunctionCall {
                    name: "function_call".to_string(),
                    arguments: vec![
                        ast::Expr::Atom(ast::Atom::Id("x".to_string())),
                        ast::Expr::Atom(ast::Atom::Id("y".to_string())),
                        ast::Expr::Atom(ast::Atom::Id("z".to_string())),
                    ],
                },
            ))],
        );

        let got = parser::ForLoopParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_while_loop() {
        let src = "while (x + 5) { function_call(x); }";
        let should_be = ast::Statement::WhileLoop(
            ast::Expr::BinOpExpr(Box::new(ast::BinOpExpr {
                left: ast::Expr::Atom(ast::Atom::Id(String::from("x"))),
                op: ast::BinOp::Add,
                right: ast::Expr::Atom(ast::Atom::Int(5)),
            })),
            vec![ast::Statement::Expr(ast::Expr::FunctionCall(
                ast::FunctionCall {
                    name: "function_call".to_string(),
                    arguments: vec![ast::Expr::Atom(ast::Atom::Id("x".to_string()))],
                },
            ))],
        );

        let got = parser::WhileLoopParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }
}
