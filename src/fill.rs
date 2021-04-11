use crate::iter::Slice2DIterMut;

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
    S: Slice2DIterMut<T>,
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
