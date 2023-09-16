pub trait Semigroup<T>
where
    T: Clone,
{
    /// Binary operation of the semigroup.
    fn bin_op(left: &T, right: &T) -> T;
}

pub trait Monoid<T>: Semigroup<T>
where
    T: Clone,
{
    /// Identity element of the monoid.
    fn id() -> T;
}
