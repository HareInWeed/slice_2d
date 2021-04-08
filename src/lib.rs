pub mod index;
pub mod slice;

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
}
