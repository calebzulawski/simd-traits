use core::simd::{LaneCount, Simd, SimdFloat, SupportedLaneCount};

/// A vector of floating point numbers.
pub trait Float: super::Signed + SimdFloat<Scalar = <Self as crate::Vector>::Scalar> {}

macro_rules! impl_float {
    { $($type:ty),* } => {
        $(
        impl<const N: usize> Float for Simd<$type, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
        }
        )*
    }
}

impl_float! { f32, f64 }
