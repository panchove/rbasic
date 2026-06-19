#[cfg(test)]
mod tests {
    // use pretty_assertions::assert_eq; // not needed

    #[test]
    fn test_lex_simple() {
        let input = "LET x = 10";
        let tokens = rbasic::lexer::lex(input);
        // Stub expectation, will be expanded later
        assert!(!tokens.is_empty());
    }
}
