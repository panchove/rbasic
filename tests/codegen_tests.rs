#[cfg(test)]
mod tests {
    use rbasic::{generate_rust, lex, Parser};

    fn compile(src: &str) -> String {
        let (tokens, _) = lex(src);
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("parse error");
        generate_rust(&prog)
    }

    #[test]
    fn test_print_literal() {
        let out = compile("PRINT 42");
        assert_eq!(out, "fn main() {\n    println!(\"{}\", 42);\n}\n");
    }

    #[test]
    fn test_print_string() {
        let out = compile("PRINT \"hello\"");
        assert_eq!(out, "fn main() {\n    println!(\"{}\", \"hello\");\n}\n");
    }

    #[test]
    fn test_let_mut() {
        let out = compile("LET MUT x: I32 = 0");
        assert!(out.contains("let mut x: i32 = 0;"));
    }

    #[test]
    fn test_let_immutable() {
        let out = compile("LET x: I32 = 0");
        assert!(out.contains("let x: i32 = 0;"));
        assert!(!out.contains("let mut"));
    }

    #[test]
    fn test_u8_decl() {
        let out = compile("LET x: U8 = 10");
        assert!(out.contains("let x: u8 = 10;"));
    }

    #[test]
    fn test_u16_decl() {
        let out = compile("LET x: U16 = 10");
        assert!(out.contains("let x: u16 = 10;"));
    }

    #[test]
    fn test_u32_decl() {
        let out = compile("LET x: U32 = 10");
        assert!(out.contains("let x: u32 = 10;"));
    }

    #[test]
    fn test_u64_decl() {
        let out = compile("LET x: U64 = 10");
        assert!(out.contains("let x: u64 = 10;"));
    }

    #[test]
    fn test_i8_decl() {
        let out = compile("LET x: I8 = 10");
        assert!(out.contains("let x: i8 = 10;"));
    }

    #[test]
    fn test_f32_decl() {
        let out = compile("LET x: F32 = 3.14");
        assert!(out.contains("let x: f32 = 3.14;"));
    }
    #[test]
    fn test_i16_decl() {
        let out = compile("LET x: I16 = 10");
        assert!(out.contains("let x: i16 = 10;"));
    }

    #[test]
    fn test_i64_decl() {
        let out = compile("LET x: I64 = 10");
        assert!(out.contains("let x: i64 = 10;"));
    }

    #[test]
    fn test_var_decl() {
        let out = compile("LET x: I32 = 10");
        assert!(out.contains("let x: i32 = 10;"));
    }

    #[test]
    fn test_var_decl_no_type() {
        let out = compile("LET x = 10");
        assert!(out.contains("let x = 10;"));
    }

    #[test]
    fn test_function() {
        let src = r#"
FUNCTION add(a: I32, b: I32) RETURNS I32
    RETURN a + b
END FUNCTION
"#;
        let out = compile(src);
        assert!(out.contains("fn add(a: i32, b: i32) -> i32 {"));
        assert!(out.contains("return a + b;"));
    }

    #[test]
    fn test_if_else() {
        let src = r#"
IF x > 0 THEN
    PRINT "pos"
ELSE
    PRINT "neg"
END IF
"#;
        let out = compile(src);
        assert!(out.contains("if x > 0 {"));
        assert!(out.contains("} else {"));
    }

    #[test]
    fn test_while() {
        let src = r#"
WHILE i < 10
    PRINT i
END WHILE
"#;
        let out = compile(src);
        assert!(out.contains("while i < 10 {"));
    }

    #[test]
    fn test_unary_minus() {
        let out = compile("PRINT -5");
        assert!(out.contains("-(5)"));
    }

    #[test]
    fn test_unary_not() {
        let out = compile("PRINT NOT flag");
        assert!(out.contains("!(flag)"));
    }

    #[test]
    fn test_binary_ops() {
        let out = compile("PRINT 1 + 2 * 3");
        assert!(out.contains("1 + 2 * 3"));
    }

    #[test]
    fn test_function_call() {
        let out = compile("add(1, 2)");
        assert!(out.contains("add(1, 2);"));
    }

    #[test]
    fn test_escape_string() {
        let out = compile("PRINT \"line1\nline2\"");
        assert!(out.contains("\\n"));
    }

    #[test]
    fn test_bool_literal() {
        let out = compile("PRINT TRUE");
        assert!(out.contains("true"));
    }

    #[test]
    fn test_pow() {
        let out = compile("PRINT 2 ^ 3");
        assert!(out.contains("as f64).powf(3 as f64)"));
    }

    #[test]
    fn test_mod() {
        let out = compile("PRINT 10 MOD 3");
        assert!(out.contains("10 % 3"));
    }

    #[test]
    fn test_intdiv() {
        let out = compile("PRINT 7 \\ 3");
        assert!(out.contains("7 / 3"));
    }

    #[test]
    fn test_for_loop() {
        let out = compile("FOR i = 1 TO 5\n    PRINT i\nEND FOR");
        assert!(out.contains("let mut i = 1;"));
        assert!(out.contains("while i <= 5"));
        assert!(out.contains("i += 1;"));
    }

    #[test]
    fn test_for_step() {
        let out = compile("FOR i = 1 TO 10 STEP 2\n    PRINT i\nEND FOR");
        assert!(out.contains("let mut i = 1;"));
        assert!(out.contains("let step = 2;"));
        assert!(out.contains("if step >= 0"));
        assert!(out.contains("while i <= 10"));
        assert!(out.contains("i += step;"));
    }

    #[test]
    fn test_for_neg_step() {
        let out = compile("FOR i = 10 TO 1 STEP -1\n    PRINT i\nEND FOR");
        assert!(out.contains("let step = "));
        assert!(out.contains("while i >= 1"));
        assert!(out.contains("i += step;"));
        assert!(out.contains("} else {"));
    }

    #[test]
    fn test_do_while_pre() {
        let out = compile("DO WHILE x < 10\n    PRINT x\nLOOP");
        assert!(out.contains("while x < 10 {"));
    }

    #[test]
    fn test_do_until_pre() {
        let out = compile("DO UNTIL done\n    PRINT \"waiting\"\nLOOP");
        assert!(out.contains("while !(done)"));
    }

    #[test]
    fn test_do_loop_while_post() {
        let out = compile("DO\n    PRINT x\nLOOP WHILE x < 10");
        assert!(out.contains("loop {"));
        assert!(out.contains("if !(x < 10) { break; }"));
    }

    #[test]
    fn test_do_loop_until_post() {
        let out = compile("DO\n    PRINT x\nLOOP UNTIL done");
        assert!(out.contains("loop {"));
        assert!(out.contains("if done { break; }"));
    }

    #[test]
    fn test_as_cast() {
        let out = compile("PRINT 42 AS I32");
        assert!(out.contains("42 as i32"));
    }
    #[test]
    fn test_as_cast_i64_to_u8() {
        let out = compile("PRINT 1000 AS U8");
        assert!(out.contains("1000 as u8"));
    }
    #[test]
    fn test_as_cast_f64_to_i32() {
        let out = compile("PRINT 3.14 AS I32");
        assert!(out.contains(" 3.14 as i32"));
    }
    #[test]
    fn test_as_cast_nested() {
        let out = compile("PRINT (100 AS I8) AS I32");
        assert!(out.contains("(100 as i8) as i32"));
    }
}
