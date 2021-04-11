use core::panic;

use crate::{
    slice::{Shape2D, Slice2DMut, SlicePtrMut},
    utils::calc_2d_index,
};

pub trait Slice2DSwap {
    fn swap(&mut self, idx1: (usize, usize), idx2: (usize, usize));
}

impl<'a, T> Slice2DSwap for Slice2DMut<'a, T> {
    fn swap(&mut self, idx1: (usize, usize), idx2: (usize, usize)) {
        unsafe {
            // we must check the boundary here, because some indices out
            // of boundary still refer to valid address, just not hold
            // by the slice
            if idx1.0 < self.get_row()
                && idx1.1 < self.get_col()
                && idx2.0 < self.get_row()
                && idx2.1 < self.get_col()
            {
                let ptr1 = self
                    .get_slice_ptr_mut()
                    .add(calc_2d_index(idx1.0, idx1.1, self));
                let ptr2 = self
                    .get_slice_ptr_mut()
                    .add(calc_2d_index(idx2.0, idx2.1, self));
                core::ptr::swap(ptr1, ptr2);
            } else {
                panic!("out of boundary");
            }
        }
    }
}
