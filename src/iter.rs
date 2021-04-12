use crate::{
    index::{GetElemRef, GetElemRefMut},
    slice::{Shape2D, SlicePtr, SlicePtrMut},
    utils::calc_2d_index,
};
use core::{iter::Iterator, marker::PhantomData, slice};

// immutable variants
pub type Row<'a, T> = slice::Iter<'a, T>;

pub struct Rows<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    slice_2d: *const S,
    row: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, S> Rows<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    #[inline]
    pub fn new(slice_2d: &S) -> Rows<'_, T, S> {
        Rows {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, S> Iterator for Rows<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let slice_2d = unsafe { &*self.slice_2d };
        if self.row < slice_2d.get_row() {
            unsafe {
                let row_ptr = slice_2d
                    .get_slice_ptr()
                    .add(calc_2d_index(self.row, 0, slice_2d));
                let row_slice = slice::from_raw_parts(row_ptr, slice_2d.get_col());
                self.row += 1;
                Some(row_slice.iter())
            }
        } else {
            None
        }
    }
}

pub struct RowSlices<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    slice_2d: *const S,
    row: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, S> RowSlices<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    #[inline]
    pub fn new(slice_2d: &S) -> RowSlices<'_, T, S> {
        RowSlices {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, S> Iterator for RowSlices<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let slice_2d = unsafe { &*self.slice_2d };
        if self.row < slice_2d.get_row() {
            unsafe {
                let row_ptr = slice_2d
                    .get_slice_ptr()
                    .add(calc_2d_index(self.row, 0, slice_2d));
                let row_slice = slice::from_raw_parts(row_ptr, slice_2d.get_col());
                self.row += 1;
                Some(row_slice)
            }
        } else {
            None
        }
    }
}

pub struct Col<'a, T> {
    ptr: *const T,
    end: *const T,
    step: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Col<'a, T> {
    #[inline]
    pub fn new<S>(slice_2d: &S, col: usize) -> Option<Col<'_, T>>
    where
        S: Shape2D + SlicePtr<T>,
    {
        if col < slice_2d.get_col() {
            unsafe { Some(Col::new_unchecked(slice_2d, col)) }
        } else {
            None
        }
    }
    #[inline]
    pub unsafe fn new_unchecked<S>(slice_2d: &S, col: usize) -> Col<'_, T>
    where
        S: Shape2D + SlicePtr<T>,
    {
        let ptr = slice_2d.get_unchecked((0, col)) as *const T;
        Col {
            ptr,
            end: ptr.add(slice_2d.get_base_col() * slice_2d.get_row()),
            step: slice_2d.get_base_col(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Col<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.ptr < self.end {
                let elem = &*self.ptr;
                self.ptr = self.ptr.add(self.step);
                Some(elem)
            } else {
                None
            }
        }
    }
}

pub struct Cols<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    slice_2d: *const S,
    col: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, S> Cols<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    #[inline]
    pub fn new(slice_2d: &S) -> Cols<'_, T, S> {
        Cols {
            slice_2d,
            col: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, S: 'a> Iterator for Cols<'a, T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    type Item = Col<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let slice_2d = unsafe { &*self.slice_2d };
        if self.col < slice_2d.get_col() {
            let old_col = self.col;
            self.col += 1;
            unsafe { Some(Col::new_unchecked(slice_2d, old_col)) }
        } else {
            None
        }
    }
}

// mutable variants
pub type RowMut<'a, T> = slice::IterMut<'a, T>;

pub struct RowsMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    slice_2d: *mut S,
    row: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T, S> RowsMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    #[inline]
    pub fn new(slice_2d: &mut S) -> RowsMut<'_, T, S> {
        RowsMut {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, S> Iterator for RowsMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    type Item = RowMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let slice_2d = unsafe { &mut *self.slice_2d };
        if self.row < slice_2d.get_row() {
            let row_ptr = unsafe { slice_2d.get_unchecked_mut((self.row, 0)) } as *mut T;
            let row_slice = unsafe { slice::from_raw_parts_mut(row_ptr, slice_2d.get_col()) };
            self.row += 1;
            Some(row_slice.iter_mut())
        } else {
            None
        }
    }
}

pub struct RowSlicesMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    slice_2d: *mut S,
    row: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T, S> RowSlicesMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    #[inline]
    pub fn new(slice_2d: &mut S) -> RowSlicesMut<'_, T, S> {
        RowSlicesMut {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, S> Iterator for RowSlicesMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        let slice_2d = unsafe { &mut *self.slice_2d };
        if self.row < slice_2d.get_row() {
            let row_ptr = unsafe { slice_2d.get_unchecked_mut((self.row, 0)) } as *mut T;
            let row_slice = unsafe { slice::from_raw_parts_mut(row_ptr, slice_2d.get_col()) };
            self.row += 1;
            Some(row_slice)
        } else {
            None
        }
    }
}

pub struct ColMut<'a, T> {
    ptr: *mut T,
    end: *mut T,
    step: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ColMut<'a, T> {
    #[inline]
    pub fn new<S>(slice_2d: &mut S, col: usize) -> Option<ColMut<'_, T>>
    where
        S: Shape2D + SlicePtrMut<T>,
    {
        if col < slice_2d.get_col() {
            unsafe { Some(ColMut::new_unchecked(slice_2d, col)) }
        } else {
            None
        }
    }
    #[inline]
    pub unsafe fn new_unchecked<S>(slice_2d: &mut S, col: usize) -> ColMut<'_, T>
    where
        S: Shape2D + SlicePtrMut<T>,
    {
        let ptr = slice_2d.get_unchecked_mut((0, col)) as *mut T;
        ColMut {
            ptr,
            end: ptr.add(slice_2d.get_base_col() * slice_2d.get_row()),
            step: slice_2d.get_base_col(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ColMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.ptr < self.end {
                let elem = &mut *self.ptr;
                self.ptr = self.ptr.add(self.step);
                Some(elem)
            } else {
                None
            }
        }
    }
}

pub struct ColsMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    slice_2d: *mut S,
    col: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T, S> ColsMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    #[inline]
    pub fn new(slice_2d: &mut S) -> ColsMut<'_, T, S> {
        ColsMut {
            slice_2d,
            col: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, S: 'a> Iterator for ColsMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    type Item = ColMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let slice_2d = unsafe { &mut *self.slice_2d };
        if self.col < slice_2d.get_col() {
            let old_col = self.col;
            self.col += 1;
            unsafe { Some(ColMut::new_unchecked(slice_2d, old_col)) }
        } else {
            None
        }
    }
}

// iterator related traits

pub trait Slice2DIter<T, S>
where
    S: Shape2D + SlicePtr<T>,
{
    fn row_iter(&self) -> Rows<'_, T, S>;
    fn row_slice_iter(&self) -> RowSlices<'_, T, S>;
    fn col_iter(&self) -> Cols<'_, T, S>;
}

pub trait Slice2DIterMut<T, S>
where
    S: Shape2D + SlicePtrMut<T>,
{
    fn row_iter_mut(&mut self) -> RowsMut<'_, T, S>;
    fn row_slice_iter_mut(&mut self) -> RowSlicesMut<'_, T, S>;
    fn col_iter_mut(&mut self) -> ColsMut<'_, T, S>;
}

impl<'a, T, S> Slice2DIter<T, S> for S
where
    S: Shape2D + SlicePtr<T>,
{
    fn row_iter(&self) -> Rows<'_, T, S> {
        Rows::new(self)
    }

    fn row_slice_iter(&self) -> RowSlices<'_, T, S> {
        RowSlices::new(self)
    }

    fn col_iter(&self) -> Cols<'_, T, S> {
        Cols::new(self)
    }
}

impl<'a, T, S> Slice2DIterMut<T, S> for S
where
    S: Shape2D + SlicePtrMut<T>,
{
    fn row_iter_mut(&mut self) -> RowsMut<'_, T, S> {
        RowsMut::new(self)
    }

    fn row_slice_iter_mut(&mut self) -> RowSlicesMut<'_, T, S> {
        RowSlicesMut::new(self)
    }

    fn col_iter_mut(&mut self) -> ColsMut<'_, T, S> {
        ColsMut::new(self)
    }
}
