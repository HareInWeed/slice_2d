use crate::{
    iter::Slice2DIter,
    slice::{Shape2D, Slice2D, Slice2DMut, SlicePtr},
};

pub fn slice_2d_eq<T, A, B>(a: &A, b: &B) -> bool
where
    T: PartialEq,
    A: Shape2D + SlicePtr<T> + Slice2DIter<T, A>,
    B: Shape2D + SlicePtr<T> + Slice2DIter<T, B>,
{
    a.get_row() == b.get_row()
        && a.get_col() == b.get_col()
        && (a.get_slice_ptr() == b.get_slice_ptr()
            || a.row_slice_iter()
                .zip(b.row_slice_iter())
                .all(|(a, b)| a == b))
}

impl<'a, T, A> PartialEq<A> for Slice2D<'a, T>
where
    T: PartialEq,
    A: Shape2D + SlicePtr<T> + Slice2DIter<T, A>,
{
    fn eq(&self, other: &A) -> bool {
        slice_2d_eq(self, other)
    }
}
impl<'a, T: Eq> Eq for Slice2D<'a, T> {}

impl<'a, T, A> PartialEq<A> for Slice2DMut<'a, T>
where
    T: PartialEq,
    A: Shape2D + SlicePtr<T> + Slice2DIter<T, A>,
{
    fn eq(&self, other: &A) -> bool {
        slice_2d_eq(self, other)
    }
}
impl<'a, T: Eq> Eq for Slice2DMut<'a, T> {}
