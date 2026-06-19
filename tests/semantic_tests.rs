// tests/semantic_tests.rs
#[cfg(test)]
mod tests {

    use rbasic::{
        analyze, lexer,
        parser::{ast::Program, Parser},
    };

    fn parse(src: &str) -> Program {
        let tokens = lexer::lex(src);
        let mut parser = Parser::new(tokens);
        parser.parse_program().expect("parse error")
    }

    fn analyze_src(src: &str) -> Result<(), Vec<rbasic::semantic::errors::SemanticError>> {
        let prog = parse(src);
        analyze(&prog).map(|_| ())
    }

    #[test]
    fn valid_global_variable() {
        let src = "LET value: I32 = 10\nPRINT value";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn duplicate_global_variable() {
        let src = "LET value: I32 = 10\nLET value: I32 = 20";
        let err = analyze_src(src).unwrap_err();
        assert!(err.iter().any(|e| e.code == "E1002"));
    }

    #[test]
    fn duplicate_local_variable() {
        let src = "FUNCTION Test()\n    LET x: I32 = 1\n    LET x: I32 = 2\nEND FUNCTION";
        let err = analyze_src(src).unwrap_err();
        assert!(err.iter().any(|e| e.code == "E1002"));
    }

    #[test]
    fn duplicate_function() {
        let src = "FUNCTION Test()\nEND FUNCTION\n\nFUNCTION Test()\nEND FUNCTION";
        let err = analyze_src(src).unwrap_err();
        assert!(err.iter().any(|e| e.code == "E1004"));
    }

    #[test]
    fn duplicate_parameter() {
        let src = "FUNCTION Add(a: I32, a: I32)\nEND FUNCTION";
        let err = analyze_src(src).unwrap_err();
        assert!(err.iter().any(|e| e.code == "E1011"));
    }

    #[test]
    fn unknown_variable() {
        let src = "PRINT missing_variable";
        let err = analyze_src(src).unwrap_err();
        assert!(err.iter().any(|e| e.code == "E1001"));
    }

    #[test]
    fn unknown_function() {
        let src = "LET result: I32 = MissingFunction()";
        let err = analyze_src(src).unwrap_err();
        assert!(err.iter().any(|e| e.code == "E1003"));
    }

    #[test]
    fn valid_function_call() {
        let src = "FUNCTION Add(a: I32, b: I32)\nEND FUNCTION\n\nAdd(1, 2)";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn case_insensitive_variable_resolution() {
        let src = "LET Counter: I32 = 10\nPRINT counter";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn case_insensitive_function_resolution() {
        let src = "FUNCTION PrintValue()\nEND FUNCTION\n\nprintvalue()";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn parameter_resolution() {
        let src = "FUNCTION Echo(value: I32)\n    PRINT value\nEND FUNCTION";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn nested_scope_resolution() {
        let src =
            "LET GlobalValue: I32 = 1\n\nFUNCTION Test()\n    PRINT GlobalValue\nEND FUNCTION";
        assert!(analyze_src(src).is_ok());
    }
}
