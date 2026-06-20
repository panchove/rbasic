#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rbasic::lexer::{self, token::Token};
    use rbasic::Parser;
    use rbasic::Statement;

    fn lex(input: &str) -> Vec<Token> {
        let (tokens, _) = lexer::lex(input);
        tokens
    }

    #[test]
    fn test_parse_function_and_call() {
        let src = r#"
FUNCTION add(a: i32, b: i32) RETURNS i32
    RETURN a + b
END FUNCTION

LET result: i32 = add(10, 20)
PRINT result
"#;
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 3);
    }

    #[test]
    fn test_if_else() {
        let src = r#"
IF x > 0 THEN
    PRINT "positive"
ELSE
    PRINT "non‑positive"
END IF
"#;
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::If { .. } => {}
            _ => panic!("expected If statement"),
        }
    }

    #[test]
    fn test_while() {
        let src = r#"
WHILE i < 10
    PRINT i
END WHILE
"#;
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::While { .. } => {}
            _ => panic!("expected While statement"),
        }
    }

    #[test]
    fn test_for_loop() {
        let src = r#"
FOR i = 1 TO 10
    PRINT i
END FOR
"#;
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::For { .. } => {}
            _ => panic!("expected For statement"),
        }
    }

    #[test]
    fn test_for_step() {
        let src = "FOR i = 1 TO 10 STEP 2\n    PRINT i\nEND FOR";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::For { step: Some(_), .. } => {}
            _ => panic!("expected For with step"),
        }
    }

    #[test]
    fn test_do_while_pre() {
        let src = "DO WHILE x < 10\n    PRINT x\nLOOP";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::DoLoop { .. } => {}
            _ => panic!("expected DoLoop"),
        }
    }

    #[test]
    fn test_do_loop_while_post() {
        let src = "DO\n    PRINT x\nLOOP WHILE x < 10";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
    }

    #[test]
    fn test_invalid_missing_end() {
        let src = "IF x > 0 THEN\n    PRINT x\n"; // missing END IF
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse_program().is_err());
    }

    #[test]
    fn test_let_mut_is_tracked() {
        let src = "LET MUT counter: I32 = 0";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { is_mut, name, .. } => {
                assert!(is_mut, "expected is_mut = true for LET MUT");
                assert_eq!(name, "counter");
            }
            _ => panic!("expected VarDecl"),
        }
    }

    #[test]
    fn test_let_immutable_is_tracked() {
        let src = "LET value: I32 = 42";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { is_mut, .. } => {
                assert!(!is_mut, "expected is_mut = false for plain LET");
            }
            _ => panic!("expected VarDecl"),
        }
    }

    #[test]
    fn test_return_no_expr() {
        let src = "FUNCTION Reset()\n    RETURN\nEND FUNCTION";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::FunctionDecl { body, .. } => match &body[0] {
                Statement::Return { expr } => assert!(expr.is_none()),
                _ => panic!("expected Return"),
            },
            _ => panic!("expected FunctionDecl"),
        }
    }

    #[test]
    fn test_nested_if() {
        let src = "IF a THEN\n    IF b THEN\n        PRINT 1\n    END IF\nEND IF";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::If { then_branch, .. } => {
                assert_eq!(then_branch.len(), 1);
                assert!(matches!(then_branch[0], Statement::If { .. }));
            }
            _ => panic!("expected If"),
        }
    }

    // ---- DIM ----

    #[test]
    fn test_dim_single() {
        let src = "DIM arr(10)";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Dim { declarations } => {
                assert_eq!(declarations.len(), 1);
                assert_eq!(declarations[0].name, "arr");
            }
            _ => panic!("expected Dim"),
        }
    }

    #[test]
    fn test_dim_multiple() {
        let src = "DIM a(10), b(20)";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Dim { declarations } => {
                assert_eq!(declarations.len(), 2);
                assert_eq!(declarations[0].name, "a");
                assert_eq!(declarations[1].name, "b");
            }
            _ => panic!("expected Dim"),
        }
    }

    #[test]
    fn test_dim_multi_dimension() {
        let src = "DIM matrix(5, 5)";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Dim { declarations } => {
                assert_eq!(declarations[0].array_type.dimensions.len(), 2);
            }
            _ => panic!("expected Dim"),
        }
    }

    #[test]
    fn test_dim_with_as_type() {
        let src = "DIM arr(10) AS F64";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Dim { declarations } => {
                assert_eq!(declarations.len(), 1);
                assert_eq!(declarations[0].name, "arr");
                assert_eq!(declarations[0].array_type.base_type.name, "F64");
            }
            _ => panic!("expected Dim"),
        }
    }

    #[test]
    fn test_dim_with_as_type_default_integer() {
        let src = "DIM arr(10)";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Dim { declarations } => {
                assert_eq!(declarations[0].array_type.base_type.name, "INTEGER");
            }
            _ => panic!("expected Dim"),
        }
    }

    #[test]
    fn test_dim_multiple_with_as_type() {
        let src = "DIM a(5) AS U32, b(10) AS STRING";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Dim { declarations } => {
                assert_eq!(declarations.len(), 2);
                assert_eq!(declarations[0].array_type.base_type.name, "U32");
                assert_eq!(declarations[1].array_type.base_type.name, "STRING");
            }
            _ => panic!("expected Dim"),
        }
    }

    // ---- ON ERROR / RESUME ----

    #[test]
    fn test_on_error_goto() {
        let src = "ON ERROR GOTO handler";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::OnError { label } => {
                assert_eq!(label, "handler");
            }
            _ => panic!("expected OnError"),
        }
    }

    #[test]
    fn test_resume() {
        let src = "RESUME";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Resume { label } => {
                assert!(label.is_none());
            }
            _ => panic!("expected Resume"),
        }
    }

    #[test]
    fn test_resume_label() {
        let src = "RESUME next_step";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Resume { label } => {
                assert_eq!(label.as_deref(), Some("next_step"));
            }
            _ => panic!("expected Resume"),
        }
    }

    // ---- DO UNTIL ----

    #[test]
    fn test_do_until_pre() {
        let src = "DO UNTIL done\n    PRINT \"wait\"\nLOOP";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::DoLoop {
                variant, condition, ..
            } => {
                assert!(matches!(variant, rbasic::DoLoopVariant::UntilPre));
                assert!(condition.is_some());
            }
            _ => panic!("expected DoLoop"),
        }
    }

    #[test]
    fn test_do_loop_until_post() {
        let src = "DO\n    PRINT x\nLOOP UNTIL done";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::DoLoop {
                variant, condition, ..
            } => {
                assert!(matches!(variant, rbasic::DoLoopVariant::UntilPost));
                assert!(condition.is_some());
            }
            _ => panic!("expected DoLoop"),
        }
    }

    // ---- AND / OR / XOR ----

    #[test]
    fn test_binary_and() {
        let src = "LET a: BOOL = TRUE AND FALSE";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::And);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    #[test]
    fn test_binary_or() {
        let src = "LET a: BOOL = TRUE OR FALSE";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::Or);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    #[test]
    fn test_binary_xor() {
        let src = "LET a: I32 = 1 XOR 0";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::Xor);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    // ---- SHL / SHR ----

    #[test]
    fn test_binary_shl() {
        let src = "LET a: I32 = 1 SHL 2";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::Shl);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    #[test]
    fn test_binary_shr() {
        let src = "LET a: I32 = 8 SHR 1";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::Shr);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    // ---- NOT unary ----

    #[test]
    fn test_unary_not_expression() {
        let src = "LET a: BOOL = NOT TRUE";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Unary { op, .. } => {
                    assert_eq!(*op, rbasic::UnaryOp::Not);
                }
                _ => panic!("expected unary expression"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    // ---- TRUE / FALSE ----

    #[test]
    fn test_bool_literal_true() {
        let src = "LET a: BOOL = TRUE";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Literal(rbasic::Literal::Bool(true)) => {}
                _ => panic!("expected Bool(true)"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    #[test]
    fn test_bool_literal_false() {
        let src = "LET a: BOOL = FALSE";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::VarDecl { init, .. } => match init {
                rbasic::Expression::Literal(rbasic::Literal::Bool(false)) => {}
                _ => panic!("expected Bool(false)"),
            },
            _ => panic!("expected VarDecl"),
        }
    }

    // ---- Pow / Mod / IntDiv ----

    #[test]
    fn test_binary_pow() {
        let src = "PRINT 2 ^ 3";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Print { expr } => match expr {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::Pow);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected Print"),
        }
    }

    #[test]
    fn test_binary_mod() {
        let src = "PRINT 10 MOD 3";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Print { expr } => match expr {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::Mod);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected Print"),
        }
    }

    #[test]
    fn test_binary_intdiv() {
        let src = "PRINT 7 \\ 3";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Print { expr } => match expr {
                rbasic::Expression::Binary { op, .. } => {
                    assert_eq!(*op, rbasic::BinaryOp::IntDiv);
                }
                _ => panic!("expected binary expression"),
            },
            _ => panic!("expected Print"),
        }
    }

    // ---- AS cast ----

    #[test]
    fn test_as_cast() {
        let src = "PRINT 42 AS I32";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Print { expr } => match expr {
                rbasic::Expression::Cast { target_type, .. } => {
                    assert_eq!(target_type, "I32");
                }
                _ => panic!("expected Cast expression"),
            },
            _ => panic!("expected Print"),
        }
    }

    // ---- Multiple functions ----

    #[test]
    fn test_multiple_functions() {
        let src = "FUNCTION a()\nEND FUNCTION\nFUNCTION b()\nEND FUNCTION";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert_eq!(prog.statements.len(), 2);
    }

    // ---- Operator precedence via grouping ----

    #[test]
    fn test_mul_over_add_precedence() {
        let src = "PRINT 1 + 2 * 3";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[0] {
            Statement::Print { expr } => match expr {
                rbasic::Expression::Binary { left: _, op, right } => {
                    assert_eq!(op, &rbasic::BinaryOp::Add);
                    match right.as_ref() {
                        rbasic::Expression::Binary { op: rop, .. } => {
                            assert_eq!(rop, &rbasic::BinaryOp::Mul);
                        }
                        _ => panic!("expected Mul as right operand"),
                    }
                }
                _ => panic!("expected binary Add"),
            },
            _ => panic!("expected Print"),
        }
    }

    #[test]
    fn test_compound_assign_add() {
        use rbasic::{CompoundAssignOp, Statement};
        let src = "LET MUT x: I32 = 0\nx += 1";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        assert!(matches!(
            &prog.statements[1],
            Statement::AssignOp { name, op: CompoundAssignOp::AddEq, .. } if name == "x"
        ));
    }

    #[test]
    fn test_compound_assign_all_ops() {
        use rbasic::{CompoundAssignOp, Statement};
        let cases = [
            ("x -= 1", CompoundAssignOp::SubEq),
            ("x *= 2", CompoundAssignOp::MulEq),
            ("x /= 2", CompoundAssignOp::DivEq),
            ("x \\= 2", CompoundAssignOp::IntDivEq),
            ("x MOD= 3", CompoundAssignOp::ModEq),
        ];
        for (src, expected_op) in &cases {
            let tokens = lex(src);
            let mut parser = Parser::new(tokens);
            let prog = parser.parse_program().expect("parse error");
            match &prog.statements[0] {
                Statement::AssignOp { op, .. } => assert_eq!(op, expected_op, "failed for {}", src),
                other => panic!("expected AssignOp for {}, got {:?}", src, other),
            }
        }
    }

    // ---- RFC-0019: INPUT ----

    #[test]
    fn test_input_no_prompt() {
        let src = "LET MUT name: STRING = \"\"\nINPUT name";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[1] {
            Statement::Input { prompt, target } => {
                assert!(prompt.is_none());
                assert_eq!(target, "name");
            }
            _ => panic!("expected Input"),
        }
    }

    #[test]
    fn test_input_with_prompt() {
        let src = "LET MUT age: I32 = 0\nINPUT \"Enter age: \", age";
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        match &prog.statements[1] {
            Statement::Input { prompt, target } => {
                assert_eq!(prompt.as_deref(), Some("Enter age: "));
                assert_eq!(target, "age");
            }
            _ => panic!("expected Input"),
        }
    }
}
