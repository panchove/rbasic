#[cfg(test)]
mod tests {
    use rbasic::codegen::rust::generate_rust;
    use rbasic::lexer::lex;
    use rbasic::parser::Parser;

    fn compile(input: &str) -> String {
        let (tokens, errors) = lex(input);
        assert!(errors.is_empty(), "Lexer errors: {:?}", errors);
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().expect("Parse error");
        generate_rust(&program)
    }

    // --- OPEN statement codegen tests ---

    #[test]
    fn test_codegen_open_input() {
        let output = compile("OPEN \"data.txt\" FOR INPUT AS #1");
        assert!(output.contains("rbasic::runtime::file::open"));
        assert!(output.contains("data.txt"));
        assert!(output.contains("FileMode::Input"));
    }

    #[test]
    fn test_codegen_open_output() {
        let output = compile("OPEN \"out.txt\" FOR OUTPUT AS #2");
        assert!(output.contains("FileMode::Output"));
    }

    #[test]
    fn test_codegen_open_random_with_len() {
        let output = compile("OPEN \"data.dat\" FOR RANDOM AS #3 LEN = 128");
        assert!(output.contains("FileMode::Random"));
        assert!(output.contains("Some(128)"));
    }

    // --- CLOSE statement codegen tests ---

    #[test]
    fn test_codegen_close_single() {
        let output = compile("CLOSE #1");
        assert!(output.contains("rbasic::runtime::file::close(1)"));
    }

    #[test]
    fn test_codegen_close_multiple() {
        let output = compile("CLOSE #1, #2, #3");
        assert!(output.contains("rbasic::runtime::file::close(1)"));
        assert!(output.contains("rbasic::runtime::file::close(2)"));
        assert!(output.contains("rbasic::runtime::file::close(3)"));
    }

    #[test]
    fn test_codegen_close_all() {
        let output = compile("CLOSE");
        assert!(output.contains("rbasic::runtime::file::close_all()"));
    }

    // --- INPUT# statement codegen tests ---

    #[test]
    fn test_codegen_input_hash() {
        let output = compile("LET name$ = \"\"\nINPUT #1, name$");
        assert!(output.contains("rbasic::runtime::file::input_hash(1)"));
        assert!(output.contains("name$ ="));
    }

    // --- PRINT# statement codegen tests ---

    #[test]
    fn test_codegen_print_hash() {
        let output = compile("PRINT #1, \"Hello\"");
        assert!(output.contains("rbasic::runtime::file::print_hash"));
        assert!(output.contains("Hello"));
    }

    // --- LINE INPUT# statement codegen tests ---

    #[test]
    fn test_codegen_line_input_hash() {
        let output = compile("LET line$ = \"\"\nLINE INPUT #1, line$");
        assert!(output.contains("rbasic::runtime::file::line_input_hash(1)"));
        assert!(output.contains("line$ ="));
    }

    // --- Complete program codegen test ---

    #[test]
    fn test_codegen_complete_file_io() {
        let output = compile(
            r#"
OPEN "data.txt" FOR INPUT AS #1
OPEN "output.txt" FOR OUTPUT AS #2
LET name$ = ""
INPUT #1, name$
PRINT #2, name$
CLOSE #1, #2
"#,
        );
        assert!(output.contains("rbasic::runtime::file::open"));
        assert!(output.contains("rbasic::runtime::file::close"));
        assert!(output.contains("rbasic::runtime::file::input_hash"));
        assert!(output.contains("rbasic::runtime::file::print_hash"));
    }
}
