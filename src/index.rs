use crate::slice::{ArrayRef, ArrayRefMut, Slice2DGeometry};

pub trait Slice2DIndex<'a, T, S>
where
    S: Slice2DGeometry + ArrayRef<T>,
{
    type Output: ?Sized + 'a;
    unsafe fn get_unchecked(self, slice: &S) -> *const Self::Output;
    fn get(self, slice: &S) -> Option<&'a Self::Output>;
    fn index(self, slice: &S) -> &'a Self::Output;
}

pub trait Slice2DIndexMut<'a, T, S>: Slice2DIndex<'a, T, S>
where
    S: Slice2DGeometry + ArrayRef<T> + ArrayRefMut<T>,
{
    unsafe fn get_unchecked_mut(self, slice: &mut S) -> *mut Self::Output;
    fn get_mut(self, slice: &mut S) -> Option<&'a mut Self::Output>;
    fn index_mut(self, slice: &mut S) -> &'a mut Self::Output;
}

// index (usize, usize)

#[inline(always)]
fn get_2d_index<S: Slice2DGeometry>(idx: (usize, usize), slice: &S) -> usize {
    idx.0 * slice.get_array_col() + idx.1
}

impl<'a, T: 'a, S> Slice2DIndex<'a, T, S> for (usize, usize)
where
    S: Slice2DGeometry + ArrayRef<T>,
{
    type Output = T;

    #[inline(always)]
    fn get(self, slice: &S) -> Option<&'a Self::Output> {
        unsafe {
            if self.0 < slice.get_row() && self.1 < slice.get_col() {
                Some(self.get_unchecked(slice).as_ref().unwrap())
            } else {
                None
            }
        }
    }

    #[inline(always)]
    unsafe fn get_unchecked(self, slice: &S) -> *const Self::Output {
        slice.get_array().add(get_2d_index(self, slice))
    }

    #[inline(always)]
    fn index(self, slice: &S) -> &'a Self::Output {
        self.get(slice).expect("out of boundary")
    }
}

impl<'a, T: 'a, S> Slice2DIndexMut<'a, T, S> for (usize, usize)
where
    S: Slice2DGeometry + ArrayRef<T> + ArrayRefMut<T>,
{
    #[inline(always)]
    fn get_mut(self, slice: &mut S) -> Option<&'a mut Self::Output> {
        unsafe {
            if self.0 < slice.get_row() && self.1 < slice.get_col() {
                Some(self.get_unchecked_mut(slice).as_mut().unwrap())
            } else {
                None
            }
        }
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(self, slice: &mut S) -> *mut Self::Output {
        slice.get_array_mut().add(get_2d_index(self, slice))
    }

    #[inline(always)]
    fn index_mut(self, slice: &mut S) -> &'a mut Self::Output {
        self.get_mut(slice).expect("out of boundary")
    }
}
