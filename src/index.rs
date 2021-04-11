use crate::{
    slice::{Shape2D, Slice2D, Slice2DMut, SlicePtr, SlicePtrMut},
    utils::{calc_2d_index, calc_2d_range},
};
use core::ops::{
    Bound, Index, IndexMut, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};

pub unsafe trait Slice2DIndex<'a, T, S>
where
    S: Shape2D + SlicePtr<T> + ?Sized,
{
    type Ref: 'a;
    unsafe fn get_unchecked(self, slice: &'a S) -> Self::Ref;
    fn get(self, slice: &'a S) -> Option<Self::Ref>;
    fn index(self, slice: &'a S) -> Self::Ref;
}

pub unsafe trait Slice2DIndexMut<'a, T, S>
where
    S: Shape2D + SlicePtrMut<T> + ?Sized,
{
    type RefMut: 'a;
    unsafe fn get_unchecked_mut(self, slice: &'a mut S) -> Self::RefMut;
    fn get_mut(self, slice: &'a mut S) -> Option<Self::RefMut>;
    fn index_mut(self, slice: &'a mut S) -> Self::RefMut;
}

// index (usize, usize)

unsafe impl<'a, T: 'a, S> Slice2DIndex<'a, T, S> for (usize, usize)
where
    S: Shape2D + SlicePtr<T>,
{
    type Ref = &'a T;

    #[inline(always)]
    unsafe fn get_unchecked(self, slice: &S) -> Self::Ref {
        &*slice
            .get_slice_ptr()
            .add(calc_2d_index(self.0, self.1, slice))
    }

    #[inline(always)]
    fn get(self, slice: &'a S) -> Option<Self::Ref> {
        unsafe {
            if self.0 < slice.get_row() && self.1 < slice.get_col() {
                Some(self.get_unchecked(slice))
            } else {
                None
            }
        }
    }

    #[inline(always)]
    fn index(self, slice: &'a S) -> Self::Ref {
        self.get(slice).expect("out of range")
    }
}

unsafe impl<'a, T: 'a, S> Slice2DIndexMut<'a, T, S> for (usize, usize)
where
    S: Shape2D + SlicePtrMut<T>,
{
    type RefMut = &'a mut T;

    #[inline(always)]
    unsafe fn get_unchecked_mut(self, slice: &mut S) -> Self::RefMut {
        &mut *slice
            .get_slice_ptr_mut()
            .add(calc_2d_index(self.0, self.1, slice))
    }

    #[inline(always)]
    fn get_mut(self, slice: &'a mut S) -> Option<Self::RefMut> {
        unsafe {
            if self.0 < slice.get_row() && self.1 < slice.get_col() {
                Some(self.get_unchecked_mut(slice))
            } else {
                None
            }
        }
    }

    #[inline(always)]
    fn index_mut(self, slice: &'a mut S) -> Self::RefMut {
        self.get_mut(slice).expect("out of range")
    }
}

// index Range
pub trait IRange: RangeBounds<usize> {}
impl<'a> IRange for (Bound<&'a usize>, Bound<&'a usize>) {}
impl IRange for (Bound<usize>, Bound<usize>) {}
impl IRange for Range<&usize> {}
impl IRange for Range<usize> {}
impl IRange for RangeFrom<&usize> {}
impl IRange for RangeFrom<usize> {}
impl IRange for RangeInclusive<&usize> {}
impl IRange for RangeInclusive<usize> {}
impl IRange for RangeTo<&usize> {}
impl IRange for RangeTo<usize> {}
impl IRange for RangeToInclusive<&usize> {}
impl IRange for RangeToInclusive<usize> {}
impl IRange for RangeFull {}

// (RangeBounds, RangeBounds)
unsafe impl<'a, T: 'a, S, B1, B2> Slice2DIndex<'a, T, S> for (B1, B2)
where
    S: Shape2D + SlicePtr<T>,
    B1: IRange,
    B2: IRange,
{
    type Ref = Slice2D<'a, T>;

    unsafe fn get_unchecked(self, slice: &S) -> Self::Ref {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        Slice2D::<T>::from_raw_parts(
            (rs, cs).get_unchecked(slice),
            slice.get_base_col(),
            re - rs,
            ce - cs,
        )
    }

    fn get(self, slice: &'a S) -> Option<Self::Ref> {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        unsafe {
            if rs < slice.get_row()
                && re <= slice.get_row()
                && cs < slice.get_col()
                && ce <= slice.get_col()
            {
                Some(self.get_unchecked(slice))
            } else {
                None
            }
        }
    }

    fn index(self, slice: &'a S) -> Self::Ref {
        self.get(slice).expect("out of range")
    }
}

unsafe impl<'a, T: 'a, S, B1, B2> Slice2DIndexMut<'a, T, S> for (B1, B2)
where
    S: Shape2D + SlicePtrMut<T>,
    B1: IRange,
    B2: IRange,
{
    type RefMut = Slice2DMut<'a, T>;

    unsafe fn get_unchecked_mut(self, slice: &mut S) -> Self::RefMut {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        Slice2DMut::<T>::from_raw_parts(
            (rs, cs).get_unchecked_mut(slice),
            slice.get_base_col(),
            re - rs,
            ce - cs,
        )
    }

    fn get_mut(self, slice: &'a mut S) -> Option<Self::RefMut> {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        unsafe {
            if rs < slice.get_row()
                && re <= slice.get_row()
                && cs < slice.get_col()
                && ce <= slice.get_col()
            {
                Some(self.get_unchecked_mut(slice))
            } else {
                None
            }
        }
    }

    fn index_mut(self, slice: &'a mut S) -> Self::RefMut {
        self.get_mut(slice).expect("out of range")
    }
}

// (RangeBounds, usize)
unsafe impl<'a, T: 'a, S, B> Slice2DIndex<'a, T, S> for (B, usize)
where
    S: Shape2D + SlicePtr<T>,
    B: IRange,
{
    type Ref = Slice2D<'a, T>;

    unsafe fn get_unchecked(self, slice: &S) -> Self::Ref {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        Slice2D::<T>::from_raw_parts(
            (rs, self.1).get_unchecked(slice),
            slice.get_base_col(),
            re - rs,
            1,
        )
    }

    fn get(self, slice: &'a S) -> Option<Self::Ref> {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        unsafe {
            if rs < slice.get_row() && re <= slice.get_row() && self.1 < slice.get_col() {
                Some(self.get_unchecked(slice))
            } else {
                None
            }
        }
    }

    fn index(self, slice: &'a S) -> Self::Ref {
        self.get(slice).expect("out of range")
    }
}

unsafe impl<'a, T: 'a, S, B> Slice2DIndexMut<'a, T, S> for (B, usize)
where
    S: Shape2D + SlicePtrMut<T>,
    B: IRange,
{
    type RefMut = Slice2DMut<'a, T>;

    unsafe fn get_unchecked_mut(self, slice: &mut S) -> Self::RefMut {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        Slice2DMut::<T>::from_raw_parts(
            (rs, self.1).get_unchecked_mut(slice),
            slice.get_base_col(),
            re - rs,
            1,
        )
    }

    fn get_mut(self, slice: &'a mut S) -> Option<Self::RefMut> {
        let (rs, re) = calc_2d_range(slice.get_row(), &self.0);
        unsafe {
            if rs < slice.get_row() && re <= slice.get_row() && self.1 < slice.get_col() {
                Some(self.get_unchecked_mut(slice))
            } else {
                None
            }
        }
    }

    fn index_mut(self, slice: &'a mut S) -> Self::RefMut {
        self.get_mut(slice).expect("out of range")
    }
}

// (usize, RangeBounds)
unsafe impl<'a, T: 'a, S, B> Slice2DIndex<'a, T, S> for (usize, B)
where
    S: Shape2D + SlicePtr<T>,
    B: IRange,
{
    type Ref = Slice2D<'a, T>;

    unsafe fn get_unchecked(self, slice: &'a S) -> Self::Ref {
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        Slice2D::<T>::from_raw_parts(
            (self.0, cs).get_unchecked(slice),
            slice.get_base_col(),
            1,
            ce - cs,
        )
    }

    fn get(self, slice: &'a S) -> Option<Self::Ref> {
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        unsafe {
            if self.0 < slice.get_row() && cs < slice.get_col() && ce <= slice.get_col() {
                Some(self.get_unchecked(slice))
            } else {
                None
            }
        }
    }

    fn index(self, slice: &'a S) -> Self::Ref {
        self.get(slice).expect("out of range")
    }
}

unsafe impl<'a, T: 'a, S, B> Slice2DIndexMut<'a, T, S> for (usize, B)
where
    S: Shape2D + SlicePtrMut<T>,
    B: IRange,
{
    type RefMut = Slice2DMut<'a, T>;

    unsafe fn get_unchecked_mut(self, slice: &mut S) -> Self::RefMut {
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        Slice2DMut::<T>::from_raw_parts(
            (self.0, cs).get_unchecked_mut(slice),
            slice.get_base_col(),
            1,
            ce - cs,
        )
    }

    fn get_mut(self, slice: &'a mut S) -> Option<Self::RefMut> {
        let (cs, ce) = calc_2d_range(slice.get_col(), &self.1);
        unsafe {
            if self.0 < slice.get_row() && cs < slice.get_col() && ce <= slice.get_col() {
                Some(self.get_unchecked_mut(slice))
            } else {
                None
            }
        }
    }

    fn index_mut(self, slice: &'a mut S) -> Self::RefMut {
        self.get_mut(slice).expect("out of range")
    }
}

// get reference of part of Slice2D
pub trait GetElemRef<'a, T>: Shape2D + SlicePtr<T> {
    fn get<I>(&'a self, index: I) -> Option<I::Ref>
    where
        I: Slice2DIndex<'a, T, Self>;
    unsafe fn get_unchecked<I>(&'a self, index: I) -> I::Ref
    where
        I: Slice2DIndex<'a, T, Self>;
}

pub trait GetElemRefMut<'a, T>: Shape2D + SlicePtrMut<T> {
    fn get_mut<I>(&'a mut self, index: I) -> Option<I::RefMut>
    where
        I: Slice2DIndexMut<'a, T, Self>;
    unsafe fn get_unchecked_mut<I>(&'a mut self, index: I) -> I::RefMut
    where
        I: Slice2DIndexMut<'a, T, Self>;
}

impl<'a, T, S> GetElemRef<'a, T> for S
where
    S: Shape2D + SlicePtr<T>,
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

impl<'a, T, S> GetElemRefMut<'a, T> for S
where
    S: Shape2D + SlicePtrMut<T>,
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
