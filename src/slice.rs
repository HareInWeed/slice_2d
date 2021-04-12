use core::{convert::From, marker::PhantomData, ptr::null};

pub trait Shape2D {
    fn get_base_col(&self) -> usize;
    fn get_row(&self) -> usize;
    fn get_col(&self) -> usize;
}
pub trait Shape2DExt: Shape2D {
    fn get_shape(&self) -> (usize, usize);
    fn is_empty(&self) -> bool;
}
impl<S> Shape2DExt for S
where
    S: Shape2D,
{
    #[inline(always)]
    fn get_shape(&self) -> (usize, usize) {
        (self.get_row(), self.get_col())
    }
    fn is_empty(&self) -> bool {
        self.get_row() == 0 || self.get_col() == 0
    }
}

pub trait SlicePtr<T> {
    fn get_slice_ptr(&self) -> *const T;
}

pub trait SlicePtrMut<T> {
    fn get_slice_ptr_mut(&self) -> *mut T;
}

#[derive(Hash, Debug, Clone)]
pub struct Slice2DRaw<T> {
    slice: *const T,

    base_col: usize,
    row: usize,
    col: usize,
}

impl<T> Slice2DRaw<T> {
    unsafe fn from_raw_parts(
        slice: *const T,
        base_col: usize,
        row: usize,
        col: usize,
    ) -> Slice2DRaw<T> {
        Slice2DRaw {
            slice,
            base_col,
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

// impl<T> SlicePtr<T> for Slice2DRaw<T> {
//     fn get_slice_ptr(&self) -> *const T {
//         self.slice
//     }
// }
// impl<T> SlicePtrMut<T> for Slice2DRaw<T> {
//     fn get_slice_ptr_mut(&self) -> *mut T {
//         self.slice as *mut T
//     }
// }

pub trait Slice2DRawRef {
    type DataT;
    fn get_slice_2d_raw(&self) -> &Slice2DRaw<Self::DataT>;
}

pub trait Slice2DRawRefMut {
    type DataT;
    fn get_slice_2d_raw_mut(&mut self) -> &mut Slice2DRaw<Self::DataT>;
}

impl<T, S> Shape2D for S
where
    S: Slice2DRawRef<DataT = T>,
{
    #[inline(always)]
    fn get_base_col(&self) -> usize {
        self.get_slice_2d_raw().base_col
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
        self
    }
}
impl<T> Slice2DRawRefMut for Slice2DRaw<T> {
    type DataT = T;

    #[inline(always)]
    fn get_slice_2d_raw_mut(&mut self) -> &mut Slice2DRaw<Self::DataT> {
        self
    }
}

#[derive(Hash, Clone, Default, Debug)]
pub struct Slice2D<'a, T> {
    raw: Slice2DRaw<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Slice2D<'a, T> {
    pub fn from_slice(slice: &[T], row: usize, col: usize) -> Slice2D<'_, T> {
        assert!(
            row * col <= slice.len(),
            "slice does not contain enough space."
        );
        unsafe { Slice2D::from_raw_parts(slice.as_ptr(), col, row, col) }
    }
    pub unsafe fn from_raw_parts<'b>(
        slice: *const T,
        base_col: usize,
        row: usize,
        col: usize,
    ) -> Slice2D<'b, T> {
        Slice2D::from_slice_2d_raw(Slice2DRaw::from_raw_parts(slice, base_col, row, col))
    }
    pub unsafe fn from_slice_2d_raw<'b>(raw: Slice2DRaw<T>) -> Slice2D<'b, T> {
        Slice2D {
            raw,
            _marker: PhantomData,
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
impl<'a, T> SlicePtr<T> for Slice2D<'a, T> {
    fn get_slice_ptr(&self) -> *const T {
        self.raw.slice
    }
}
impl<'a, T> From<Slice2DMut<'a, T>> for Slice2D<'a, T> {
    fn from(s: Slice2DMut<'a, T>) -> Self {
        Slice2D {
            raw: s.raw,
            _marker: PhantomData,
        }
    }
}

#[derive(Hash, Default, Debug)]
pub struct Slice2DMut<'a, T> {
    raw: Slice2DRaw<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Slice2DMut<'a, T> {
    pub fn from_slice(slice: &mut [T], row: usize, col: usize) -> Slice2DMut<'_, T> {
        assert!(
            row * col <= slice.len(),
            "slice does not contain enough space."
        );
        unsafe { Slice2DMut::from_raw_parts(slice.as_mut_ptr(), col, row, col) }
    }
    pub unsafe fn from_raw_parts<'b>(
        slice: *mut T,
        base_col: usize,
        row: usize,
        col: usize,
    ) -> Slice2DMut<'b, T> {
        Slice2DMut::from_slice_2d_raw(Slice2DRaw::from_raw_parts(slice, base_col, row, col))
    }
    pub unsafe fn from_slice_2d_raw<'b>(raw: Slice2DRaw<T>) -> Slice2DMut<'b, T> {
        Slice2DMut {
            raw,
            _marker: PhantomData,
        }
    }
}
impl<'a, T> SlicePtr<T> for Slice2DMut<'a, T> {
    fn get_slice_ptr(&self) -> *const T {
        self.raw.slice
    }
}
impl<'a, T> SlicePtrMut<T> for Slice2DMut<'a, T> {
    fn get_slice_ptr_mut(&self) -> *mut T {
        self.raw.slice as *mut T
    }
}
impl<'a, T> Slice2DRawRef for Slice2DMut<'a, T> {
    type DataT = T;

    #[inline(always)]
    fn get_slice_2d_raw(&self) -> &Slice2DRaw<Self::DataT> {
        &self.raw
    }
}
impl<'a, T> Slice2DRawRefMut for Slice2DMut<'a, T> {
    type DataT = T;

    #[inline(always)]
    fn get_slice_2d_raw_mut(&mut self) -> &mut Slice2DRaw<Self::DataT> {
        &mut self.raw
    }
}
