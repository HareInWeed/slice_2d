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
    pub unsafe fn new<'b>(slice_2d: &'b Slice2DRaw<T>) -> Rows<'b, T> {
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
                    .get_slice()
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

pub struct Col<'a, T> {
    slice_2d: &'a Slice2DRaw<T>,
    col: usize,
    row: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Col<'a, T> {
    #[inline]
    pub unsafe fn new<'b>(cols: &Cols<'b, T>, col: usize) -> Col<'b, T> {
        Col {
            slice_2d: cols.slice_2d,
            row: 0,
            col,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Col<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.slice_2d.get_row() {
            let idx = (self.row, self.col);
            self.row += 1;
            Some(self.slice_2d.get(idx).unwrap())
        } else {
            None
        }
    }
}

pub struct Cols<'a, T> {
    slice_2d: &'a Slice2DRaw<T>,
    col: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Cols<'a, T> {
    #[inline]
    pub unsafe fn new<'b>(slice_2d: &'b Slice2DRaw<T>) -> Cols<'b, T> {
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
        if self.col < self.slice_2d.get_col() {
            let old_col = self.col;
            self.col += 1;
            unsafe { Some(Col::new(self, old_col)) }
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
    pub unsafe fn new<'b>(slice_2d: &'b mut Slice2DRaw<T>) -> RowsMut<'b, T> {
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

pub struct ColMut<'a, T> {
    slice_2d: *mut Slice2DRaw<T>,
    col: usize,
    row: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ColMut<'a, T> {
    #[inline]
    pub unsafe fn new<'b>(slice_2d: &'b mut Slice2DRaw<T>, col: usize) -> ColMut<'b, T> {
        ColMut {
            slice_2d: slice_2d,
            row: 0,
            col,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ColMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let slice_2d = unsafe { &mut *self.slice_2d };
        if self.row < slice_2d.get_row() {
            let idx = (self.row, self.col);
            self.row += 1;
            unsafe { Some(slice_2d.get_unchecked_mut(idx)) }
        } else {
            None
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
    pub unsafe fn new<'b>(slice_2d: &'b mut Slice2DRaw<T>) -> ColsMut<'b, T> {
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
            unsafe { Some(ColMut::new(slice_2d, old_col)) }
        } else {
            None
        }
    }
}

// iterator related traits

pub trait Slice2DIter<T> {
    fn row_iter(&self) -> Rows<'_, T>;
    fn col_iter(&self) -> Cols<'_, T>;
}

pub trait Slice2DIterMut<T> {
    fn row_iter_mut(&mut self) -> RowsMut<'_, T>;
    fn col_iter_mut(&mut self) -> ColsMut<'_, T>;
}

impl<'a, T, S> Slice2DIter<T> for S
where
    S: Slice2DRawRef<DataT = T>,
{
    fn row_iter(&self) -> Rows<'_, T> {
        unsafe { Rows::new(self.get_slice_2d_raw()) }
    }

    fn col_iter(&self) -> Cols<'_, T> {
        unsafe { Cols::new(self.get_slice_2d_raw()) }
    }
}

impl<'a, T, S> Slice2DIterMut<T> for S
where
    S: Slice2DRawRefMut<DataT = T>,
{
    fn row_iter_mut(&mut self) -> RowsMut<'_, T> {
        unsafe { RowsMut::new(self.get_slice_2d_raw_mut()) }
    }

    fn col_iter_mut(&mut self) -> ColsMut<'_, T> {
        unsafe { ColsMut::new(self.get_slice_2d_raw_mut()) }
    }
}
