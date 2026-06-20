#[cfg(test)]
mod tests {
    use rbasic::lexer::{lex, token::TokenKind};

    // --- OPEN statement tests ---

    #[test]
    fn test_lex_open_input() {
        let (tokens, errors) = lex("OPEN \"data.txt\" FOR INPUT AS #1");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Open));
        assert!(matches!(kinds[1], TokenKind::StringLit(s) if s == "data.txt"));
        assert!(matches!(kinds[2], TokenKind::For));
        assert!(matches!(kinds[3], TokenKind::Input));
        assert!(matches!(kinds[4], TokenKind::As));
        assert!(matches!(kinds[5], TokenKind::Hash));
        assert!(matches!(kinds[6], TokenKind::Int(1)));
    }

    #[test]
    fn test_lex_open_output() {
        let (tokens, errors) = lex("OPEN \"out.txt\" FOR OUTPUT AS #2");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Open));
        assert!(matches!(kinds[2], TokenKind::For));
        assert!(matches!(kinds[3], TokenKind::Identifier(s) if s.to_uppercase() == "OUTPUT"));
    }

    #[test]
    fn test_lex_open_random() {
        let (tokens, errors) = lex("OPEN \"data.dat\" FOR RANDOM AS #3 LEN = 128");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Open));
        assert!(matches!(kinds[3], TokenKind::Random));
    }

    #[test]
    fn test_lex_open_binary() {
        let (tokens, errors) = lex("OPEN \"image.bin\" FOR BINARY AS #4");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Open));
        assert!(matches!(kinds[3], TokenKind::Binary));
    }

    // --- CLOSE statement tests ---

    #[test]
    fn test_lex_close_single() {
        let (tokens, errors) = lex("CLOSE #1");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Close));
        assert!(matches!(kinds[1], TokenKind::Hash));
        assert!(matches!(kinds[2], TokenKind::Int(1)));
    }

    #[test]
    fn test_lex_close_multiple() {
        let (tokens, errors) = lex("CLOSE #1, #2, #3");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Close));
        assert!(matches!(kinds[1], TokenKind::Hash));
        assert!(matches!(kinds[2], TokenKind::Int(1)));
        assert!(matches!(kinds[3], TokenKind::Comma));
        assert!(matches!(kinds[4], TokenKind::Hash));
        assert!(matches!(kinds[5], TokenKind::Int(2)));
    }

    #[test]
    fn test_lex_close_all() {
        let (tokens, errors) = lex("CLOSE");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Close));
        assert!(matches!(kinds[1], TokenKind::Eof));
    }

    // --- INPUT# statement tests ---

    #[test]
    fn test_lex_input_hash() {
        let (tokens, errors) = lex("INPUT #1, name$");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Input));
        assert!(matches!(kinds[1], TokenKind::Hash));
        assert!(matches!(kinds[2], TokenKind::Int(1)));
        assert!(matches!(kinds[3], TokenKind::Comma));
        assert!(matches!(kinds[4], TokenKind::Identifier(s) if s == "name$"));
    }

    #[test]
    fn test_lex_input_hash_multiple() {
        let (tokens, errors) = lex("INPUT #1, id, score");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Input));
        assert!(matches!(kinds[1], TokenKind::Hash));
        assert!(matches!(kinds[4], TokenKind::Identifier(s) if s == "id"));
        assert!(matches!(kinds[6], TokenKind::Identifier(s) if s == "score"));
    }

    // --- PRINT# statement tests ---

    #[test]
    fn test_lex_print_hash() {
        let (tokens, errors) = lex("PRINT #1, \"Hello\"");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Print));
        assert!(matches!(kinds[1], TokenKind::Hash));
        assert!(matches!(kinds[2], TokenKind::Int(1)));
        assert!(matches!(kinds[3], TokenKind::Comma));
        assert!(matches!(kinds[4], TokenKind::StringLit(s) if s == "Hello"));
    }

    #[test]
    fn test_lex_print_hash_with_semicolon() {
        let (tokens, errors) = lex("PRINT #1, id; \" \"; name$");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Print));
        assert!(matches!(kinds[1], TokenKind::Hash));
        assert!(matches!(kinds[5], TokenKind::Semi));
    }

    // --- LINE INPUT# statement tests ---

    #[test]
    fn test_lex_line_input_hash() {
        let (tokens, errors) = lex("LINE INPUT #1, line$");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Line));
        assert!(matches!(kinds[1], TokenKind::Input));
        assert!(matches!(kinds[2], TokenKind::Hash));
        assert!(matches!(kinds[3], TokenKind::Int(1)));
        assert!(matches!(kinds[4], TokenKind::Comma));
        assert!(matches!(kinds[5], TokenKind::Identifier(s) if s == "line$"));
    }

    // --- File function tests ---

    #[test]
    fn test_lex_eof_function() {
        let (tokens, errors) = lex("PRINT EOF(1)");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Print));
        assert!(matches!(kinds[1], TokenKind::Identifier(s) if s.to_uppercase() == "EOF"));
    }

    #[test]
    fn test_lex_lof_function() {
        let (tokens, errors) = lex("PRINT LOF(1)");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Print));
        assert!(matches!(kinds[1], TokenKind::Identifier(s) if s.to_uppercase() == "LOF"));
    }

    #[test]
    fn test_lex_freefile() {
        let (tokens, errors) = lex("PRINT FREEFILE");
        assert!(errors.is_empty());
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Print));
        assert!(matches!(kinds[1], TokenKind::Identifier(s) if s.to_uppercase() == "FREEFILE"));
    }
}
