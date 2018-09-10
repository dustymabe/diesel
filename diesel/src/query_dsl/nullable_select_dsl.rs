/// The `nullable` method
///
/// This trait should not be relied on directly by most apps. Its behavior is
/// provided by [`QueryDsl`]. However you may need a where clause on this trait
/// to call `nullable` from generic code.
///
/// [`QueryDsl`]: ../trait.QueryDsl.html#method.nullable
pub trait NullableSelectDsl {
    /// The return type of `nullable`
    type Output;

    /// See the trait documentation
    fn nullable(self) -> Self::Output;
}
