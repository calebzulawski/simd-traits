//! Numeric traits for generic SIMD mathematics.

mod float;
mod int;

pub mod cast;
pub use float::*;
pub use int::*;

use crate::Vector;
use core::{
    ops,
    simd::{LaneCount, Simd, SupportedLaneCount},
};

/// A vector of numbers.
pub trait Num:
    Vector
    + PartialEq
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + ops::AddAssign
    + ops::SubAssign
    + ops::MulAssign
    + ops::DivAssign
    + ops::RemAssign
{
    /// Create a vector containing all zeros.
    fn zero() -> Self;

    /// Create a vector containing all ones.
    fn one() -> Self;
}

macro_rules! impl_num {
    { $($type:ty),* } => {
        $(
        impl<const N: usize> Num for Simd<$type, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            fn zero() -> Self {
                Simd::splat(0 as $type)
            }

            fn one() -> Self {
                Simd::splat(1 as $type)
            }
        }
        )*
    }
}

impl_num! { i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 }

/// A vector of signed numbers.
pub trait Signed: Num + ops::Neg<Output = Self> {}

macro_rules! impl_signed {
    { $($type:ty),* } => {
        $(
        impl<const N: usize> Signed for Simd<$type, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
        }
        )*
    }
}

impl_signed! { i8, i16, i32, i64, f32, f64 }

/// A vector of unsigned numbers.
pub trait Unsigned: Num {}

macro_rules! impl_unsigned {
    { $($type:ty),* } => {
        $(
        impl<const N: usize> Unsigned for Simd<$type, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
        }
        )*
    }
}

impl_unsigned! { u8, u16, u32, u64 }
