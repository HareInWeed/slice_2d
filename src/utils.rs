use crate::slice::Shape2D;
use core::ops::{Bound, RangeBounds};

#[inline(always)]
pub fn calc_2d_index<S: Shape2D>(r: usize, c: usize, slice: &S) -> usize {
    r * slice.get_base_col() + c
}

pub fn calc_2d_range<B: RangeBounds<usize>>(len: usize, bound: &B) -> (usize, usize) {
    (
        match bound.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        },
        match bound.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => len,
        },
    )
}
