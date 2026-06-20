#[cfg(test)]
mod tests {
    use rbasic::lexer::{
        lex,
        token::{LexErrorCode, TokenKind},
    };

    #[test]
    fn test_lex_simple() {
        let (tokens, errors) = lex("LET x = 10");
        assert!(errors.is_empty());
        assert!(!tokens.is_empty());
    }

    #[test]
    fn comment_ignored() {
        let (tokens, errors) = lex("' this is a comment");
        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn comment_inline_ignored() {
        let (tokens, errors) = lex("LET x = 10 ' assign x");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Let));
        assert!(matches!(kinds[1], TokenKind::Identifier(s) if s == "x"));
        assert!(matches!(kinds[2], TokenKind::Assign));
        assert!(matches!(kinds[3], TokenKind::Int(10)));
        assert!(matches!(kinds[4], TokenKind::Eof));
        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn comment_does_not_consume_next_line() {
        let (tokens, errors) = lex("' comment\nPRINT 1");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Print));
        assert!(matches!(kinds[1], TokenKind::Int(1)));
        assert!(matches!(kinds[2], TokenKind::Eof));
    }

    // --- Lexical error tests ---

    #[test]
    fn invalid_char_produces_error() {
        let (tokens, errors) = lex("LET x = §");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, LexErrorCode::InvalidChar);
        assert!(errors[0].message.contains('§'));
        // Valid tokens before the bad char are still produced
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Let));
    }

    #[test]
    fn unterminated_string_produces_error() {
        let (tokens, errors) = lex("PRINT \"Hello");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, LexErrorCode::UnterminatedString);
        // PRINT token is still emitted
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Print));
    }

    #[test]
    fn invalid_number_produces_error() {
        let (_, errors) = lex("12.34.56");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, LexErrorCode::InvalidNumber);
    }

    #[test]
    fn multiple_errors_collected() {
        // Two bad chars in one line — both reported, not just the first
        let (_, errors) = lex("§ @");
        assert_eq!(errors.len(), 2);
        assert!(errors.iter().all(|e| e.code == LexErrorCode::InvalidChar));
    }

    #[test]
    fn valid_code_has_no_errors() {
        let src = "FUNCTION add(a: I32, b: I32) RETURNS I32\n    RETURN a + b\nEND FUNCTION";
        let (_, errors) = lex(src);
        assert!(errors.is_empty());
    }

    #[test]
    fn keywords_case_insensitive() {
        use rbasic::lexer::lex;
        let (lower, _) = lex("let mut function return if then else end while print");
        let (upper, _) = lex("LET MUT FUNCTION RETURN IF THEN ELSE END WHILE PRINT");
        let lower_kinds: Vec<_> = lower.iter().map(|t| &t.kind).collect();
        let upper_kinds: Vec<_> = upper.iter().map(|t| &t.kind).collect();
        assert_eq!(lower_kinds, upper_kinds);
    }

    #[test]
    fn lone_exclamation_is_invalid() {
        let (_, errors) = lex("LET x = !5");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, LexErrorCode::InvalidChar);
    }

    #[test]
    fn valid_float_literal() {
        let (tokens, errors) = lex("1.5");
        assert!(errors.is_empty());
        assert!(matches!(tokens[0].kind, TokenKind::Float(f) if (f - 1.5).abs() < 1e-9));
    }

    #[test]
    fn integer_literal() {
        let (tokens, errors) = lex("42");
        assert!(errors.is_empty());
        assert!(matches!(tokens[0].kind, TokenKind::Int(42)));
    }

    #[test]
    fn string_with_escapes() {
        // \n in source becomes an actual newline char in the stored token
        let (tokens, errors) = lex(r#""hello\nworld""#);
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::StringLit(s) if s.contains('\n')));
    }

    #[test]
    fn string_escape_tab() {
        let (tokens, errors) = lex(r#""col1\tcol2""#);
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::StringLit(s) if s.contains('\t')));
    }

    #[test]
    fn string_escape_backslash() {
        let (tokens, errors) = lex(r#""C:\\path""#);
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::StringLit(s) if s.contains('\\')));
    }

    #[test]
    fn string_escape_quote() {
        let (tokens, errors) = lex(r#""say \"hi\"""#);
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::StringLit(s) if s.contains('"')));
    }

    // ---- Bool literal ----

    #[test]
    fn bool_literal_true() {
        let (tokens, errors) = lex("TRUE");
        assert!(errors.is_empty());
        assert!(matches!(tokens[0].kind, TokenKind::Bool(true)));
    }

    #[test]
    fn bool_literal_false() {
        let (tokens, errors) = lex("FALSE");
        assert!(errors.is_empty());
        assert!(matches!(tokens[0].kind, TokenKind::Bool(false)));
    }

    #[test]
    fn bool_literal_case_insensitive() {
        let (tokens, _) = lex("true True TRUE false False FALSE");
        let bools: Vec<_> = tokens
            .iter()
            .filter_map(|t| match &t.kind {
                TokenKind::Bool(b) => Some(*b),
                _ => None,
            })
            .collect();
        assert_eq!(bools.len(), 6);
        assert_eq!(bools, vec![true, true, true, false, false, false]);
    }

    // ---- Extended keyword tokens ----

    fn assert_keyword(input: &str, expected: TokenKind) {
        let (tokens, errors) = lex(input);
        assert!(
            errors.is_empty(),
            "errors for input '{}': {:?}",
            input,
            errors
        );
        assert_eq!(tokens[0].kind, expected, "mismatch for input '{}'", input);
    }

    #[test]
    fn keyword_step() {
        assert_keyword("STEP", TokenKind::Step);
    }
    #[test]
    fn keyword_do() {
        assert_keyword("DO", TokenKind::Do);
    }
    #[test]
    fn keyword_loop() {
        assert_keyword("LOOP", TokenKind::Loop);
    }
    #[test]
    fn keyword_until() {
        assert_keyword("UNTIL", TokenKind::Until);
    }
    #[test]
    fn keyword_as() {
        assert_keyword("AS", TokenKind::As);
    }
    #[test]
    fn keyword_and() {
        assert_keyword("AND", TokenKind::And);
    }
    #[test]
    fn keyword_or() {
        assert_keyword("OR", TokenKind::Or);
    }
    #[test]
    fn keyword_xor() {
        assert_keyword("XOR", TokenKind::Xor);
    }
    #[test]
    fn keyword_dim() {
        assert_keyword("DIM", TokenKind::Dim);
    }
    #[test]
    fn keyword_on() {
        assert_keyword("ON", TokenKind::On);
    }
    #[test]
    fn keyword_error() {
        assert_keyword("ERROR", TokenKind::Error);
    }
    #[test]
    fn keyword_goto() {
        assert_keyword("GOTO", TokenKind::Goto);
    }
    #[test]
    fn keyword_resume() {
        assert_keyword("RESUME", TokenKind::Resume);
    }
    #[test]
    fn keyword_shl() {
        assert_keyword("SHL", TokenKind::Shl);
    }
    #[test]
    fn keyword_shr() {
        assert_keyword("SHR", TokenKind::Shr);
    }
    #[test]
    fn keyword_mod() {
        assert_keyword("MOD", TokenKind::Mod);
    }

    // ---- Extended operator tokens ----

    #[test]
    fn caret_operator() {
        let (tokens, errors) = lex("2 ^ 3");
        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Caret));
    }

    #[test]
    fn backslash_operator() {
        let (tokens, errors) = lex("7 \\ 3");
        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Backslash));
    }

    // ---- UTF-8 support ----

    #[test]
    fn utf8_source_encoding() {
        let (tokens, errors) = lex("PRINT \"café\" ' ñoño");
        assert!(errors.is_empty(), "UTF-8 source should not produce errors");
        assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::Print)));
        assert!(tokens
            .iter()
            .any(|t| matches!(&t.kind, TokenKind::StringLit(s) if s == "café")));
    }

    // ---- Reserved word rejection ----

    #[test]
    fn reserved_next_is_identifier() {
        // NEXT is reserved but tokenised as Identifier in v0.1
        let (tokens, errors) = lex("NEXT");
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::Identifier(s) if s == "NEXT"));
    }

    #[test]
    fn reserved_module_is_identifier() {
        let (tokens, errors) = lex("MODULE");
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::Identifier(s) if s == "MODULE"));
    }

    #[test]
    fn reserved_import_is_identifier() {
        let (tokens, errors) = lex("IMPORT");
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::Identifier(s) if s == "IMPORT"));
    }

    #[test]
    fn end_function_sequence() {
        let (tokens, errors) = lex("END FUNCTION");
        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::End));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Function));
    }

    #[test]
    fn end_if_sequence() {
        let (tokens, errors) = lex("END IF");
        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::End));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::If));
    }

    #[test]
    fn end_for_sequence() {
        let (tokens, errors) = lex("END FOR");
        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::End));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::For));
    }

    #[test]
    fn keyword_case_insensitive_extended() {
        let upper = "DO WHILE x < 10\nLOOP";
        let lower = "do while x < 10\nloop";
        let (tokens_upper, _) = lex(upper);
        let (tokens_lower, _) = lex(lower);
        assert_eq!(tokens_upper.len(), tokens_lower.len());
        for (u, l) in tokens_upper.iter().zip(tokens_lower.iter()) {
            assert_eq!(u.kind, l.kind, "mismatch: {:?} vs {:?}", u.kind, l.kind);
        }
    }

    #[test]
    fn identifier_with_trailing_dollar() {
        let (tokens, errors) = lex("name$ MID$ LEFT$");
        assert!(
            errors.is_empty(),
            "trailing $ should be valid: {:?}",
            errors
        );
        assert!(matches!(&tokens[0].kind, TokenKind::Identifier(s) if s == "name$"));
        assert!(matches!(&tokens[1].kind, TokenKind::Identifier(s) if s == "MID$"));
        assert!(matches!(&tokens[2].kind, TokenKind::Identifier(s) if s == "LEFT$"));
    }

    #[test]
    fn identifier_dollar_in_middle_is_error() {
        let (_tokens, errors) = lex("na$me");
        assert!(!errors.is_empty(), "should produce error for '$' in middle");
        assert_eq!(errors[0].code, LexErrorCode::InvalidChar);
        assert!(
            errors[0].message.contains("must be the last character"),
            "error message: {}",
            errors[0].message
        );
    }

    #[test]
    fn identifier_multiple_dollars_is_error() {
        let (_tokens, errors) = lex("foo$$");
        assert!(!errors.is_empty(), "should produce error for multiple '$'");
        assert_eq!(errors[0].code, LexErrorCode::InvalidChar);
    }

    #[test]
    fn identifier_dollar_then_content_is_error() {
        let (_tokens, errors) = lex("abc$def");
        assert!(
            !errors.is_empty(),
            "should produce error for '$def' after '$'"
        );
        assert_eq!(errors[0].code, LexErrorCode::InvalidChar);
    }

    #[test]
    fn identifier_with_underscore() {
        let (tokens, errors) = lex("_myVar counter_1");
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::Identifier(s) if s == "_myVar"));
        assert!(matches!(&tokens[1].kind, TokenKind::Identifier(s) if s == "counter_1"));
    }

    #[test]
    fn keyword_plus_equal() {
        let (tokens, errors) = lex("x += 1");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::PlusEqual);
    }

    #[test]
    fn keyword_minus_equal() {
        let (tokens, errors) = lex("x -= 1");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::MinusEqual);
    }

    #[test]
    fn keyword_star_equal() {
        let (tokens, errors) = lex("x *= 2");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::StarEqual);
    }

    #[test]
    fn keyword_slash_equal() {
        let (tokens, errors) = lex("x /= 2");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::SlashEqual);
    }

    #[test]
    fn keyword_backslash_equal() {
        let (tokens, errors) = lex("x \\= 2");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::BackslashEqual);
    }

    #[test]
    fn keyword_mod_equal() {
        let (tokens, errors) = lex("x MOD= 3");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::ModEqual);
    }

    #[test]
    fn plus_without_equal_is_plus() {
        let (tokens, errors) = lex("x + 1");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::Plus);
    }

    #[test]
    fn mod_without_equal_is_mod() {
        let (tokens, errors) = lex("x MOD 3");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::Mod);
    }
}
