use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A symbol used for variables in Lambda Calculus.
pub trait Symbol
where
    Self: Clone + Default + PartialEq + Eq + PartialOrd + Ord + Hash,
{
}

impl<S> Symbol for S where
    S: Clone + Default + PartialEq + Eq + PartialOrd + Ord + Hash
{
}

/// A generic expression. An expression is a wrapper over [`ExprKind`], which by
/// itself is either a variable, a function application, or a lambda
/// abstraction.
pub trait Expression: Sized {
    /// Symbol used for variables.
    type Symbol: Symbol;

    /// Creates an expression from its kind.
    fn from_kind(kind: ExprKind<Self>) -> Self;

    /// Returns an immutable reference to the kind of this expression.
    fn kind(&self) -> &ExprKind<Self>;

    /// Attempts to return a mutable reference to the kind of this expression.
    fn try_kind_mut(&mut self) -> Option<&mut ExprKind<Self>>;

    /// Attempts to take the kind of this expression by replacing it with the
    /// default expression.
    fn try_take_kind(&mut self) -> Option<ExprKind<Self>>;

    /// Attempts to convert this expression into its kind.
    fn try_into_kind(mut self) -> Result<ExprKind<Self>, Self> {
        self.try_take_kind().ok_or(self)
    }

    /// Drops this expression in place: i.e. destroys the content of this given
    /// expression by a mutable reference, replacing the old expression by the
    /// default expression.
    fn drop_in_place(&mut self) {
        let mut drop_stack = Vec::new();

        drop_stack.extend(self.try_take_kind());

        while let Some(expr) = drop_stack.pop() {
            match expr {
                ExprKind::Var(_) => (),
                ExprKind::App(fun, arg) => {
                    drop_stack.extend(fun.try_into_kind());
                    drop_stack.extend(arg.try_into_kind());
                }
                ExprKind::Lam(_, body) => {
                    drop_stack.extend(body.try_into_kind())
                }
            }
        }
    }
}

/// An expression that is shallowly cloned and whose memory to store its kind is
/// shared. Typically implemented by expressions using `Rc` and `Arc`.
pub trait SharedExpr: Expression + Clone {}

/// The generic kind of an expression. Either variable, function application or
/// lambda abstraction.
#[derive(Debug)]
pub enum ExprKind<E>
where
    E: Expression,
{
    /// A variable in lambda calculus, e.g. `x`, `f`, `y`, etc.
    Var(E::Symbol),
    /// A function application in lambda calculus, e.g. `f x`, `(\x.x) y`, etc.
    App(E, E),
    /// A lambda abstraction in lambda calculus, e.g. `\x.x`, `\y.y y`, etc.
    Lam(E::Symbol, E),
}

impl<E> Default for ExprKind<E>
where
    E: Expression,
{
    fn default() -> Self {
        ExprKind::Var(E::Symbol::default())
    }
}

impl<E> ExprKind<E>
where
    E: Expression,
{
    /// Performs a shallow clone, i.e. clones only the surface of the expression
    /// kind. This is intended only for cases where `E` is a pointer such that
    /// `Clone` implemetation is shallow.
    pub fn shallow_clone(&self) -> Self
    where
        E: SharedExpr,
    {
        match self {
            ExprKind::Var(symbol) => ExprKind::Var(symbol.clone()),
            ExprKind::App(fun, arg) => ExprKind::App(fun.clone(), arg.clone()),
            ExprKind::Lam(arg, body) => {
                ExprKind::Lam(arg.clone(), body.clone())
            }
        }
    }

    /// Performs a deep clone of the expression kind, even if the wrapper `E` is
    /// reference counted.
    pub fn deep_clone(&self) -> Self {
        enum Operation<'input, E>
        where
            E: Expression,
        {
            Clone(&'input ExprKind<E>),
            CloneAppArg(&'input ExprKind<E>),
            MakeApp(ExprKind<E>),
            MakeLam(E::Symbol),
        }

        let mut op_stack = vec![Operation::Clone(self)];
        let mut output = None;

        while let Some(operation) = op_stack.pop() {
            match operation {
                Operation::Clone(input) => match input {
                    ExprKind::Var(symbol) => {
                        let expr = ExprKind::Var(symbol.clone());
                        output = Some(expr);
                    }
                    ExprKind::App(fun, arg) => {
                        op_stack.push(Operation::CloneAppArg(arg.kind()));
                        op_stack.push(Operation::Clone(fun.kind()));
                    }
                    ExprKind::Lam(arg, body) => {
                        op_stack.push(Operation::MakeLam(arg.clone()));
                        op_stack.push(Operation::Clone(body.kind()));
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
                        ExprKind::App(E::from_kind(fun), E::from_kind(arg));
                    output = Some(expr);
                }

                Operation::MakeLam(arg) => {
                    let body =
                        output.take().expect("cloning lam requires body");
                    let expr = ExprKind::Lam(arg, E::from_kind(body));
                    output = Some(expr);
                }
            }
        }

        output.expect("clone always yield a result")
    }
}

impl<E> Clone for ExprKind<E>
where
    E: Expression,
{
    fn clone(&self) -> Self {
        self.deep_clone()
    }
}

impl<E> PartialEq for ExprKind<E>
where
    E: Expression,
{
    fn eq(&self, other: &Self) -> bool {
        let mut equals = true;
        let mut pairs = vec![(self, other)];

        while let Some((left, right)) = pairs.pop().filter(|_| equals) {
            match (left, right) {
                (ExprKind::Var(symbol_left), ExprKind::Var(symbol_right)) => {
                    equals = symbol_left == symbol_right;
                }

                (
                    ExprKind::App(fun_left, arg_left),
                    ExprKind::App(fun_right, arg_right),
                ) => {
                    pairs.push((arg_left.kind(), arg_right.kind()));
                    pairs.push((fun_left.kind(), fun_right.kind()));
                }

                (
                    ExprKind::Lam(arg_left, body_left),
                    ExprKind::Lam(arg_right, body_right),
                ) => {
                    equals = arg_left == arg_right;
                    if equals {
                        pairs.push((body_left.kind(), body_right.kind()));
                    }
                }

                _ => equals = false,
            }
        }

        equals
    }
}

impl<E> Eq for ExprKind<E> where E: Expression {}

impl<E> PartialOrd for ExprKind<E>
where
    E: Expression,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E> Ord for ExprKind<E>
where
    E: Expression,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ordering = Ordering::Equal;
        let mut pairs = vec![(self, other)];

        while let Some((left, right)) = pairs.pop().filter(|_| ordering.is_eq())
        {
            match (left, right) {
                (ExprKind::Var(symbol_left), ExprKind::Var(symbol_right)) => {
                    ordering = symbol_left.cmp(symbol_right);
                }

                (
                    ExprKind::App(fun_left, arg_left),
                    ExprKind::App(fun_right, arg_right),
                ) => {
                    pairs.push((arg_left.kind(), arg_right.kind()));
                    pairs.push((fun_left.kind(), fun_right.kind()));
                }

                (
                    ExprKind::Lam(arg_left, body_left),
                    ExprKind::Lam(arg_right, body_right),
                ) => {
                    ordering = arg_left.cmp(&arg_right);
                    if ordering.is_eq() {
                        pairs.push((body_left.kind(), body_right.kind()));
                    }
                }

                (ExprKind::Var(_), _) => ordering = Ordering::Less,
                (ExprKind::App(_, _), ExprKind::Var(_)) => {
                    ordering = Ordering::Greater
                }
                (ExprKind::App(_, _), ExprKind::Lam(_, _)) => {
                    ordering = Ordering::Less
                }
                (ExprKind::Lam(_, _), _) => ordering = Ordering::Greater,
            }
        }

        ordering
    }
}

impl<E> Hash for ExprKind<E>
where
    E: Expression,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut target_stack = vec![self];

        while let Some(expr) = target_stack.pop() {
            match expr {
                ExprKind::Var(var) => {
                    state.write_u8(0);
                    var.hash(state);
                }
                ExprKind::App(fun, arg) => {
                    state.write_u8(1);
                    target_stack.push(fun.kind());
                    target_stack.push(arg.kind());
                }
                ExprKind::Lam(_, body) => {
                    state.write_u8(2);
                    target_stack.push(body.kind())
                }
            }
        }
    }
}
