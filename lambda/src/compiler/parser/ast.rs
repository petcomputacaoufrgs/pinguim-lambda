use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Variable(String),
    Application { function: Box<Value>, argument: Box<Value> },
    Lambda { parameter: String, body: Box<Value> },
}

pub struct Program {
    main_expression: Value,
    expressions: IndexMap<String, Value>,
}
