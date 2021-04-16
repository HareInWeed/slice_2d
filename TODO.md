# TODO

- [ ] FIXME: handle zst properly
- [x] `PartialEq` and `Eq` trait
- [ ] dedicated `Hash` impl for `Slice2D` & `Slice2DMut`
- [ ] iterator related
  - [ ] `Iterator` trait
    - [ ] `size_hint`
  - [ ] `ExactIterator` trait
  - [ ] `FusedIterator` trait
  - [ ] `IntoIterator` impl for `Slice2D` & `Slice2DMut`
- [ ] more split methods
  - [ ] `split_parts`
  - [ ] `split_at_multiple`
- [ ] more swap methods
  - [ ] `swap_chunks`
- [ ] 2d wrapper for `[T; N]` and `Vec<T>`
