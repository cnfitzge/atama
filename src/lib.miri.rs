//! Grep bootstrap for `MIRI_REPLACE_LIBRS_IF_NOT_TEST` to learn what this is about.
#![no_std]
#![feature(rustc_private)]
extern crate atama as real;
pub use real::*;

