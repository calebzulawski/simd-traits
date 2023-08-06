//! Cast element types.

use crate::Vector;
use core::simd::{LaneCount, Simd, SimdFloat, SimdInt, SimdUint, SupportedLaneCount};

/// Cast a vector's element type.
pub trait CastFrom<T: Vector>: Vector {
    /// Cast the vector.
    fn cast_from(from: T) -> Self;
}

macro_rules! impl_cast_from {
    { $($to:ty),* => $from:ty => $trait:ident } => {
        $(
        impl<const N: usize> CastFrom<Simd<$from, N>> for Simd<$to, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            fn cast_from(from: Simd<$from, N>) -> Self {
                $trait::cast::<$to>(from)
            }
        }
        )*
    };
    { $trait:ident for $($from:ty),* } => {
        $(
        impl_cast_from! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => $from => $trait }
        )*
    };
    {} => {
        impl_cast_from! { SimdUint for u8, u16, u32, u64, usize }
        impl_cast_from! { SimdInt for i8, i16, i32, i64, isize }
        impl_cast_from! { SimdFloat for f32, f64 }
    }
}

impl_cast_from! {}

/// Cast a vector's element type.
pub trait CastTo<T: Vector>: Vector {
    /// Cast the vector.
    fn cast_to(self) -> T;
}

impl<T, U> CastTo<T> for U
where
    U: Vector,
    T: CastFrom<U>,
{
    fn cast_to(self) -> T {
        T::cast_from(self)
    }
}
