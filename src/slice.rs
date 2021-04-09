use crate::index::{Slice2DIndex, Slice2DIndexMut};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ptr::null;

pub trait Slice2DShape {
    fn get_array_col(&self) -> usize;
    fn get_row(&self) -> usize;
    fn get_col(&self) -> usize;
}

pub trait ArrayPtr<T> {
    fn get_array(&self) -> *const T;
}

pub trait ArrayPtrMut<T> {
    fn get_array_mut(&self) -> *mut T;
}

impl<'a, T, S> GetElemRefMut<'a, T> for S
where
    S: Slice2DShape + ArrayPtr<T> + ArrayPtrMut<T>,
{
    fn get_mut<I>(&'a mut self, index: I) -> Option<I::RefMut>
    where
        I: Slice2DIndexMut<'a, T, Self>,
    {
        index.get_mut(self)
    }
    unsafe fn get_unchecked_mut<I>(&'a mut self, index: I) -> I::RefMut
    where
        I: Slice2DIndexMut<'a, T, Self>,
    {
        index.get_unchecked_mut(self)
    }
}

pub trait SplitSlice2D<'a, T>: GetElemRef<'a, T> {
    fn split_at_vertically(&'a self, j: usize) -> [Slice2D<'a, T>; 2] {
        [
            self.get((.., ..j)).expect("out of boundary"),
            self.get((.., j..)).expect("out of boundary"),
        ]
    }
    fn split_at_horizontally(&'a self, i: usize) -> [Slice2D<'a, T>; 2] {
        [
            self.get((..i, ..)).expect("out of boundary"),
            self.get((..i, ..)).expect("out of boundary"),
        ]
    }
    fn split_at(&'a self, idx: (usize, usize)) -> [[Slice2D<'a, T>; 2]; 2] {
        let (i, j) = idx;
        [
            [
                self.get((..i, ..j)).expect("out of boundary"),
                self.get((..i, j..)).expect("out of boundary"),
            ],
            [
                self.get((i.., ..j)).expect("out of boundary"),
                self.get((i.., j..)).expect("out of boundary"),
            ],
        ]
    }
}

#[derive(Hash, Debug, Clone)]
pub struct Slice2DRaw<T> {
    array: *const T,

    array_col: usize,
    row: usize,
    col: usize,
}

impl<T> Slice2DRaw<T> {
    unsafe fn from_raw_parts(
        array: *const T,
        array_col: usize,
        row: usize,
        col: usize,
    ) -> Slice2DRaw<T> {
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
        unsafe { Self::from_raw_parts(null(), 0, 0, 0) }
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

#[derive(Hash, Clone, Default, Debug)]
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
        unsafe { Slice2D::<'b, T>::from_raw_parts(slice.as_ptr(), col, row, col) }
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
}
impl<'a, T> Slice2DRawRef for Slice2D<'a, T> {
    type DataT = T;

    #[inline(always)]
    fn get_slice_2d_raw(&self) -> &Slice2DRaw<Self::DataT> {
        &self.raw
    }
}
impl<'a, T> ArrayPtr<T> for Slice2D<'a, T> {
    fn get_array(&self) -> *const T {
        self.raw.array
    }
}
impl<'a, T> SplitSlice2D<'a, T> for Slice2D<'a, T> {}

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
        unsafe { Slice2DMut::<'b, T>::from_raw_parts(slice.as_mut_ptr(), col, row, col) }
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
}
impl<'a, T> ArrayPtr<T> for Slice2DMut<'a, T> {
    fn get_array(&self) -> *const T {
        self.raw.array
    }
}
impl<'a, T> ArrayPtrMut<T> for Slice2DMut<'a, T> {
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
impl<'a, T> SplitSlice2D<'a, T> for Slice2DMut<'a, T> {}

// get trait
pub trait GetElemRef<'a, T>: Sized + Slice2DShape + ArrayPtr<T> {
    fn get<I>(&'a self, index: I) -> Option<I::Ref>
    where
        I: Slice2DIndex<'a, T, Self>;
    unsafe fn get_unchecked<I>(&'a self, index: I) -> I::Ref
    where
        I: Slice2DIndex<'a, T, Self>;
}

pub trait GetElemRefMut<'a, T>: Slice2DShape + ArrayPtr<T> + ArrayPtrMut<T> {
    fn get_mut<I>(&'a mut self, index: I) -> Option<I::RefMut>
    where
        I: Slice2DIndexMut<'a, T, Self>;
    unsafe fn get_unchecked_mut<I>(&'a mut self, index: I) -> I::RefMut
    where
        I: Slice2DIndexMut<'a, T, Self>;
}

impl<'a, T, S> GetElemRef<'a, T> for S
where
    S: Slice2DShape + ArrayPtr<T>,
{
    fn get<I>(&'a self, index: I) -> Option<I::Ref>
    where
        I: Slice2DIndex<'a, T, Self>,
    {
        index.get(self)
    }
    unsafe fn get_unchecked<I>(&'a self, index: I) -> I::Ref
    where
        I: Slice2DIndex<'a, T, Self>,
    {
        index.get_unchecked(self)
    }
}

// because `Index` trait in Rust can only return reference for now,
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
