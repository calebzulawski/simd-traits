//! Traits for generic SIMD operations.
#![no_std]
#![feature(portable_simd)]

mod mask;
mod vector;
pub use mask::*;
pub use vector::*;

pub mod num;
