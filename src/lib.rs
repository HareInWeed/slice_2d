#![no_std]

pub mod slice;

pub mod cmp;
pub mod fill;
pub mod index;
pub mod iter;
pub mod split;
pub mod swap;
pub mod utils;

pub mod array_2d;
pub mod vec_2d;

pub mod prelude {
    pub use crate::slice::{Slice2D, Slice2DMut};

    pub use crate::fill::Slice2DFill;
    pub use crate::index::{GetElemRef, GetElemRefMut};
    pub use crate::iter::{Slice2DIter, Slice2DIterMut};
    pub use crate::slice::{Shape2D, Shape2DExt};
    pub use crate::split::{Split, SplitMut};
    pub use crate::swap::Slice2DSwap;
}

pub use crate::prelude::*;
