#[cfg(test)]
mod tests {
    use rbasic::lexer::lex;
    use rbasic::parser::{ast::*, Parser};

    fn parse(input: &str) -> Program {
        let (tokens, errors) = lex(input);
        assert!(errors.is_empty(), "Lexer errors: {:?}", errors);
        let mut parser = Parser::new(tokens);
        parser.parse_program().expect("Parse error")
    }

    // --- OPEN statement tests ---

    #[test]
    fn test_parse_open_input() {
        let prog = parse("OPEN \"data.txt\" FOR INPUT AS #1");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Open {
                filename,
                mode,
                handle,
                record_len,
            } => {
                assert_eq!(
                    *filename,
                    Expression::Literal(Literal::String("data.txt".to_string()))
                );
                assert_eq!(*mode, FileMode::Input);
                assert_eq!(*handle, Expression::Literal(Literal::Int(1)));
                assert!(record_len.is_none());
            }
            _ => panic!("Expected Open statement"),
        }
    }

    #[test]
    fn test_parse_open_output() {
        let prog = parse("OPEN \"out.txt\" FOR OUTPUT AS #2");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Open { mode, handle, .. } => {
                assert_eq!(*mode, FileMode::Output);
                assert_eq!(*handle, Expression::Literal(Literal::Int(2)));
            }
            _ => panic!("Expected Open statement"),
        }
    }

    #[test]
    fn test_parse_open_random_with_len() {
        let prog = parse("OPEN \"data.dat\" FOR RANDOM AS #3 LEN = 128");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Open {
                mode,
                handle,
                record_len,
                ..
            } => {
                assert_eq!(*mode, FileMode::Random);
                assert_eq!(*handle, Expression::Literal(Literal::Int(3)));
                assert_eq!(*record_len, Some(Expression::Literal(Literal::Int(128))));
            }
            _ => panic!("Expected Open statement"),
        }
    }

    // --- CLOSE statement tests ---

    #[test]
    fn test_parse_close_single() {
        let prog = parse("CLOSE #1");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Close { handles } => {
                assert_eq!(handles.len(), 1);
                assert_eq!(handles[0], Expression::Literal(Literal::Int(1)));
            }
            _ => panic!("Expected Close statement"),
        }
    }

    #[test]
    fn test_parse_close_multiple() {
        let prog = parse("CLOSE #1, #2, #3");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Close { handles } => {
                assert_eq!(handles.len(), 3);
                assert_eq!(handles[0], Expression::Literal(Literal::Int(1)));
                assert_eq!(handles[1], Expression::Literal(Literal::Int(2)));
                assert_eq!(handles[2], Expression::Literal(Literal::Int(3)));
            }
            _ => panic!("Expected Close statement"),
        }
    }

    #[test]
    fn test_parse_close_all() {
        let prog = parse("CLOSE");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Close { handles } => {
                assert!(handles.is_empty());
            }
            _ => panic!("Expected Close statement"),
        }
    }

    // --- INPUT# statement tests ---

    #[test]
    fn test_parse_input_hash() {
        let prog = parse("LET name$ = \"\"\nINPUT #1, name$");
        assert_eq!(prog.statements.len(), 2);
        match &prog.statements[1] {
            Statement::InputHash { handle, targets } => {
                assert_eq!(*handle, Expression::Literal(Literal::Int(1)));
                assert_eq!(targets.len(), 1);
                assert_eq!(targets[0], "name$");
            }
            _ => panic!("Expected InputHash statement"),
        }
    }

    #[test]
    fn test_parse_input_hash_multiple() {
        let prog = parse("LET id = 0\nLET score = 0\nINPUT #1, id, score");
        assert_eq!(prog.statements.len(), 3);
        match &prog.statements[2] {
            Statement::InputHash { handle, targets } => {
                assert_eq!(*handle, Expression::Literal(Literal::Int(1)));
                assert_eq!(targets.len(), 2);
                assert_eq!(targets[0], "id");
                assert_eq!(targets[1], "score");
            }
            _ => panic!("Expected InputHash statement"),
        }
    }

    // --- PRINT# statement tests ---

    #[test]
    fn test_parse_print_hash() {
        let prog = parse("PRINT #1, \"Hello\"");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::PrintHash { handle, items } => {
                assert_eq!(*handle, Expression::Literal(Literal::Int(1)));
                assert_eq!(items.len(), 1);
                assert!(
                    matches!(&items[0], PrintItem::Expr(Expression::Literal(Literal::String(s))) if s == "Hello")
                );
            }
            _ => panic!("Expected PrintHash statement"),
        }
    }

    #[test]
    fn test_parse_print_hash_with_comma() {
        let prog = parse("PRINT #1, a, b");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::PrintHash { handle, items } => {
                assert_eq!(*handle, Expression::Literal(Literal::Int(1)));
                assert_eq!(items.len(), 3);
                assert!(matches!(&items[0], PrintItem::Expr(_)));
                assert!(matches!(&items[1], PrintItem::Comma));
                assert!(matches!(&items[2], PrintItem::Expr(_)));
            }
            _ => panic!("Expected PrintHash statement"),
        }
    }

    #[test]
    fn test_parse_print_hash_with_semicolon() {
        let prog = parse("PRINT #1, id; \" \"; name$");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::PrintHash { handle, items } => {
                assert_eq!(*handle, Expression::Literal(Literal::Int(1)));
                assert_eq!(items.len(), 5);
                assert!(matches!(&items[0], PrintItem::Expr(_)));
                assert!(matches!(&items[1], PrintItem::Semi));
                assert!(
                    matches!(&items[2], PrintItem::Expr(Expression::Literal(Literal::String(s))) if s == " ")
                );
                assert!(matches!(&items[3], PrintItem::Semi));
                assert!(matches!(&items[4], PrintItem::Expr(_)));
            }
            _ => panic!("Expected PrintHash statement"),
        }
    }

    // --- LINE INPUT# statement tests ---

    #[test]
    fn test_parse_line_input_hash() {
        let prog = parse("LET line$ = \"\"\nLINE INPUT #1, line$");
        assert_eq!(prog.statements.len(), 2);
        match &prog.statements[1] {
            Statement::LineInputHash { handle, target } => {
                assert_eq!(*handle, Expression::Literal(Literal::Int(1)));
                assert_eq!(target, "line$");
            }
            _ => panic!("Expected LineInputHash statement"),
        }
    }

    // --- Complete program test ---

    #[test]
    fn test_parse_complete_file_io() {
        let prog = parse(
            r#"
OPEN "data.txt" FOR INPUT AS #1
OPEN "output.txt" FOR OUTPUT AS #2
LET name$ = ""
INPUT #1, name$
PRINT #2, name$
LINE INPUT #1, line$
PRINT #2, line$
CLOSE #1, #2
"#,
        );
        assert_eq!(prog.statements.len(), 8);
        assert!(matches!(&prog.statements[0], Statement::Open { .. }));
        assert!(matches!(&prog.statements[1], Statement::Open { .. }));
        assert!(matches!(&prog.statements[2], Statement::VarDecl { .. }));
        assert!(matches!(&prog.statements[3], Statement::InputHash { .. }));
        assert!(matches!(&prog.statements[4], Statement::PrintHash { .. }));
        assert!(matches!(
            &prog.statements[5],
            Statement::LineInputHash { .. }
        ));
        assert!(matches!(&prog.statements[6], Statement::PrintHash { .. }));
        assert!(matches!(&prog.statements[7], Statement::Close { .. }));
    }
}
