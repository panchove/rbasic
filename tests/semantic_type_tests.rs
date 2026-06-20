#[cfg(test)]
mod tests {

    use rbasic::{
        analyze, lexer,
        parser::{ast::Program, Parser},
    };

    fn parse(src: &str) -> Program {
        let (tokens, _) = lexer::lex(src);
        let mut parser = Parser::new(tokens);
        parser.parse_program().expect("parse error")
    }

    fn analyze_src(src: &str) -> Result<(), Vec<rbasic::semantic::errors::SemanticError>> {
        let prog = parse(src);
        analyze(&prog).map(|_| ())
    }

    fn has_error(src: &str, code: &'static str) -> bool {
        match analyze_src(src) {
            Ok(_) => false,
            Err(errors) => errors.iter().any(|e| e.code == code),
        }
    }

    // ---- Phase 1: Type name validation ----

    #[test]
    fn primitive_bool() {
        assert!(analyze_src("LET flag: BOOL = TRUE").is_ok());
    }
    #[test]
    fn primitive_i32() {
        assert!(analyze_src("LET num: I32 = 10").is_ok());
    }
    #[test]
    fn primitive_u8() {
        assert!(analyze_src("LET num: U8 = 10").is_ok());
    }
    #[test]
    fn primitive_u16() {
        assert!(analyze_src("LET num: U16 = 10").is_ok());
    }
    #[test]
    fn primitive_u32() {
        assert!(analyze_src("LET num: U32 = 10").is_ok());
    }
    #[test]
    fn primitive_u64() {
        assert!(analyze_src("LET num: U64 = 10").is_ok());
    }
    #[test]
    fn primitive_i8() {
        assert!(analyze_src("LET num: I8 = 10").is_ok());
    }
    #[test]
    fn primitive_i16() {
        assert!(analyze_src("LET num: I16 = 10").is_ok());
    }
    #[test]
    fn primitive_i64() {
        assert!(analyze_src("LET num: I64 = 10").is_ok());
    }
    #[test]
    fn primitive_f32() {
        assert!(analyze_src("LET val: F32 = 3.14").is_ok());
    }
    #[test]
    fn primitive_f64() {
        assert!(analyze_src("LET val: F64 = 3.14").is_ok());
    }
    #[test]
    fn primitive_string() {
        assert!(analyze_src("LET txt: STRING = \"hello\"").is_ok());
    }
    #[test]
    fn unknown_type_error() {
        assert!(has_error("LET x: VECTOR = 10", "E1010"));
    }

    // ---- Phase 3: Type mismatch (E1020) ----

    #[test]
    fn type_mismatch_var_decl() {
        assert!(has_error("LET x: I32 = TRUE", "E1020"));
    }

    #[test]
    fn type_mismatch_var_decl_string() {
        assert!(has_error("LET x: I32 = \"hello\"", "E1020"));
    }

    #[test]
    fn type_mismatch_var_decl_bool() {
        assert!(has_error("LET x: STRING = TRUE", "E1020"));
    }

    #[test]
    fn type_mismatch_argument() {
        let src = "FUNCTION foo(x: I32)\nEND FUNCTION\nfoo(TRUE)";
        assert!(has_error(src, "E1020"));
    }

    // ---- Phase 3: Invalid binary operation (E1021) ----

    #[test]
    fn binary_bool_plus_int() {
        assert!(has_error("PRINT TRUE + 1", "E1021"));
    }

    #[test]
    fn binary_string_plus_int() {
        assert!(has_error("PRINT \"hi\" + 1", "E1021"));
    }

    #[test]
    fn binary_bool_eq_int() {
        assert!(has_error("PRINT TRUE == 1", "E1021"));
    }

    #[test]
    fn binary_int_lt_bool() {
        assert!(has_error("PRINT 1 < TRUE", "E1021"));
    }

    // ---- Phase 3: Invalid unary operation (E1022) ----

    #[test]
    fn unary_neg_bool() {
        assert!(has_error("PRINT -TRUE", "E1022"));
    }

    #[test]
    fn unary_neg_string() {
        assert!(has_error("PRINT -\"hi\"", "E1022"));
    }

    #[test]
    fn unary_not_int() {
        assert!(has_error("PRINT NOT 42", "E1022"));
    }

    // ---- Phase 3: Argument count mismatch (E1030) ----

    #[test]
    fn too_few_arguments() {
        let src = "FUNCTION foo(a: I32, b: I32)\nEND FUNCTION\nfoo(1)";
        assert!(has_error(src, "E1030"));
    }

    #[test]
    fn too_many_arguments() {
        let src = "FUNCTION foo(a: I32)\nEND FUNCTION\nfoo(1, 2)";
        assert!(has_error(src, "E1030"));
    }

    // ---- Phase 3: Return type mismatch (E1031) ----

    #[test]
    fn return_type_mismatch() {
        let src = "FUNCTION foo() RETURNS I32\n    RETURN TRUE\nEND FUNCTION";
        assert!(has_error(src, "E1031"));
    }

    #[test]
    fn return_type_mismatch_string() {
        let src = "FUNCTION foo() RETURNS I32\n    RETURN \"hi\"\nEND FUNCTION";
        assert!(has_error(src, "E1031"));
    }

    // ---- Phase 3: Invalid condition type (E1032) ----

    #[test]
    fn if_condition_int() {
        assert!(has_error("IF 1 THEN\nEND IF", "E1032"));
    }

    #[test]
    fn if_condition_string() {
        assert!(has_error("IF \"hi\" THEN\nEND IF", "E1032"));
    }

    #[test]
    fn while_condition_int() {
        assert!(has_error("WHILE 1\nEND WHILE", "E1032"));
    }

    #[test]
    fn while_condition_float() {
        assert!(has_error("WHILE 3.14\nEND WHILE", "E1032"));
    }

    // ---- Phase 3: Return outside function (E1033) ----

    #[test]
    fn return_at_top_level() {
        assert!(has_error("RETURN 42", "E1033"));
    }

    // ---- Valid type checks ----

    #[test]
    fn valid_arithmetic_i32() {
        assert!(analyze_src("PRINT 1 + 2 * 3").is_ok());
    }

    #[test]
    fn valid_arithmetic_i64() {
        assert!(analyze_src("LET a: I64 = 1\nLET b: I64 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_u8_arithmetic() {
        assert!(analyze_src("LET a: U8 = 1\nLET b: U8 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_u16_arithmetic() {
        assert!(analyze_src("LET a: U16 = 1\nLET b: U16 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_u32_arithmetic() {
        assert!(analyze_src("LET a: U32 = 1\nLET b: U32 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_u64_arithmetic() {
        assert!(analyze_src("LET a: U64 = 1\nLET b: U64 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_mixed_unsigned_arithmetic() {
        assert!(analyze_src("LET a: U8 = 1\nLET b: U16 = 2\nPRINT a + b").is_ok());
        assert!(analyze_src("LET a: U16 = 1\nLET b: U32 = 2\nPRINT a + b").is_ok());
        assert!(analyze_src("LET a: U32 = 1\nLET b: U64 = 2\nPRINT a + b").is_ok());
        assert!(analyze_src("LET a: U8 = 1\nLET b: U64 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_u8_pow() {
        assert!(analyze_src("LET x: U8 = 2\nLET y: U8 = 3\nPRINT x ^ y").is_ok());
    }
    #[test]
    fn valid_u16_pow() {
        assert!(analyze_src("LET x: U16 = 2\nLET y: U16 = 3\nPRINT x ^ y").is_ok());
    }
    #[test]
    fn valid_u32_pow() {
        assert!(analyze_src("LET x: U32 = 2\nLET y: U32 = 3\nPRINT x ^ y").is_ok());
    }
    #[test]
    fn valid_u64_pow() {
        assert!(analyze_src("LET x: U64 = 2\nLET y: U64 = 3\nPRINT x ^ y").is_ok());
    }
    #[test]
    fn valid_u8_mod() {
        assert!(analyze_src("LET x: U8 = 10\nLET y: U8 = 3\nPRINT x MOD y").is_ok());
    }
    #[test]
    fn valid_u16_mod() {
        assert!(analyze_src("LET x: U16 = 10\nLET y: U16 = 3\nPRINT x MOD y").is_ok());
    }
    #[test]
    fn valid_u32_mod() {
        assert!(analyze_src("LET x: U32 = 10\nLET y: U32 = 3\nPRINT x MOD y").is_ok());
    }
    #[test]
    fn valid_u64_mod() {
        assert!(analyze_src("LET x: U64 = 10\nLET y: U64 = 3\nPRINT x MOD y").is_ok());
    }
    #[test]
    fn valid_u8_intdiv() {
        assert!(analyze_src("LET x: U8 = 7\nLET y: U8 = 3\nPRINT x \\ y").is_ok());
    }
    #[test]
    fn valid_u16_intdiv() {
        assert!(analyze_src("LET x: U16 = 7\nLET y: U16 = 3\nPRINT x \\ y").is_ok());
    }
    #[test]
    fn valid_u32_intdiv() {
        assert!(analyze_src("LET x: U32 = 7\nLET y: U32 = 3\nPRINT x \\ y").is_ok());
    }
    #[test]
    fn valid_u64_intdiv() {
        assert!(analyze_src("LET x: U64 = 7\nLET y: U64 = 3\nPRINT x \\ y").is_ok());
    }
    #[test]
    fn valid_u8_comparison() {
        assert!(analyze_src("LET a: U8 = 1\nLET b: U8 = 2\nPRINT a < b").is_ok());
    }
    #[test]
    fn valid_u16_comparison() {
        assert!(analyze_src("LET a: U16 = 1\nLET b: U16 = 2\nPRINT a < b").is_ok());
    }
    #[test]
    fn valid_u32_comparison() {
        assert!(analyze_src("LET a: U32 = 1\nLET b: U32 = 2\nPRINT a < b").is_ok());
    }
    #[test]
    fn valid_u64_comparison() {
        assert!(analyze_src("LET a: U64 = 1\nLET b: U64 = 2\nPRINT a < b").is_ok());
    }
    #[test]
    fn unsigned_neg_should_fail() {
        assert!(has_error("LET x: U8 = 5\nPRINT -x", "E1022"));
        assert!(has_error("LET x: U16 = 5\nPRINT -x", "E1022"));
        assert!(has_error("LET x: U32 = 5\nPRINT -x", "E1022"));
        assert!(has_error("LET x: U64 = 5\nPRINT -x", "E1022"));
    }
    #[test]
    fn signed_unsigned_mixed_should_fail() {
        assert!(has_error(
            "LET a: I8 = 1\nLET b: U8 = 2\nPRINT a + b",
            "E1021"
        ));
        assert!(has_error(
            "LET a: U8 = 1\nLET b: I8 = 2\nPRINT a + b",
            "E1021"
        ));
        assert!(has_error(
            "LET a: I32 = 1\nLET b: U32 = 2\nPRINT a + b",
            "E1021"
        ));
        assert!(has_error(
            "LET a: U64 = 1\nLET b: I64 = 2\nPRINT a + b",
            "E1021"
        ));
    }
    #[test]
    fn valid_i8_arithmetic() {
        assert!(analyze_src("LET a: I8 = 1\nLET b: I8 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_i16_arithmetic() {
        assert!(analyze_src("LET a: I16 = 1\nLET b: I16 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_mixed_int_arithmetic() {
        assert!(analyze_src("LET a: I8 = 1\nLET b: I16 = 2\nPRINT a + b").is_ok());
        assert!(analyze_src("LET a: I16 = 1\nLET b: I32 = 2\nPRINT a + b").is_ok());
        assert!(analyze_src("LET a: I32 = 1\nLET b: I64 = 2\nPRINT a + b").is_ok());
        assert!(analyze_src("LET a: I8 = 1\nLET b: I64 = 2\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_f32_arithmetic() {
        assert!(analyze_src("LET a: F32 = 1.5\nLET b: F32 = 2.5\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_f32_f64_mixed() {
        assert!(analyze_src("LET a: F32 = 1.5\nLET b: F64 = 2.5\nPRINT a + b").is_ok());
        assert!(analyze_src("LET a: F64 = 1.5\nLET b: F32 = 2.5\nPRINT a + b").is_ok());
    }
    #[test]
    fn valid_arithmetic_f64() {
        assert!(analyze_src("PRINT 1.5 + 2.5").is_ok());
    }

    #[test]
    fn valid_comparison_i32() {
        assert!(analyze_src("PRINT 1 < 2").is_ok());
    }

    #[test]
    fn valid_equality_bool() {
        assert!(analyze_src("PRINT TRUE == FALSE").is_ok());
    }

    #[test]
    fn valid_if_bool() {
        assert!(analyze_src("IF TRUE THEN\n    PRINT 1\nEND IF").is_ok());
    }

    // ---- Power (^) and Mod (MOD) ----

    #[test]
    fn valid_pow_i32() {
        assert!(analyze_src("PRINT 2 ^ 3").is_ok());
    }

    #[test]
    fn valid_i8_pow() {
        assert!(analyze_src("LET x: I8 = 2\nLET y: I8 = 3\nPRINT x ^ y").is_ok());
    }
    #[test]
    fn valid_i16_pow() {
        assert!(analyze_src("LET x: I16 = 2\nLET y: I16 = 3\nPRINT x ^ y").is_ok());
    }
    #[test]
    fn valid_i64_pow() {
        assert!(analyze_src("LET x: I64 = 2\nLET y: I64 = 3\nPRINT x ^ y").is_ok());
    }

    #[test]
    fn valid_i8_mod() {
        assert!(analyze_src("LET x: I8 = 10\nLET y: I8 = 3\nPRINT x MOD y").is_ok());
    }
    #[test]
    fn valid_i16_mod() {
        assert!(analyze_src("LET x: I16 = 10\nLET y: I16 = 3\nPRINT x MOD y").is_ok());
    }
    #[test]
    fn valid_i64_mod() {
        assert!(analyze_src("LET x: I64 = 10\nLET y: I64 = 3\nPRINT x MOD y").is_ok());
    }

    #[test]
    fn valid_i8_intdiv() {
        assert!(analyze_src("LET x: I8 = 7\nLET y: I8 = 3\nPRINT x \\ y").is_ok());
    }
    #[test]
    fn valid_i16_intdiv() {
        assert!(analyze_src("LET x: I16 = 7\nLET y: I16 = 3\nPRINT x \\ y").is_ok());
    }
    #[test]
    fn valid_i64_intdiv() {
        assert!(analyze_src("LET x: I64 = 7\nLET y: I64 = 3\nPRINT x \\ y").is_ok());
    }

    #[test]
    fn valid_i8_neg() {
        assert!(analyze_src("LET x: I8 = 5\nPRINT -x").is_ok());
    }
    #[test]
    fn valid_i16_neg() {
        assert!(analyze_src("LET x: I16 = 5\nPRINT -x").is_ok());
    }
    #[test]
    fn valid_f32_neg() {
        assert!(analyze_src("LET x: F32 = 5.5\nPRINT -x").is_ok());
    }
    #[test]
    fn valid_i64_neg() {
        assert!(analyze_src("LET x: I64 = 5\nPRINT -x").is_ok());
    }

    #[test]
    fn valid_f32_pow() {
        assert!(analyze_src("LET x: F32 = 2.0\nLET y: F32 = 3.0\nPRINT x ^ y").is_ok());
    }
    #[test]
    fn valid_f32_comparison() {
        assert!(analyze_src("LET a: F32 = 1.5\nLET b: F32 = 2.5\nPRINT a < b").is_ok());
    }
    #[test]
    fn valid_i8_comparison() {
        assert!(analyze_src("LET a: I8 = 1\nLET b: I8 = 2\nPRINT a < b").is_ok());
    }
    #[test]
    fn valid_i16_comparison() {
        assert!(analyze_src("LET a: I16 = 1\nLET b: I16 = 2\nPRINT a < b").is_ok());
    }
    #[test]
    fn valid_i64_comparison() {
        assert!(analyze_src("LET a: I64 = 1\nLET b: I64 = 2\nPRINT a < b").is_ok());
    }

    #[test]
    fn valid_pow_f64() {
        assert!(analyze_src("PRINT 2.0 ^ 3.0").is_ok());
    }

    #[test]
    fn valid_mod_i32() {
        assert!(analyze_src("PRINT 10 MOD 3").is_ok());
    }

    #[test]
    fn pow_type_mismatch() {
        assert!(has_error("PRINT 2 ^ TRUE", "E1021"));
    }

    #[test]
    fn mod_type_mismatch() {
        assert!(has_error("PRINT 10 MOD TRUE", "E1021"));
    }

    #[test]
    fn valid_intdiv_i32() {
        assert!(analyze_src("PRINT 7 \\ 3").is_ok());
    }

    #[test]
    fn intdiv_not_for_float() {
        assert!(has_error("PRINT 7.0 \\ 3.0", "E1021"));
    }

    #[test]
    fn mod_not_for_float() {
        assert!(has_error("PRINT 10.5 MOD 3.0", "E1021"));
    }

    #[test]
    fn valid_while_bool() {
        assert!(analyze_src("LET x: I32 = 0\nWHILE x < 10\nEND WHILE").is_ok());
    }

    #[test]
    fn valid_function_return_i32() {
        let src = "FUNCTION foo() RETURNS I32\n    RETURN 42\nEND FUNCTION";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn valid_function_return_string() {
        let src = "FUNCTION foo() RETURNS STRING\n    RETURN \"hi\"\nEND FUNCTION";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn valid_function_call_args() {
        let src =
            "FUNCTION add(a: I32, b: I32) RETURNS I32\n    RETURN a + b\nEND FUNCTION\nadd(1, 2)";
        assert!(analyze_src(src).is_ok());
    }

    #[test]
    fn valid_unary_neg() {
        assert!(analyze_src("PRINT -42").is_ok());
    }

    #[test]
    fn valid_unary_not() {
        assert!(analyze_src("PRINT NOT TRUE").is_ok());
    }

    // ---- FOR loop ----

    #[test]
    fn valid_for_loop() {
        assert!(analyze_src("FOR i = 1 TO 10\n    PRINT i\nEND FOR").is_ok());
    }

    #[test]
    fn for_variable_in_body() {
        assert!(analyze_src("FOR i = 1 TO 10\n    PRINT i\nEND FOR\nPRINT i").is_err());
    }

    #[test]
    fn for_bounds_type_mismatch() {
        assert!(has_error("FOR i = 1 TO TRUE\nEND FOR", "E1020"));
    }

    #[test]
    fn for_step_type_mismatch() {
        assert!(has_error("FOR i = 1 TO 10 STEP TRUE\nEND FOR", "E1034"));
    }

    #[test]
    fn for_step_bounds_mismatch() {
        // I32 bounds + F64 step is valid with integer-to-float widening
        assert!(analyze_src("FOR i = 1 TO 10 STEP 1.5\nEND FOR").is_ok());
    }

    #[test]
    fn valid_for_step() {
        assert!(analyze_src("FOR i = 1 TO 10 STEP 2\n    PRINT i\nEND FOR").is_ok());
    }

    #[test]
    fn valid_for_neg_step() {
        assert!(analyze_src("FOR i = 10 TO 1 STEP -1\n    PRINT i\nEND FOR").is_ok());
    }

    #[test]
    fn valid_for_step_float() {
        assert!(analyze_src("FOR x = 1.0 TO 10.0 STEP 0.5\n    PRINT x\nEND FOR").is_ok());
    }

    #[test]
    fn valid_do_while() {
        assert!(analyze_src("LET x: I32 = 0\nDO WHILE x < 10\n    PRINT x\nLOOP").is_ok());
    }

    #[test]
    fn valid_do_until() {
        assert!(
            analyze_src("LET done: BOOL = FALSE\nDO UNTIL done\n    PRINT \"wait\"\nLOOP").is_ok()
        );
    }

    #[test]
    fn valid_do_loop_while() {
        assert!(analyze_src("LET x: I32 = 0\nDO\n    PRINT x\nLOOP WHILE x < 10").is_ok());
    }

    #[test]
    fn valid_do_loop_until() {
        assert!(
            analyze_src("LET done: BOOL = FALSE\nDO\n    PRINT \"wait\"\nLOOP UNTIL done").is_ok()
        );
    }

    #[test]
    fn do_while_condition_type_mismatch() {
        assert!(has_error("DO WHILE 1\nLOOP", "E1032"));
    }

    #[test]
    fn do_until_condition_type_mismatch() {
        assert!(has_error("DO UNTIL 1\nLOOP", "E1032"));
    }

    #[test]
    fn do_loop_while_condition_type_mismatch() {
        assert!(has_error("DO\nLOOP WHILE 1", "E1032"));
    }

    #[test]
    fn do_loop_until_condition_type_mismatch() {
        assert!(has_error("DO\nLOOP UNTIL 1", "E1032"));
    }

    // ---- AS cast ----

    #[test]
    fn as_cast_i32_to_i8() {
        assert!(analyze_src("LET a: I8 = 100 AS I8").is_ok());
    }
    #[test]
    fn as_cast_i32_to_u32() {
        assert!(analyze_src("LET a: U32 = 10 AS U32").is_ok());
    }
    #[test]
    fn as_cast_i32_to_f32() {
        assert!(analyze_src("LET a: F32 = 42 AS F32").is_ok());
    }
    #[test]
    fn as_cast_f64_to_i32() {
        assert!(analyze_src("LET a: I32 = 3.14 AS I32").is_ok());
    }
    #[test]
    fn as_cast_i64_to_u8() {
        assert!(analyze_src("LET a: U8 = 255 AS U8").is_ok());
    }
    #[test]
    fn as_cast_bool_should_fail() {
        assert!(has_error("LET a: I32 = TRUE AS I32", "E1020"));
    }
    #[test]
    fn logical_and() {
        assert!(analyze_src("LET a: BOOL = TRUE AND FALSE").is_ok());
    }
    #[test]
    fn logical_or() {
        assert!(analyze_src("LET a: BOOL = TRUE OR FALSE").is_ok());
    }
    #[test]
    fn logical_xor() {
        assert!(analyze_src("LET a: BOOL = TRUE XOR FALSE").is_ok());
    }
    #[test]
    fn string_concat() {
        assert!(analyze_src("LET s: STRING = \"hi\" + \"!\"").is_ok());
    }

    // ---- RFC-0011: Classic BASIC type aliases ----

    #[test]
    fn alias_boolean() {
        assert!(analyze_src("LET f: BOOLEAN = TRUE").is_ok());
    }
    #[test]
    fn alias_byte() {
        assert!(analyze_src("LET b: BYTE = 10").is_ok());
    }
    #[test]
    fn alias_word() {
        assert!(analyze_src("LET w: WORD = 1024").is_ok());
    }
    #[test]
    fn alias_dword() {
        assert!(analyze_src("LET dw: DWORD = 65535").is_ok());
    }
    #[test]
    fn alias_qword() {
        assert!(analyze_src("LET qw: QWORD = 4294967295").is_ok());
    }
    #[test]
    fn alias_integer() {
        assert!(analyze_src("LET n: INTEGER = 42").is_ok());
    }
    #[test]
    fn alias_long() {
        assert!(analyze_src("LET n: LONG = 9999999").is_ok());
    }
    #[test]
    fn alias_longlong() {
        assert!(analyze_src("LET n: LONGLONG = 9999999999").is_ok());
    }
    #[test]
    fn alias_single() {
        assert!(analyze_src("LET x: SINGLE = 1.5").is_ok());
    }
    #[test]
    fn alias_double() {
        assert!(analyze_src("LET x: DOUBLE = 3.14").is_ok());
    }
    #[test]
    fn alias_case_insensitive() {
        assert!(analyze_src("LET n: integer = 1").is_ok());
        assert!(analyze_src("LET n: Double = 1.0").is_ok());
        assert!(analyze_src("LET f: Boolean = TRUE").is_ok());
    }
    #[test]
    fn alias_in_function_param() {
        let src =
            "FUNCTION Add(a: INTEGER, b: INTEGER) RETURNS INTEGER\n    RETURN a + b\nEND FUNCTION";
        assert!(analyze_src(src).is_ok());
    }
    #[test]
    fn alias_in_cast() {
        assert!(analyze_src("PRINT 3.14 AS INTEGER").is_ok());
    }
    #[test]
    fn alias_unknown_still_fails() {
        assert!(has_error("LET x: VARIANT = 0", "E1010"));
    }

    // ---- Assignment (RFC-0015) ----

    #[test]
    fn assign_to_mutable_ok() {
        assert!(analyze_src("LET MUT x: I32 = 0\nx = 42").is_ok());
    }

    #[test]
    fn assign_to_immutable_fails() {
        assert!(has_error("LET x: I32 = 0\nx = 42", "E1042"));
    }

    #[test]
    fn assign_to_undeclared_fails() {
        assert!(has_error("x = 42", "E1040"));
    }

    #[test]
    fn assign_type_mismatch_fails() {
        assert!(has_error("LET MUT x: I32 = 0\nx = TRUE", "E1041"));
    }

    #[test]
    fn assign_string_type_mismatch_fails() {
        assert!(has_error("LET MUT s: STRING = \"hi\"\ns = 42", "E1041"));
    }

    #[test]
    fn assign_for_loop_var_ok() {
        assert!(analyze_src("FOR i = 1 TO 5\n    i = i + 1\nEND FOR").is_ok());
    }

    #[test]
    fn assign_param_ok() {
        let src = "FUNCTION foo(x: I32)\n    x = 10\nEND FUNCTION";
        assert!(analyze_src(src).is_ok());
    }

    // ---- DIM array (RFC-0016) ----

    #[test]
    fn dim_semantic_register() {
        assert!(analyze_src("DIM arr(10)").is_ok());
    }

    #[test]
    fn dim_with_as_type_f64() {
        assert!(analyze_src("DIM arr(10) AS F64").is_ok());
    }

    #[test]
    fn dim_with_as_type_string() {
        assert!(analyze_src("DIM s(5) AS STRING").is_ok());
    }

    #[test]
    fn dim_duplicate_emits_e1002() {
        assert!(has_error("DIM a(10)\nDIM a(20)", "E1002"));
    }

    #[test]
    fn dim_duplicate_not_e1003() {
        // Regression: duplicate DIM should NOT emit E1003
        assert!(!has_error("DIM a(10)\nDIM a(20)", "E1003"));
    }

    // ---- Built-in String Functions (RFC-0017) ----

    #[test]
    fn builtin_len_ok() {
        assert!(analyze_src("LET MUT s: STRING = \"hi\"\nPRINT LEN(s)").is_ok());
    }

    #[test]
    fn builtin_len_wrong_type_fails() {
        assert!(has_error("PRINT LEN(42)", "E1020"));
    }

    #[test]
    fn builtin_mid_ok() {
        assert!(analyze_src("PRINT MID$(\"hello\", 2, 3)").is_ok());
    }

    #[test]
    fn builtin_mid_wrong_args_fails() {
        assert!(has_error("PRINT MID$(\"hello\", 2)", "E1030"));
    }

    #[test]
    fn builtin_left_ok() {
        assert!(analyze_src("PRINT LEFT$(\"hello\", 2)").is_ok());
    }

    #[test]
    fn builtin_right_ok() {
        assert!(analyze_src("PRINT RIGHT$(\"hello\", 2)").is_ok());
    }

    #[test]
    fn builtin_chr_ok() {
        assert!(analyze_src("PRINT CHR$(65)").is_ok());
    }

    #[test]
    fn builtin_chr_wrong_type_fails() {
        assert!(has_error("PRINT CHR$(\"hello\")", "E1020"));
    }

    #[test]
    fn builtin_asc_ok() {
        assert!(analyze_src("PRINT ASC(\"A\")").is_ok());
    }

    #[test]
    fn builtin_asc_wrong_type_fails() {
        assert!(has_error("PRINT ASC(42)", "E1020"));
    }

    #[test]
    fn builtin_instr_2arg_ok() {
        assert!(analyze_src("PRINT INSTR(\"hello\", \"ll\")").is_ok());
    }

    #[test]
    fn builtin_instr_3arg_ok() {
        assert!(analyze_src("PRINT INSTR(1, \"hello\", \"ll\")").is_ok());
    }

    #[test]
    fn builtin_instr_wrong_args_fails() {
        assert!(has_error("PRINT INSTR(\"hello\")", "E1030"));
    }

    #[test]
    fn builtin_val_ok() {
        assert!(analyze_src("PRINT VAL(\"42.5\")").is_ok());
    }

    #[test]
    fn builtin_str_ok() {
        assert!(analyze_src("PRINT STR$(42)").is_ok());
    }

    #[test]
    fn builtin_ucase_ok() {
        assert!(analyze_src("PRINT UCASE$(\"hello\")").is_ok());
    }

    #[test]
    fn builtin_lcase_ok() {
        assert!(analyze_src("PRINT LCASE$(\"HELLO\")").is_ok());
    }

    #[test]
    fn builtin_trim_ok() {
        assert!(analyze_src("PRINT TRIM$(\"  hi  \")").is_ok());
    }

    #[test]
    fn builtin_ltrim_ok() {
        assert!(analyze_src("PRINT LTRIM$(\"  hi  \")").is_ok());
    }

    #[test]
    fn builtin_rtrim_ok() {
        assert!(analyze_src("PRINT RTRIM$(\"  hi  \")").is_ok());
    }

    #[test]
    fn builtin_space_ok() {
        assert!(analyze_src("PRINT SPACE$(3)").is_ok());
    }

    #[test]
    fn builtin_string_ok() {
        assert!(analyze_src("PRINT STRING$(3, \"A\")").is_ok());
    }

    #[test]
    fn builtin_nested_ok() {
        assert!(analyze_src("PRINT LEFT$(UCASE$(\"hello\"), 3)").is_ok());
    }

    #[test]
    fn builtin_non_dollar_alias_ok() {
        assert!(analyze_src("PRINT MID(\"hello\", 2, 3)").is_ok());
    }

    #[test]
    fn builtin_unknown_fails() {
        assert!(has_error("PRINT FOO$(\"hello\")", "E1003"));
    }

    // ---- RFC-0018: Compound Assignment Operators ----

    #[test]
    fn compound_assign_add_ok() {
        assert!(analyze_src("LET MUT x: I32 = 0\nx += 1").is_ok());
    }

    #[test]
    fn compound_assign_sub_ok() {
        assert!(analyze_src("LET MUT x: I32 = 10\nx -= 3").is_ok());
    }

    #[test]
    fn compound_assign_mul_ok() {
        assert!(analyze_src("LET MUT x: I32 = 4\nx *= 2").is_ok());
    }

    #[test]
    fn compound_assign_div_ok() {
        assert!(analyze_src("LET MUT x: I32 = 8\nx /= 2").is_ok());
    }

    #[test]
    fn compound_assign_intdiv_ok() {
        assert!(analyze_src("LET MUT x: I32 = 8\nx \\= 3").is_ok());
    }

    #[test]
    fn compound_assign_mod_ok() {
        assert!(analyze_src("LET MUT x: I32 = 8\nx MOD= 3").is_ok());
    }

    #[test]
    fn compound_assign_string_concat_ok() {
        assert!(analyze_src("LET MUT s: STRING = \"hello\"\ns += \" world\"").is_ok());
    }

    #[test]
    fn compound_assign_undeclared_fails() {
        assert!(has_error("x += 1", "E1043"));
    }

    #[test]
    fn compound_assign_immutable_fails() {
        assert!(has_error("LET x: I32 = 0\nx += 1", "E1044"));
    }

    #[test]
    fn compound_assign_string_plus_int_fails() {
        assert!(has_error("LET MUT s: STRING = \"hello\"\ns += 1", "E1045"));
    }

    #[test]
    fn compound_assign_float_intdiv_fails() {
        assert!(has_error("LET MUT x: F64 = 8.0\nx \\= 2", "E1045"));
    }

    // ---- RFC-0019: INPUT semantic rules ----

    #[test]
    fn input_valid_string() {
        assert!(analyze_src("LET MUT s: STRING = \"\"\nINPUT s").is_ok());
    }

    #[test]
    fn input_valid_integer() {
        assert!(analyze_src("LET MUT n: I32 = 0\nINPUT n").is_ok());
    }

    #[test]
    fn input_valid_with_prompt() {
        assert!(analyze_src("LET MUT n: I32 = 0\nINPUT \"Enter n: \", n").is_ok());
    }

    #[test]
    fn input_undeclared_target_fails() {
        assert!(has_error("INPUT x", "E1050"));
    }

    #[test]
    fn input_immutable_target_fails() {
        assert!(has_error("LET x: I32 = 0\nINPUT x", "E1051"));
    }

    #[test]
    fn input_unsupported_type_fails() {
        assert!(has_error("LET MUT x: I16 = 0\nINPUT x", "E1052"));
    }
}
