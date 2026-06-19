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
}
