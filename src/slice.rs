use crate::index::{Slice2DIndex, Slice2DIndexMut};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ptr::null;

pub trait Slice2DShape {
    fn get_array_col(&self) -> usize;
    fn get_row(&self) -> usize;
    fn get_col(&self) -> usize;
}

pub trait ArrayRef<T> {
    fn get_array(&self) -> *const T;
}

pub trait ArrayRefMut<T> {
    fn get_array_mut(&self) -> *mut T;
}

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct Slice2DRaw<T> {
    array: *const T,

    array_col: usize,
    row: usize,
    col: usize,
}

impl<T> Slice2DRaw<T> {
    fn from_raw_parts(array: *const T, array_col: usize, row: usize, col: usize) -> Slice2DRaw<T> {
        Slice2DRaw {
            array,
            array_col,
            row,
            col,
        }
    }
}

impl<T> Default for Slice2DRaw<T> {
    fn default() -> Self {
        Self::from_raw_parts(null(), 0, 0, 0)
    }
}

pub trait Slice2DRawRef {
    type DataT;
    fn get_slice_2d_raw(&self) -> &Slice2DRaw<Self::DataT>;
}

impl<T, S> Slice2DShape for S
where
    S: Slice2DRawRef<DataT = T>,
{
    #[inline(always)]
    fn get_array_col(&self) -> usize {
        self.get_slice_2d_raw().array_col
    }

    #[inline(always)]
    fn get_row(&self) -> usize {
        self.get_slice_2d_raw().row
    }

    #[inline(always)]
    fn get_col(&self) -> usize {
        self.get_slice_2d_raw().col
    }
}

impl<T> Slice2DRawRef for Slice2DRaw<T> {
    type DataT = T;

    #[inline(always)]
    fn get_slice_2d_raw(&self) -> &Slice2DRaw<Self::DataT> {
        &self
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Default, Debug)]
pub struct Slice2D<'a, T> {
    raw: Slice2DRaw<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Slice2D<'a, T> {
    pub fn from_slice<'b>(slice: &'b [T], row: usize, col: usize) -> Slice2D<'b, T> {
        assert!(
            row * col <= slice.len(),
            "slice does not contain enough space."
        );
        Slice2D {
            raw: Slice2DRaw::from_raw_parts(slice.as_ptr(), col, row, col),
            phantom: PhantomData,
        }
    }
    pub unsafe fn from_raw_parts<'b>(
        slice: *const T,
        array_col: usize,
        row: usize,
        col: usize,
    ) -> Slice2D<'b, T> {
        Slice2D {
            raw: Slice2DRaw::from_raw_parts(slice, array_col, row, col),
            phantom: PhantomData,
        }
    }
    pub fn get<I>(&self, index: I) -> Option<I::Ref>
    where
        I: Slice2DIndex<'a, T, Self>,
    {
        index.get(self)
    }
    pub unsafe fn get_unchecked<I>(&self, index: I) -> I::Ref
    where
        I: Slice2DIndex<'a, T, Self>,
    {
        index.get_unchecked(self)
    }
}

impl<'a, T> Slice2DRawRef for Slice2D<'a, T> {
    type DataT = T;

    #[inline(always)]
    fn get_slice_2d_raw(&self) -> &Slice2DRaw<Self::DataT> {
        &self.raw
    }
}

impl<'a, T> ArrayRef<T> for Slice2D<'a, T> {
    fn get_array(&self) -> *const T {
        self.raw.array
    }
}

#[derive(Hash, Default, Debug)]
pub struct Slice2DMut<'a, T> {
    raw: Slice2DRaw<T>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Slice2DMut<'a, T> {
    pub fn from_slice<'b>(slice: &'b mut [T], row: usize, col: usize) -> Slice2DMut<'b, T> {
        assert!(
            row * col <= slice.len(),
            "slice does not contain enough space."
        );
        Slice2DMut {
            raw: Slice2DRaw::from_raw_parts(slice.as_ptr(), col, row, col),
            phantom: PhantomData,
        }
    }
    pub unsafe fn from_raw_parts<'b>(
        slice: *mut T,
        array_col: usize,
        row: usize,
        col: usize,
    ) -> Slice2DMut<'b, T> {
        Slice2DMut {
            raw: Slice2DRaw::from_raw_parts(slice, array_col, row, col),
            phantom: PhantomData,
        }
    }

    pub fn get<I>(&self, index: I) -> Option<I::Ref>
    where
        I: Slice2DIndex<'a, T, Self>,
    {
        index.get(self)
    }
    pub unsafe fn get_unchecked<I>(&self, index: I) -> I::Ref
    where
        I: Slice2DIndex<'a, T, Self>,
    {
        index.get_unchecked(self)
    }
    pub fn get_mut<I>(&mut self, index: I) -> Option<I::RefMut>
    where
        I: Slice2DIndexMut<'a, T, Self>,
    {
        index.get_mut(self)
    }
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> I::RefMut
    where
        I: Slice2DIndexMut<'a, T, Self>,
    {
        index.get_unchecked_mut(self)
    }
}

impl<'a, T> ArrayRef<T> for Slice2DMut<'a, T> {
    fn get_array(&self) -> *const T {
        self.raw.array
    }
}

impl<'a, T> ArrayRefMut<T> for Slice2DMut<'a, T> {
    fn get_array_mut(&self) -> *mut T {
        self.raw.array as *mut T
    }
}

impl<'a, T> Slice2DRawRef for Slice2DMut<'a, T> {
    type DataT = T;

    #[inline(always)]
    fn get_slice_2d_raw(&self) -> &Slice2DRaw<Self::DataT> {
        &self.raw
    }
}

// because `Index` trait in Rust can only return reference,
// we can not index a Slice2D with ranges
impl<'a, T: 'a> Index<(usize, usize)> for Slice2D<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &T {
        index.index(self)
    }
}

impl<'a, T: 'a> Index<(usize, usize)> for Slice2DMut<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &T {
        index.index(self)
    }
}

impl<'a, T: 'a> IndexMut<(usize, usize)> for Slice2DMut<'a, T> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        index.index_mut(self)
    }
}
