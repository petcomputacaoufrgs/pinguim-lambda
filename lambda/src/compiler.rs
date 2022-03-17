pub mod lexer;
pub mod parser;
pub mod expansor;

use lexer::generate_tokens;
use parser::parse;
use pinguim_language::error::{Diagnostics, Error};
use pinguim_language::position::Span;
