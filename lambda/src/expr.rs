use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use std::mem::ManuallyDrop;

pub trait Symbol
where
    Self: Clone + Default + PartialEq + Eq + PartialOrd + Ord + Hash,
{
}

impl<S> Symbol for S where
    S: Clone + Default + PartialEq + Eq + PartialOrd + Ord + Hash
{
}

#[derive(Debug)]
pub enum Expr<S>
where
    S: Symbol,
{
    Var(S),
    App(Nested<S>, Nested<S>),
    Lam(S, Nested<S>),
}

impl<S> Default for Expr<S>
where
    S: Symbol,
{
    fn default() -> Self {
        Expr::Var(S::default())
    }
}

impl<S> Clone for Expr<S>
where
    S: Symbol,
{
    fn clone(&self) -> Self {
        enum Operation<'input, S>
        where
            S: Symbol,
        {
            Clone(&'input Expr<S>),
            CloneAppArg(&'input Expr<S>),
            MakeApp(Expr<S>),
            MakeLam(S),
        }

        let mut op_stack = vec![Operation::Clone(self)];
        let mut output = None;

        while let Some(operation) = op_stack.pop() {
            match operation {
                Operation::Clone(input) => match input {
                    Expr::Var(symbol) => {
                        let expr = Expr::Var(symbol.clone());
                        output = Some(expr);
                    }
                    Expr::App(fun, arg) => {
                        op_stack.push(Operation::CloneAppArg(arg.expr()));
                        op_stack.push(Operation::Clone(fun.expr()));
                    }
                    Expr::Lam(arg, body) => {
                        op_stack.push(Operation::MakeLam(arg.clone()));
                        op_stack.push(Operation::Clone(body.expr()));
                    }
                },

                Operation::CloneAppArg(arg) => {
                    let fun =
                        output.take().expect("cloning app arg requires fun");
                    op_stack.push(Operation::MakeApp(fun));
                    op_stack.push(Operation::Clone(arg));
                }

                Operation::MakeApp(fun) => {
                    let arg = output.take().expect("cloning app requires arg");
                    let expr = Expr::App(Nested::new(fun), Nested::new(arg));
                    output = Some(expr);
                }

                Operation::MakeLam(arg) => {
                    let body =
                        output.take().expect("cloning lam requires body");
                    let expr = Expr::Lam(arg, Nested::new(body));
                    output = Some(expr);
                }
            }
        }

        output.expect("clone always yield a result")
    }
}

impl<S> PartialEq for Expr<S>
where
    S: Symbol,
{
    fn eq(&self, other: &Self) -> bool {
        let mut equals = true;
        let mut pairs = vec![(self, other)];

        while let Some((left, right)) = pairs.pop().filter(|_| equals) {
            match (left, right) {
                (Expr::Var(symbol_left), Expr::Var(symbol_right)) => {
                    equals = symbol_left == symbol_right;
                }

                (
                    Expr::App(fun_left, arg_left),
                    Expr::App(fun_right, arg_right),
                ) => {
                    pairs.push((arg_left.expr(), arg_right.expr()));
                    pairs.push((fun_left.expr(), fun_right.expr()));
                }

                (
                    Expr::Lam(arg_left, body_left),
                    Expr::Lam(arg_right, body_right),
                ) => {
                    equals = arg_left == arg_right;
                    if equals {
                        pairs.push((body_left.expr(), body_right.expr()));
                    }
                }

                _ => equals = false,
            }
        }

        equals
    }
}

impl<S> Eq for Expr<S> where S: Symbol {}

impl<S> PartialOrd for Expr<S>
where
    S: Symbol,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> Ord for Expr<S>
where
    S: Symbol,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ordering = Ordering::Equal;
        let mut pairs = vec![(self, other)];

        while let Some((left, right)) = pairs.pop().filter(|_| ordering.is_eq())
        {
            match (left, right) {
                (Expr::Var(symbol_left), Expr::Var(symbol_right)) => {
                    ordering = symbol_left.cmp(symbol_right);
                }

                (
                    Expr::App(fun_left, arg_left),
                    Expr::App(fun_right, arg_right),
                ) => {
                    pairs.push((arg_left.expr(), arg_right.expr()));
                    pairs.push((fun_left.expr(), fun_right.expr()));
                }

                (
                    Expr::Lam(arg_left, body_left),
                    Expr::Lam(arg_right, body_right),
                ) => {
                    ordering = arg_left.cmp(&arg_right);
                    if ordering.is_eq() {
                        pairs.push((body_left.expr(), body_right.expr()));
                    }
                }

                (Expr::Var(_), _) => ordering = Ordering::Less,
                (Expr::App(_, _), Expr::Var(_)) => ordering = Ordering::Greater,
                (Expr::App(_, _), Expr::Lam(_, _)) => ordering = Ordering::Less,
                (Expr::Lam(_, _), _) => ordering = Ordering::Greater,
            }
        }

        ordering
    }
}

impl<S> Hash for Expr<S>
where
    S: Symbol,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut target_stack = vec![self];

        while let Some(expr) = target_stack.pop() {
            match expr {
                Expr::Var(var) => {
                    state.write_u8(0);
                    var.hash(state);
                }
                Expr::App(fun, arg) => {
                    state.write_u8(1);
                    target_stack.push(fun.expr());
                    target_stack.push(arg.expr());
                }
                Expr::Lam(_, body) => {
                    state.write_u8(2);
                    target_stack.push(body.expr())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Nested<S>
where
    S: Symbol,
{
    expr: ManuallyDrop<Box<Expr<S>>>,
}

impl<S> From<Expr<S>> for Nested<S>
where
    S: Symbol,
{
    fn from(expr: Expr<S>) -> Self {
        Self::new(expr)
    }
}

impl<S> Nested<S>
where
    S: Symbol,
{
    pub fn new(expr: Expr<S>) -> Self {
        Self { expr: ManuallyDrop::new(Box::new(expr)) }
    }

    pub fn expr(&self) -> &Expr<S> {
        &**self.expr
    }

    pub fn expr_mut(&mut self) -> &mut Expr<S> {
        &mut **self.expr
    }

    pub fn take_expr(&mut self) -> Expr<S> {
        mem::take(self.expr_mut())
    }

    pub fn into_expr(mut self) -> Expr<S> {
        self.take_expr()
    }
}

impl<S> Drop for Nested<S>
where
    S: Symbol,
{
    fn drop(&mut self) {
        let mut drop_stack = vec![self.take_expr()];

        while let Some(expr) = drop_stack.pop() {
            match expr {
                Expr::Var(_) => (),
                Expr::App(fun, arg) => {
                    drop_stack.push(fun.into_expr());
                    drop_stack.push(arg.into_expr());
                }
                Expr::Lam(_, body) => drop_stack.push(body.into_expr()),
            }
        }
    }
}
