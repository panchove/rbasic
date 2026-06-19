#[derive(Debug)]
pub struct Scope {
    pub symbols: Vec<super::symbols::Symbol>,
    // parent scope optional for nesting
    pub parent: Option<Box<Scope>>,
}
