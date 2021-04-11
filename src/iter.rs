use crate::{
    index::{GetElemRef, GetElemRefMut},
    slice::{Slice2DRaw, Slice2DRawRef, Slice2DRawRefMut, SlicePtr},
    utils::calc_2d_index,
    Shape2D,
};
use core::{iter::Iterator, marker::PhantomData, slice};

// immutable variants
pub type Row<'a, T> = slice::Iter<'a, T>;

pub struct Rows<'a, T> {
    slice_2d: *const Slice2DRaw<T>,
    row: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Rows<'a, T> {
    #[inline]
    pub fn new(slice_2d: &Slice2DRaw<T>) -> Rows<'_, T> {
        Rows {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Rows<'a, T> {
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

pub struct RowSlices<'a, T> {
    slice_2d: *const Slice2DRaw<T>,
    row: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RowSlices<'a, T> {
    #[inline]
    pub fn new(slice_2d: &Slice2DRaw<T>) -> RowSlices<'_, T> {
        RowSlices {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for RowSlices<'a, T> {
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
    pub fn new(slice_2d: &Slice2DRaw<T>, col: usize) -> Option<Col<'_, T>> {
        if col < slice_2d.get_col() {
            unsafe { Some(Col::new_unchecked(slice_2d, col)) }
        } else {
            None
        }
    }
    #[inline]
    pub unsafe fn new_unchecked(slice_2d: &Slice2DRaw<T>, col: usize) -> Col<'_, T> {
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

pub struct Cols<'a, T> {
    slice_2d: *const Slice2DRaw<T>,
    col: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Cols<'a, T> {
    #[inline]
    pub fn new(slice_2d: &Slice2DRaw<T>) -> Cols<'_, T> {
        Cols {
            slice_2d,
            col: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Cols<'a, T> {
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

pub struct RowsMut<'a, T> {
    slice_2d: *mut Slice2DRaw<T>,
    row: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> RowsMut<'a, T> {
    #[inline]
    pub fn new(slice_2d: &mut Slice2DRaw<T>) -> RowsMut<'_, T> {
        RowsMut {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for RowsMut<'a, T> {
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

pub struct RowSlicesMut<'a, T> {
    slice_2d: *mut Slice2DRaw<T>,
    row: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> RowSlicesMut<'a, T> {
    #[inline]
    pub fn new(slice_2d: &mut Slice2DRaw<T>) -> RowSlicesMut<'_, T> {
        RowSlicesMut {
            slice_2d,
            row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for RowSlicesMut<'a, T> {
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
    pub fn new(slice_2d: &mut Slice2DRaw<T>, col: usize) -> Option<ColMut<'_, T>> {
        if col < slice_2d.get_col() {
            unsafe { Some(ColMut::new_unchecked(slice_2d, col)) }
        } else {
            None
        }
    }
    #[inline]
    pub unsafe fn new_unchecked(slice_2d: &mut Slice2DRaw<T>, col: usize) -> ColMut<'_, T> {
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

pub struct ColsMut<'a, T> {
    slice_2d: *mut Slice2DRaw<T>,
    col: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> ColsMut<'a, T> {
    #[inline]
    pub fn new(slice_2d: &mut Slice2DRaw<T>) -> ColsMut<'_, T> {
        ColsMut {
            slice_2d,
            col: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ColsMut<'a, T> {
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

pub trait Slice2DIter<T> {
    fn row_iter(&self) -> Rows<'_, T>;
    fn row_slice_iter(&self) -> RowSlices<'_, T>;
    fn col_iter(&self) -> Cols<'_, T>;
}

pub trait Slice2DIterMut<T> {
    fn row_iter_mut(&mut self) -> RowsMut<'_, T>;
    fn row_slice_iter_mut(&mut self) -> RowSlicesMut<'_, T>;
    fn col_iter_mut(&mut self) -> ColsMut<'_, T>;
}

impl<'a, T, S> Slice2DIter<T> for S
where
    S: Slice2DRawRef<DataT = T>,
{
    fn row_iter(&self) -> Rows<'_, T> {
        Rows::new(self.get_slice_2d_raw())
    }

    fn row_slice_iter(&self) -> RowSlices<'_, T> {
        RowSlices::new(self.get_slice_2d_raw())
    }

    fn col_iter(&self) -> Cols<'_, T> {
        Cols::new(self.get_slice_2d_raw())
    }
}

impl<'a, T, S> Slice2DIterMut<T> for S
where
    S: Slice2DRawRefMut<DataT = T>,
{
    fn row_iter_mut(&mut self) -> RowsMut<'_, T> {
        RowsMut::new(self.get_slice_2d_raw_mut())
    }

    fn row_slice_iter_mut(&mut self) -> RowSlicesMut<'_, T> {
        RowSlicesMut::new(self.get_slice_2d_raw_mut())
    }

    fn col_iter_mut(&mut self) -> ColsMut<'_, T> {
        ColsMut::new(self.get_slice_2d_raw_mut())
    }
}
