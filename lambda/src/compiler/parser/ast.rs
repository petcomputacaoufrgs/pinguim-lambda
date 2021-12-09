#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum value {
    Variable(String),
    Application { function: Box<Value>, argument: Box<Value> },
    Lambda { parameter: String, body: Box<Value> },
}

