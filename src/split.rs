use crate::{
    index::GetElemRef,
    slice::{Shape2D, Slice2D, Slice2DMut, SlicePtr, SlicePtrMut},
    utils::calc_2d_index,
};

// split slice2d into slices
pub trait Split<'a, T> {
    fn split_at_vertically(&'a self, j: usize) -> Option<[Slice2D<'a, T>; 2]>;
    fn split_at_horizontally(&'a self, i: usize) -> Option<[Slice2D<'a, T>; 2]>;
    fn split_at(&'a self, idx: (usize, usize)) -> Option<[[Slice2D<'a, T>; 2]; 2]>;
}

impl<'a, T, G> Split<'a, T> for G
where
    G: Shape2D + SlicePtr<T>,
{
    fn split_at_vertically(&'a self, j: usize) -> Option<[Slice2D<'a, T>; 2]> {
        if j < self.get_col() {
            unsafe { Some([self.get_unchecked((.., ..j)), self.get_unchecked((.., j..))]) }
        } else {
            None
        }
    }
    fn split_at_horizontally(&'a self, i: usize) -> Option<[Slice2D<'a, T>; 2]> {
        if i < self.get_row() {
            unsafe { Some([self.get_unchecked((..i, ..)), self.get_unchecked((i.., ..))]) }
        } else {
            None
        }
    }
    fn split_at(&'a self, idx: (usize, usize)) -> Option<[[Slice2D<'a, T>; 2]; 2]> {
        let (i, j) = idx;
        if i <= self.get_row() && j <= self.get_col() {
            unsafe {
                Some([
                    [
                        self.get_unchecked((..i, ..j)),
                        self.get_unchecked((..i, j..)),
                    ],
                    [
                        self.get_unchecked((i.., ..j)),
                        self.get_unchecked((i.., j..)),
                    ],
                ])
            }
        } else {
            None
        }
    }
}

pub trait SplitMut<'a, T> {
    fn split_at_vertically_mut(&'a mut self, j: usize) -> Option<[Slice2DMut<'a, T>; 2]>;
    fn split_at_horizontally_mut(&'a mut self, i: usize) -> Option<[Slice2DMut<'a, T>; 2]>;
    fn split_at_mut(&'a mut self, idx: (usize, usize)) -> Option<[[Slice2DMut<'a, T>; 2]; 2]>;
}

impl<'a, T, G> SplitMut<'a, T> for G
where
    G: Shape2D + SlicePtrMut<T>,
{
    fn split_at_vertically_mut(&'a mut self, j: usize) -> Option<[Slice2DMut<'a, T>; 2]> {
        if j < self.get_col() {
            unsafe {
                Some([
                    Slice2DMut::from_raw_parts(
                        self.get_slice_mut(),
                        self.get_base_col(),
                        self.get_row(),
                        j,
                    ),
                    Slice2DMut::from_raw_parts(
                        self.get_slice_mut().add(calc_2d_index(0, j, self)),
                        self.get_base_col(),
                        self.get_row(),
                        self.get_col() - j,
                    ),
                ])
            }
        } else {
            None
        }
    }
    fn split_at_horizontally_mut(&'a mut self, i: usize) -> Option<[Slice2DMut<'a, T>; 2]> {
        if i < self.get_row() {
            unsafe {
                Some([
                    Slice2DMut::from_raw_parts(
                        self.get_slice_mut(),
                        self.get_base_col(),
                        i,
                        self.get_col(),
                    ),
                    Slice2DMut::from_raw_parts(
                        self.get_slice_mut().add(calc_2d_index(i, 0, self)),
                        self.get_base_col(),
                        self.get_row() - i,
                        self.get_col(),
                    ),
                ])
            }
        } else {
            None
        }
    }
    fn split_at_mut(&'a mut self, idx: (usize, usize)) -> Option<[[Slice2DMut<'a, T>; 2]; 2]> {
        let (i, j) = idx;
        if i <= self.get_row() && j <= self.get_col() {
            unsafe {
                Some([
                    [
                        Slice2DMut::from_raw_parts(self.get_slice_mut(), self.get_base_col(), i, j),
                        Slice2DMut::from_raw_parts(
                            self.get_slice_mut().add(calc_2d_index(i, 0, self)),
                            self.get_base_col(),
                            i,
                            self.get_col() - j,
                        ),
                    ],
                    [
                        Slice2DMut::from_raw_parts(
                            self.get_slice_mut().add(calc_2d_index(0, j, self)),
                            self.get_base_col(),
                            self.get_row() - i,
                            j,
                        ),
                        Slice2DMut::from_raw_parts(
                            self.get_slice_mut().add(calc_2d_index(i, j, self)),
                            self.get_base_col(),
                            self.get_row() - i,
                            self.get_col() - j,
                        ),
                    ],
                ])
            }
        } else {
            None
        }
    }
}
