//! Cast element types.

use crate::Vector;
use core::simd::{LaneCount, Simd, SimdFloat, SimdInt, SimdUint, SupportedLaneCount};

/// Cast a vector's element type.
pub trait CastTo<T>: Vector {
    /// The output vector type.
    type Output: Vector<Scalar = T>;

    /// Cast the vector.
    fn cast_to(self) -> Self::Output;
}

macro_rules! impl_cast_to {
    { $($to:ty),* => $from:ty => $trait:ident } => {
        $(
        impl<const N: usize> CastTo<$to> for Simd<$from, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Simd<$to, N>;
            fn cast_to(self) -> Self::Output {
                $trait::cast::<$to>(self)
            }
        }
        )*
    };
    { $trait:ident for $($from:ty),* } => {
        $(
        impl_cast_to! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => $from => $trait }
        )*
    };
    {} => {
        impl_cast_to! { SimdUint for u8, u16, u32, u64, usize }
        impl_cast_to! { SimdInt for i8, i16, i32, i64, isize }
        impl_cast_to! { SimdFloat for f32, f64 }
    }
}

impl_cast_to! {}

/// Cast a vector's element to any primitive type.
pub trait CastPrimitive:
    CastTo<u8>
    + CastTo<u16>
    + CastTo<u32>
    + CastTo<u64>
    + CastTo<usize>
    + CastTo<i8>
    + CastTo<i16>
    + CastTo<i32>
    + CastTo<i64>
    + CastTo<isize>
    + CastTo<f32>
    + CastTo<f64>
{
}

impl<T> CastPrimitive for T where
    T: CastTo<u8>
        + CastTo<u16>
        + CastTo<u32>
        + CastTo<u64>
        + CastTo<usize>
        + CastTo<i8>
        + CastTo<i16>
        + CastTo<i32>
        + CastTo<i64>
        + CastTo<isize>
        + CastTo<f32>
        + CastTo<f64>
{
}
