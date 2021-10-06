use super::{ExprKind, Expression, SharedExpr, Symbol};
use std::mem;
use std::rc::Rc;

/// An expression wrapper which uses a shared pointer, but that is not
/// thread-safe. Memory of the stored expression kind is shared through
/// reference counting.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RcExpr<S>
where
    S: Symbol,
{
    kind: Rc<ExprKind<Self>>,
}

impl<S> Expression for RcExpr<S>
where
    S: Symbol,
{
    type Symbol = S;

    fn from_kind(kind: ExprKind<Self>) -> Self {
        Self { kind: Rc::new(kind) }
    }

    fn kind(&self) -> &ExprKind<Self> {
        &*self.kind
    }

    fn try_kind_mut(&mut self) -> Option<&mut ExprKind<Self>> {
        Rc::get_mut(&mut self.kind)
    }

    fn try_take_kind(&mut self) -> Option<ExprKind<Self>> {
        self.try_kind_mut().map(mem::take)
    }
}

impl<S> SharedExpr for RcExpr<S> where S: Symbol {}

impl<S> Drop for RcExpr<S>
where
    S: Symbol,
{
    fn drop(&mut self) {
        self.drop_in_place();
    }
}
