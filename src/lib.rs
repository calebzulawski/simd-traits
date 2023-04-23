//! Traits for generic SIMD operations.
#![no_std]
#![feature(portable_simd)]

mod array;
mod mask;
mod vector;
pub use array::*;
pub use mask::*;
pub use vector::*;

pub mod num;
pub mod swizzle;
