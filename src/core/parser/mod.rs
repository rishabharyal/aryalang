pub mod ast;
pub mod definition;
mod expression_handler;
mod if_statement_handler;
mod let_statement_handler;
mod statements_handler;

pub use definition::Parser;
