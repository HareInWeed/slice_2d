use crate::{
    iter::Slice2DIter,
    slice::{Shape2D, Slice2D, Slice2DMut, SlicePtr},
};

impl<'a, T, A> PartialEq<A> for Slice2D<'a, T>
where
    T: PartialEq,
    A: Shape2D + SlicePtr<T> + Slice2DIter<T>,
{
    fn eq(&self, other: &A) -> bool {
        self.get_row() == other.get_row()
            && self.get_col() == other.get_col()
            && (self.get_slice_ptr() == other.get_slice_ptr()
                || self
                    .row_slice_iter()
                    .zip(other.row_slice_iter())
                    .all(|(a, b)| a == b))
    }
}
impl<'a, T: PartialEq> Eq for Slice2D<'a, T> {}

impl<'a, T, A> PartialEq<A> for Slice2DMut<'a, T>
where
    T: PartialEq,
    A: Shape2D + SlicePtr<T> + Slice2DIter<T>,
{
    fn eq(&self, other: &A) -> bool {
        self.get_row() == other.get_row()
            && self.get_col() == other.get_col()
            && (self.get_slice_ptr() == other.get_slice_ptr()
                || self
                    .row_slice_iter()
                    .zip(other.row_slice_iter())
                    .all(|(a, b)| a == b))
    }
}
impl<'a, T: PartialEq> Eq for Slice2DMut<'a, T> {}
