use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::sync::Arc;

pub trait Symbol
where
    Self: Clone + Default + PartialEq + Eq + PartialOrd + Ord + Hash,
{
}

impl<S> Symbol for S where
    S: Clone + Default + PartialEq + Eq + PartialOrd + Ord + Hash
{
}

pub trait Expr: Sized {
    type Symbol: Symbol;

    fn from_data(data: ExprData<Self>) -> Self;

    fn data(&self) -> &ExprData<Self>;

    fn try_data_mut(&mut self) -> Option<&mut ExprData<Self>>;

    fn try_take_data(&mut self) -> Option<ExprData<Self>>;

    fn try_into_data(mut self) -> Result<ExprData<Self>, Self> {
        self.try_take_data().ok_or(self)
    }

    fn drop_in_place(&mut self) {
        let mut drop_stack = Vec::new();

        drop_stack.extend(self.try_take_data());

        while let Some(expr) = drop_stack.pop() {
            match expr {
                ExprData::Var(_) => (),
                ExprData::App(fun, arg) => {
                    drop_stack.extend(fun.try_into_data());
                    drop_stack.extend(arg.try_into_data());
                }
                ExprData::Lam(_, body) => {
                    drop_stack.extend(body.try_into_data())
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ExprData<E>
where
    E: Expr,
{
    Var(E::Symbol),
    App(E, E),
    Lam(E::Symbol, E),
}

impl<E> Default for ExprData<E>
where
    E: Expr,
{
    fn default() -> Self {
        ExprData::Var(E::Symbol::default())
    }
}

impl<E> ExprData<E>
where
    E: Expr,
{
    pub fn shallow_clone(&self) -> Self
    where
        E: Clone,
    {
        match self {
            ExprData::Var(symbol) => ExprData::Var(symbol.clone()),
            ExprData::App(fun, arg) => ExprData::App(fun.clone(), arg.clone()),
            ExprData::Lam(arg, body) => {
                ExprData::Lam(arg.clone(), body.clone())
            }
        }
    }

    pub fn deep_clone(&self) -> Self {
        enum Operation<'input, E>
        where
            E: Expr,
        {
            Clone(&'input ExprData<E>),
            CloneAppArg(&'input ExprData<E>),
            MakeApp(ExprData<E>),
            MakeLam(E::Symbol),
        }

        let mut op_stack = vec![Operation::Clone(self)];
        let mut output = None;

        while let Some(operation) = op_stack.pop() {
            match operation {
                Operation::Clone(input) => match input {
                    ExprData::Var(symbol) => {
                        let expr = ExprData::Var(symbol.clone());
                        output = Some(expr);
                    }
                    ExprData::App(fun, arg) => {
                        op_stack.push(Operation::CloneAppArg(arg.data()));
                        op_stack.push(Operation::Clone(fun.data()));
                    }
                    ExprData::Lam(arg, body) => {
                        op_stack.push(Operation::MakeLam(arg.clone()));
                        op_stack.push(Operation::Clone(body.data()));
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
                    let expr =
                        ExprData::App(E::from_data(fun), E::from_data(arg));
                    output = Some(expr);
                }

                Operation::MakeLam(arg) => {
                    let body =
                        output.take().expect("cloning lam requires body");
                    let expr = ExprData::Lam(arg, E::from_data(body));
                    output = Some(expr);
                }
            }
        }

        output.expect("clone always yield a result")
    }
}

impl<E> Clone for ExprData<E>
where
    E: Expr,
{
    fn clone(&self) -> Self {
        self.deep_clone()
    }
}

impl<E> PartialEq for ExprData<E>
where
    E: Expr,
{
    fn eq(&self, other: &Self) -> bool {
        let mut equals = true;
        let mut pairs = vec![(self, other)];

        while let Some((left, right)) = pairs.pop().filter(|_| equals) {
            match (left, right) {
                (ExprData::Var(symbol_left), ExprData::Var(symbol_right)) => {
                    equals = symbol_left == symbol_right;
                }

                (
                    ExprData::App(fun_left, arg_left),
                    ExprData::App(fun_right, arg_right),
                ) => {
                    pairs.push((arg_left.data(), arg_right.data()));
                    pairs.push((fun_left.data(), fun_right.data()));
                }

                (
                    ExprData::Lam(arg_left, body_left),
                    ExprData::Lam(arg_right, body_right),
                ) => {
                    equals = arg_left == arg_right;
                    if equals {
                        pairs.push((body_left.data(), body_right.data()));
                    }
                }

                _ => equals = false,
            }
        }

        equals
    }
}

impl<E> Eq for ExprData<E> where E: Expr {}

impl<E> PartialOrd for ExprData<E>
where
    E: Expr,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E> Ord for ExprData<E>
where
    E: Expr,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ordering = Ordering::Equal;
        let mut pairs = vec![(self, other)];

        while let Some((left, right)) = pairs.pop().filter(|_| ordering.is_eq())
        {
            match (left, right) {
                (ExprData::Var(symbol_left), ExprData::Var(symbol_right)) => {
                    ordering = symbol_left.cmp(symbol_right);
                }

                (
                    ExprData::App(fun_left, arg_left),
                    ExprData::App(fun_right, arg_right),
                ) => {
                    pairs.push((arg_left.data(), arg_right.data()));
                    pairs.push((fun_left.data(), fun_right.data()));
                }

                (
                    ExprData::Lam(arg_left, body_left),
                    ExprData::Lam(arg_right, body_right),
                ) => {
                    ordering = arg_left.cmp(&arg_right);
                    if ordering.is_eq() {
                        pairs.push((body_left.data(), body_right.data()));
                    }
                }

                (ExprData::Var(_), _) => ordering = Ordering::Less,
                (ExprData::App(_, _), ExprData::Var(_)) => {
                    ordering = Ordering::Greater
                }
                (ExprData::App(_, _), ExprData::Lam(_, _)) => {
                    ordering = Ordering::Less
                }
                (ExprData::Lam(_, _), _) => ordering = Ordering::Greater,
            }
        }

        ordering
    }
}

impl<E> Hash for ExprData<E>
where
    E: Expr,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut target_stack = vec![self];

        while let Some(expr) = target_stack.pop() {
            match expr {
                ExprData::Var(var) => {
                    state.write_u8(0);
                    var.hash(state);
                }
                ExprData::App(fun, arg) => {
                    state.write_u8(1);
                    target_stack.push(fun.data());
                    target_stack.push(arg.data());
                }
                ExprData::Lam(_, body) => {
                    state.write_u8(2);
                    target_stack.push(body.data())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BoxedExpr<S>
where
    S: Symbol,
{
    data: ManuallyDrop<Box<ExprData<Self>>>,
}

impl<S> Expr for BoxedExpr<S>
where
    S: Symbol,
{
    type Symbol = S;

    fn from_data(data: ExprData<Self>) -> Self {
        Self { data: ManuallyDrop::new(Box::new(data)) }
    }

    fn data(&self) -> &ExprData<Self> {
        &**self.data
    }

    fn try_data_mut(&mut self) -> Option<&mut ExprData<Self>> {
        Some(self.data_mut())
    }

    fn try_take_data(&mut self) -> Option<ExprData<Self>> {
        Some(self.take_data())
    }

    fn try_into_data(self) -> Result<ExprData<Self>, Self> {
        Ok(self.into_data())
    }
}

impl<S> BoxedExpr<S>
where
    S: Symbol,
{
    pub fn data_mut(&mut self) -> &mut ExprData<Self> {
        &mut **self.data
    }

    pub fn take_data(&mut self) -> ExprData<Self> {
        mem::take(self.data_mut())
    }

    pub fn into_data(mut self) -> ExprData<Self> {
        self.take_data()
    }
}

impl<S> Drop for BoxedExpr<S>
where
    S: Symbol,
{
    fn drop(&mut self) {
        self.drop_in_place();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RcExpr<S>
where
    S: Symbol,
{
    data: Rc<ExprData<Self>>,
}

impl<S> Expr for RcExpr<S>
where
    S: Symbol,
{
    type Symbol = S;

    fn from_data(data: ExprData<Self>) -> Self {
        Self { data: Rc::new(data) }
    }

    fn data(&self) -> &ExprData<Self> {
        &*self.data
    }

    fn try_data_mut(&mut self) -> Option<&mut ExprData<Self>> {
        Rc::get_mut(&mut self.data)
    }

    fn try_take_data(&mut self) -> Option<ExprData<Self>> {
        self.try_data_mut().map(mem::take)
    }
}

impl<S> Drop for RcExpr<S>
where
    S: Symbol,
{
    fn drop(&mut self) {
        self.drop_in_place();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ArcExpr<S>
where
    S: Symbol,
{
    data: Arc<ExprData<Self>>,
}

impl<S> Expr for ArcExpr<S>
where
    S: Symbol,
{
    type Symbol = S;

    fn from_data(data: ExprData<Self>) -> Self {
        Self { data: Arc::new(data) }
    }

    fn data(&self) -> &ExprData<Self> {
        &*self.data
    }

    fn try_data_mut(&mut self) -> Option<&mut ExprData<Self>> {
        Arc::get_mut(&mut self.data)
    }

    fn try_take_data(&mut self) -> Option<ExprData<Self>> {
        self.try_data_mut().map(mem::take)
    }
}

impl<S> Drop for ArcExpr<S>
where
    S: Symbol,
{
    fn drop(&mut self) {
        self.drop_in_place();
    }
}
