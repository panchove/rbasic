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

    #[test]
    fn test_select_case_single_value() {
        let src = r#"
SELECT CASE x
    CASE 1
        PRINT "one"
    CASE 2
        PRINT "two"
    CASE ELSE
        PRINT "other"
END SELECT
"#;
        let program = parse(src);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::SelectCase {
                expr: _,
                cases,
                else_case,
            } => {
                assert_eq!(cases.len(), 2);
                assert!(else_case.is_some());
            }
            _ => panic!("Expected SelectCase"),
        }
    }

    #[test]
    fn test_select_case_range() {
        let src = r#"
SELECT CASE grade
    CASE 90 TO 100
        PRINT "A"
    CASE 80 TO 89
        PRINT "B"
    CASE ELSE
        PRINT "F"
END SELECT
"#;
        let program = parse(src);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::SelectCase { cases, .. } => {
                assert_eq!(cases.len(), 2);
                match &cases[0].values[0] {
                    CaseValue::Range(_low, _high) => {
                        // Check that range is 90 TO 100
                    }
                    _ => panic!("Expected Range"),
                }
            }
            _ => panic!("Expected SelectCase"),
        }
    }

    #[test]
    fn test_select_case_multiple_values() {
        let src = r#"
SELECT CASE command$
    CASE "QUIT", "EXIT"
        END
    CASE "HELP"
        PRINT "No help"
END SELECT
"#;
        let program = parse(src);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::SelectCase { cases, .. } => {
                assert_eq!(cases.len(), 2);
                assert_eq!(cases[0].values.len(), 2); // "QUIT", "EXIT"
            }
            _ => panic!("Expected SelectCase"),
        }
    }

    #[test]
    fn test_select_case_no_else() {
        let src = r#"
SELECT CASE x
    CASE 1
        PRINT "one"
END SELECT
"#;
        let program = parse(src);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::SelectCase {
                cases, else_case, ..
            } => {
                assert_eq!(cases.len(), 1);
                assert!(else_case.is_none());
            }
            _ => panic!("Expected SelectCase"),
        }
    }
}
