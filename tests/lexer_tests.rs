#[cfg(test)]
mod tests {
    use rbasic::lexer::{lex, token::TokenKind};

    #[test]
    fn test_lex_simple() {
        let input = "LET x = 10";
        let tokens = lex(input);
        assert!(!tokens.is_empty());
    }

    #[test]
    fn comment_ignored() {
        // A full-line comment produces only EOF
        let tokens = lex("' this is a comment");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn comment_inline_ignored() {
        // Tokens before the comment are kept; the comment is discarded
        let tokens = lex("LET x = 10 ' assign x");
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Let));
        assert!(matches!(kinds[1], TokenKind::Identifier(s) if s == "x"));
        assert!(matches!(kinds[2], TokenKind::Assign));
        assert!(matches!(kinds[3], TokenKind::Int(10)));
        assert!(matches!(kinds[4], TokenKind::Eof));
        assert_eq!(tokens.len(), 5); // no extra tokens from the comment
    }

    #[test]
    fn comment_does_not_consume_next_line() {
        // Code after the commented line must still be lexed
        let tokens = lex("' comment\nPRINT 1");
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Print));
        assert!(matches!(kinds[1], TokenKind::Int(1)));
        assert!(matches!(kinds[2], TokenKind::Eof));
    }
}
