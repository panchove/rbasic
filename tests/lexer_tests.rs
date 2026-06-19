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
        // The lexer stores the raw content; escapes are handled at codegen
        let (tokens, errors) = lex(r#""hello\nworld""#);
        assert!(errors.is_empty());
        assert!(matches!(&tokens[0].kind, TokenKind::StringLit(s) if s.contains("\\n")));
    }
}
