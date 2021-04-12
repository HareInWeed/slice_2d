use crate::{
    iter::Slice2DIterMut,
    slice::{Shape2D, SlicePtrMut},
};

pub trait Slice2DFill<T> {
    fn fill(&mut self, value: T)
    where
        T: Clone;
    fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T;
}

impl<T: Clone, S> Slice2DFill<T> for S
where
    S: Shape2D + SlicePtrMut<T> + Slice2DIterMut<T, S>,
{
    #[inline]
    fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.row_iter_mut()
            .flatten()
            .for_each(|e| *e = value.clone());
    }
    #[inline]
    fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        self.row_iter_mut().flatten().for_each(|e| *e = f());
    }
}
