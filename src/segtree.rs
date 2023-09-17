use crate::monoid::Monoid;
use std::marker::PhantomData;

pub struct Segtree<T, M>
where
    M: Monoid<T>,
    T: Clone,
{
    n: usize,
    size: usize,
    log: usize,
    dat: Vec<T>,
    phantom: PhantomData<M>,
}

impl<T, M> Segtree<T, M>
where
    T: Clone,
    M: Monoid<T>,
{
    pub fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            size: n * 2,
            log: n.trailing_zeros() as usize,
            dat: vec![M::id(); n * 2],
            phantom: PhantomData,
        }
    }
}
