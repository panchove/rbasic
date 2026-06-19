#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    // placeholder for kind
    pub kind: SymbolKind,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable,
    Function,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    // simple Vec for now
    pub symbols: Vec<Symbol>,
}
