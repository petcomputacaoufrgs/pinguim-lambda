pub mod position;
pub mod lexer;
pub mod parser;
pub mod error;

use lexer::generate_tokens;
use parser::parse;