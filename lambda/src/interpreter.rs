use crate::expr::{ArcExpr, Expression};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Default)]
pub struct DeBruijnSymbol {
    pub index: usize,
    pub rendered: String,
}

impl PartialEq for DeBruijnSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for DeBruijnSymbol {}

impl PartialOrd for DeBruijnSymbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DeBruijnSymbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl Hash for DeBruijnSymbol {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.index.hash(state)
    }
}

pub type RuntimeExpr = ArcExpr<DeBruijnSymbol>;

#[derive(Debug)]
enum Operation<'input> {
    Evaluate(&'input RuntimeExpr),
}

#[derive(Debug)]
struct Evaluator<'input> {
    operations: Vec<Operation<'input>>,
    output: Option<RuntimeExpr>,
    substitutions: Vec<RuntimeExpr>,
}

impl<'input> Evaluator<'input> {
    fn new(expression: &RuntimeExpr) -> Self {
        todo!()
    }

    fn run(&mut self) {
        todo!()
    }

    fn output(&mut self) -> Option<RuntimeExpr> {
        todo!()
    }
}

pub fn evaluate(expression: &RuntimeExpr) -> RuntimeExpr {
    let mut evaluator = Evaluator::new(expression);
    evaluator.run();
    evaluator.output().expect("evaluate has always pushed")
}
