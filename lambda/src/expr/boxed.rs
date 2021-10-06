use super::{ExprKind, Expression, Symbol};
use std::mem;
use std::mem::ManuallyDrop;

/// An expression that is boxed, i.e. the kind of the expression is stored in an
/// onwer heap-allocated pointer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BoxedExpr<S>
where
    S: Symbol,
{
    kind: ManuallyDrop<Box<ExprKind<Self>>>,
}

impl<S> Expression for BoxedExpr<S>
where
    S: Symbol,
{
    type Symbol = S;

    fn from_kind(kind: ExprKind<Self>) -> Self {
        Self { kind: ManuallyDrop::new(Box::new(kind)) }
    }

    fn kind(&self) -> &ExprKind<Self> {
        &**self.kind
    }

    fn try_kind_mut(&mut self) -> Option<&mut ExprKind<Self>> {
        Some(self.kind_mut())
    }

    fn try_take_kind(&mut self) -> Option<ExprKind<Self>> {
        Some(self.take_kind())
    }

    fn try_into_kind(self) -> Result<ExprKind<Self>, Self> {
        Ok(self.into_kind())
    }
}

impl<S> BoxedExpr<S>
where
    S: Symbol,
{
    /// Successfully yields a mutable reference to the stored kind.
    pub fn kind_mut(&mut self) -> &mut ExprKind<Self> {
        &mut **self.kind
    }

    /// Successfully replaces the stored kind with the default kind and returns
    /// the previous value.
    pub fn take_kind(&mut self) -> ExprKind<Self> {
        mem::take(self.kind_mut())
    }

    /// Successfully converts the expression into the stored kind.
    pub fn into_kind(mut self) -> ExprKind<Self> {
        self.take_kind()
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
