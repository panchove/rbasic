pub mod analyzer;
pub mod errors;
pub mod scopes;
pub mod symbols;
pub mod types;

pub use analyzer::analyze;
pub use errors::SemanticErrorCode;
