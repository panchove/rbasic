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
        assert_eq!(
            out,
            "fn main() {\n    println!(\"{}\", \"hello\".to_string());\n}\n"
        );
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

    #[test]
    fn test_shl() {
        let out = compile("LET a: I32 = 1\nLET b: I32 = 1 SHL a");
        assert!(out.contains("<<"));
    }

    #[test]
    fn test_shr() {
        let out = compile("LET a: I32 = 8\nLET b: I32 = a SHR 1");
        assert!(out.contains(">>"));
    }

    #[test]
    fn test_xor() {
        let out = compile("LET a: I32 = 1\nLET b: I32 = a XOR 0");
        assert!(out.contains(" ^ "));
    }

    #[test]
    fn test_function_no_return_type() {
        let out = compile("FUNCTION greet()\n    PRINT \"hi\"\nEND FUNCTION");
        assert!(out.contains("fn greet()"));
        assert!(!out.contains("->"));
    }

    // ---- AND / OR codegen ----

    #[test]
    fn test_and_codegen() {
        let out = compile("LET a: BOOL = TRUE AND FALSE");
        assert!(out.contains(" && "));
    }

    #[test]
    fn test_or_codegen() {
        let out = compile("LET a: BOOL = TRUE OR FALSE");
        assert!(out.contains(" || "));
    }

    // ---- DIM / ON ERROR / RESUME do not crash codegen ----

    #[test]
    fn test_dim_no_codegen_crash() {
        let out = compile("DIM arr(10)");
        assert!(out.contains("fn main()"));
    }

    #[test]
    fn test_on_error_no_codegen_crash() {
        let out = compile("ON ERROR GOTO handler");
        assert!(out.contains("fn main()"));
    }

    #[test]
    fn test_resume_no_codegen_crash() {
        let out = compile("RESUME");
        assert!(out.contains("fn main()"));
    }

    #[test]
    fn test_resume_label_no_codegen_crash() {
        let out = compile("RESUME next");
        assert!(out.contains("fn main()"));
    }

    // ---- Multiple functions ----

    #[test]
    fn test_multiple_functions_codegen() {
        let out = compile(
            "FUNCTION a()\n    PRINT 1\nEND FUNCTION\nFUNCTION b()\n    PRINT 2\nEND FUNCTION",
        );
        assert!(out.contains("fn a()"));
        assert!(out.contains("fn b()"));
        assert!(out.contains("fn main()"));
    }

    // ---- DO UNTIL codegen ----

    #[test]
    fn test_do_until_pre_codegen() {
        let out = compile("DO UNTIL done\n    PRINT \"wait\"\nLOOP");
        assert!(out.contains("while !(done)"));
    }

    #[test]
    fn test_do_loop_until_post_codegen() {
        let out = compile("DO\n    PRINT x\nLOOP UNTIL done");
        assert!(out.contains("if done { break; }"));
    }

    // ---- Bool literal codegen ----

    #[test]
    fn test_bool_literal_false_codegen() {
        let out = compile("PRINT FALSE");
        assert!(out.contains("false"));
    }

    // ---- NOT unary codegen with expression ----

    #[test]
    fn test_not_expr_codegen() {
        let out = compile("PRINT NOT (x > 0)");
        assert!(out.contains("!(("));
        assert!(out.contains(" > 0"));
    }

    // ---- STRING type decl ----

    #[test]
    fn test_string_decl_codegen() {
        let out = compile("LET s: STRING = \"hello\"");
        assert!(out.contains("let s: String = \"hello\".to_string()"));
    }

    // ---- AS cast with type alias ----

    #[test]
    fn test_as_cast_alias_codegen() {
        let out = compile("LET a: INTEGER = 42 AS INTEGER");
        assert!(out.contains("42 as i32"));
    }

    // ---- Assignment (RFC-0015) ----

    #[test]
    fn test_assign_expr() {
        let out = compile("LET MUT x: I32 = 0\nx = 42");
        assert!(out.contains("let mut x: i32 = 0;"));
        assert!(out.contains("x = 42;"));
    }

    #[test]
    fn test_assign_string() {
        let out = compile("LET MUT s: STRING = \"a\"\ns = \"b\"");
        assert!(out.contains("s = \"b\".to_string()"));
    }

    #[test]
    fn test_assign_from_var() {
        let out = compile("LET MUT x: I32 = 0\nLET MUT y: I32 = 10\nx = y");
        assert!(out.contains("x = y;"));
    }

    // ---- DIM codegen (RFC-0016) ----

    #[test]
    fn test_dim_codegen_single() {
        let out = compile("DIM arr(10)");
        assert!(out.contains("let arr: Vec<i32> = vec![0i32; 11];"));
    }

    #[test]
    fn test_dim_codegen_multi() {
        let out = compile("DIM matrix(2, 3)");
        assert!(out.contains("Vec<Vec<i32>>"));
    }

    #[test]
    fn test_dim_codegen_with_as_f64() {
        let out = compile("DIM arr(10) AS F64");
        assert!(out.contains("vec![0.0f64; 11]"));
    }

    #[test]
    fn test_dim_codegen_with_as_u32() {
        let out = compile("DIM arr(10) AS U32");
        assert!(out.contains("vec![0u32; 11]"));
    }

    #[test]
    fn test_dim_codegen_multi_with_as_string() {
        let out = compile("DIM s(3) AS STRING");
        assert!(out.contains("Vec<String>"));
        assert!(out.contains("String::new()"));
    }

    // ---- Built-in String Functions (RFC-0017) ----

    #[test]
    fn test_builtin_len() {
        let out = compile("PRINT LEN(\"hello\")");
        assert!(out.contains("\"hello\".to_string().len() as i32"));
    }

    #[test]
    fn test_builtin_mid() {
        let out = compile("PRINT MID$(\"hello\", 2, 3)");
        assert!(out.contains("\"hello\".to_string().chars().skip((2 - 1) as usize).take(3 as usize).collect::<String>()"));
    }

    #[test]
    fn test_builtin_left() {
        let out = compile("PRINT LEFT$(\"hello\", 2)");
        assert!(out.contains("\"hello\".to_string().chars().take(2 as usize).collect::<String>()"));
    }

    #[test]
    fn test_builtin_right() {
        let out = compile("PRINT RIGHT$(\"hello\", 2)");
        assert!(out.contains(
            "rev().take(2 as usize).collect::<String>().chars().rev().collect::<String>()"
        ));
    }

    #[test]
    fn test_builtin_chr() {
        let out = compile("PRINT CHR$(65)");
        assert!(
            out.contains("char::from_u32(65 as u32).map(|c| c.to_string()).unwrap_or_default()")
        );
    }

    #[test]
    fn test_builtin_asc() {
        let out = compile("PRINT ASC(\"A\")");
        assert!(out.contains("\"A\".to_string().chars().next().map(|c| c as i32).unwrap_or(0)"));
    }

    #[test]
    fn test_builtin_instr_2arg() {
        let out = compile("PRINT INSTR(\"hello\", \"ll\")");
        assert!(out.contains("\"hello\".to_string().find(&\"ll\".to_string())"));
    }

    #[test]
    fn test_builtin_instr_3arg() {
        let out = compile("PRINT INSTR(1, \"hello\", \"ll\")");
        assert!(out.contains("\"hello\".to_string()[1 as usize..]"));
    }

    #[test]
    fn test_builtin_val() {
        let out = compile("PRINT VAL(\"42.5\")");
        assert!(out.contains("\"42.5\".to_string().trim().parse::<f64>().unwrap_or(0.0)"));
    }

    #[test]
    fn test_builtin_str() {
        let out = compile("PRINT STR$(42)");
        assert!(out.contains("42.to_string()"));
    }

    #[test]
    fn test_builtin_ucase() {
        let out = compile("PRINT UCASE$(\"hello\")");
        assert!(out.contains("\"hello\".to_string().to_uppercase()"));
    }

    #[test]
    fn test_builtin_lcase() {
        let out = compile("PRINT LCASE$(\"HELLO\")");
        assert!(out.contains("\"HELLO\".to_string().to_lowercase()"));
    }

    #[test]
    fn test_builtin_trim() {
        let out = compile("PRINT TRIM$(\"  hi  \")");
        assert!(out.contains("\"  hi  \".to_string().trim().to_string()"));
    }

    #[test]
    fn test_builtin_ltrim() {
        let out = compile("PRINT LTRIM$(\"  hi  \")");
        assert!(out.contains("\"  hi  \".to_string().trim_start().to_string()"));
    }

    #[test]
    fn test_builtin_rtrim() {
        let out = compile("PRINT RTRIM$(\"  hi  \")");
        assert!(out.contains("\"  hi  \".to_string().trim_end().to_string()"));
    }

    #[test]
    fn test_builtin_space() {
        let out = compile("PRINT SPACE$(3)");
        assert!(out.contains("\" \".repeat(3 as usize)"));
    }

    #[test]
    fn test_builtin_string() {
        let out = compile("PRINT STRING$(3, \"A\")");
        assert!(out.contains("\"A\".to_string().repeat(3 as usize)"));
    }

    #[test]
    fn test_builtin_nested() {
        let out = compile("PRINT LEFT$(UCASE$(\"hello\"), 3)");
        assert!(out.contains(".to_uppercase()"));
        assert!(out.contains(".chars().take(3 as usize).collect::<String>()"));
    }

    #[test]
    fn test_builtin_non_dollar_alias() {
        let out = compile("PRINT MID(\"hello\", 2, 3)");
        assert!(out.contains("chars().skip((2 - 1) as usize)"));
    }

    #[test]
    fn test_builtin_in_expr() {
        let out = compile("LET MUT x: I32 = LEN(\"hello\")");
        assert!(out.contains("let mut x: i32 = \"hello\".to_string().len() as i32;"));
    }

    #[test]
    fn test_compound_assign_add_codegen() {
        let out = compile("LET MUT x: I32 = 0\nx += 1");
        assert!(out.contains("x = x + 1;"));
    }

    #[test]
    fn test_compound_assign_sub_codegen() {
        let out = compile("LET MUT x: I32 = 10\nx -= 3");
        assert!(out.contains("x = x - 3;"));
    }

    #[test]
    fn test_compound_assign_mul_codegen() {
        let out = compile("LET MUT x: I32 = 4\nx *= 2");
        assert!(out.contains("x = x * 2;"));
    }

    #[test]
    fn test_compound_assign_div_codegen() {
        let out = compile("LET MUT x: I32 = 8\nx /= 2");
        assert!(out.contains("x = x / 2;"));
    }

    #[test]
    fn test_compound_assign_intdiv_codegen() {
        let out = compile("LET MUT x: I32 = 8\nx \\= 3");
        assert!(out.contains("x = x / 3;"));
    }

    #[test]
    fn test_compound_assign_mod_codegen() {
        let out = compile("LET MUT x: I32 = 8\nx MOD= 3");
        assert!(out.contains("x = x % 3;"));
    }

    #[test]
    fn test_compound_assign_string_codegen() {
        let out = compile("LET MUT s: STRING = \"hello\"\ns += \" world\"");
        assert!(out.contains("s = s + \" world\".to_string();"));
    }
}
