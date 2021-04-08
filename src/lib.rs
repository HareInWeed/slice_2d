pub mod index;
pub mod slice;
pub use slice::{Slice2D, Slice2DMut};

#[cfg(test)]
mod tests {
    use crate::slice::{Slice2D, Slice2DMut};
    #[test]
    fn slice_2d_index() {
        const ROW: usize = 3;
        const COL: usize = 5;
        let v = (0..(ROW * COL) as i32).collect::<Vec<_>>();
        let s = Slice2D::from_slice(v.as_slice(), ROW, COL);
        for i in 0..ROW {
            for j in 0..COL {
                assert_eq!(s[(i, j)], (i * COL + j) as i32);
                assert_eq!(s.get((i, j)), Some(&((i * COL + j) as i32)));
            }
        }
        assert_eq!(s.get((0, COL)), None);
        assert_eq!(s.get((ROW, 0)), None);
        assert_eq!(s.get((ROW, COL)), None);
    }

    #[test]
    fn slice_2d_mut_index() {
        const ROW: usize = 3;
        const COL: usize = 5;
        let mut v = (0..(ROW * COL) as i32).collect::<Vec<_>>();
        let mut s = Slice2DMut::from_slice(v.as_mut_slice(), ROW, COL);
        for i in 0..ROW {
            for j in 0..COL {
                assert_eq!(s[(i, j)], (i * COL + j) as i32);
                assert_eq!(s.get((i, j)), Some(&((i * COL + j) as i32)));
            }
        }
        assert_eq!(s.get((0, COL)), None);
        assert_eq!(s.get((ROW, 0)), None);
        assert_eq!(s.get((ROW, COL)), None);
        for i in 0..ROW {
            for j in 0..COL {
                s[(i, j)] += 1;
            }
        }
        for i in 0..ROW {
            for j in 0..COL {
                assert_eq!(s[(i, j)], (i * COL + j) as i32 + 1);
                assert_eq!(s.get((i, j)), Some(&((i * COL + j) as i32 + 1)));
            }
        }
        assert_eq!(s.get((0, COL)), None);
        assert_eq!(s.get((ROW, 0)), None);
        assert_eq!(s.get((ROW, COL)), None);
    }

    #[test]
    fn slice_2d_slice() {
        use core::ops::Bound::*;
        const ROW: usize = 3;
        const COL: usize = 5;
        const RS: usize = 1;
        const RE: usize = ROW - 1;
        const CS: usize = 2;
        const CE: usize = COL - 1;
        let v = (0..(ROW * COL) as i32).collect::<Vec<_>>();
        let hs = Slice2D::from_slice(v.as_slice(), ROW, COL);

        // (RangeBounds, usize)
        let s = hs.get((RS..RE, CS)).unwrap();
        for i in 0..RE - RS {
            assert_eq!(s.get((i, 0)), Some(&(((RS + i) * COL + CS) as i32)));
        }
        assert_eq!(s.get((0, CS)), None);
        assert_eq!(s.get((RE - RS, 0)), None);
        assert_eq!(s.get((RE - RS, CS)), None);

        let s = hs.get(((Included(RS), Included(RE)), CS)).unwrap();
        for i in 0..RE - RS + 1 {
            assert_eq!(s.get((i, 0)), Some(&(((RS + i) * COL + CS) as i32)));
        }
        assert_eq!(s.get((0, CS)), None);
        assert_eq!(s.get((RE - RS + 1, 0)), None);
        assert_eq!(s.get((RE - RS + 1, CS)), None);

        // (usize, RangeBounds)
        let s = hs.get((RS, CS..CE)).unwrap();
        for j in 0..CE - CS {
            assert_eq!(s.get((0, j)), Some(&((RS * COL + CS + j) as i32)));
        }
        assert_eq!(s.get((0, CE - CS)), None);
        assert_eq!(s.get((RS, 0)), None);
        assert_eq!(s.get((RS, CE - CS)), None);

        let s = hs.get((RS, (Included(CS), Included(CE)))).unwrap();
        for j in 0..CE - CS + 1 {
            assert_eq!(s.get((0, j)), Some(&((RS * COL + CS + j) as i32)));
        }
        assert_eq!(s.get((0, CE - CS + 1)), None);
        assert_eq!(s.get((RS, 0)), None);
        assert_eq!(s.get((RS, CE - CS + 1)), None);

        // (RangeBounds, RangeBounds)
        let s = hs.get((RS..RE, CS..CE)).unwrap();
        for i in 0..RE - RS {
            for j in 0..CE - CS {
                assert_eq!(s.get((i, j)), Some(&(((RS + i) * COL + CS + j) as i32)));
            }
        }
        assert_eq!(s.get((0, CE - CS)), None);
        assert_eq!(s.get((RE - RS, 0)), None);
        assert_eq!(s.get((RE - RS, CE - CS)), None);

        let s = hs
            .get(((Included(RS), Included(RE)), (Included(CS), Included(CE))))
            .unwrap();
        for i in 0..RE - RS + 1 {
            for j in 0..CE - CS + 1 {
                assert_eq!(s.get((i, j)), Some(&(((RS + i) * COL + CS + j) as i32)));
            }
        }
        assert_eq!(s.get((0, CE - CS + 1)), None);
        assert_eq!(s.get((RE - RS + 1, 0)), None);
        assert_eq!(s.get((RE - RS + 1, CE - CS + 1)), None);
    }
}