//! The `libc` names `rav1d` uses, for `wasm32-unknown-unknown`, which has no
//! libc. See the note in `lib.rs`.
#![allow(non_camel_case_types)]

pub(crate) type ptrdiff_t = isize;
pub(crate) type intptr_t = isize;
pub(crate) type uintptr_t = usize;
pub(crate) type off_t = i64;

// `Rav1dError` uses these as discriminants and the C API negates them. No
// errno convention exists on this target, so they are Linux's values.
pub(crate) const ENOENT: i32 = 2;
pub(crate) const EAGAIN: i32 = 11;
pub(crate) const ENOMEM: i32 = 12;
pub(crate) const EINVAL: i32 = 22;
pub(crate) const ERANGE: i32 = 34;
pub(crate) const ENOPROTOOPT: i32 = 92;
