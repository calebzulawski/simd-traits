use core::{
    ops,
    simd::{LaneCount, Simd, SimdInt, SimdUint, SupportedLaneCount},
};

/// A vector of integer numbers.
pub trait Int:
    super::Num
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::BitXor<Output = Self>
    + ops::Shl<Output = Self>
    + ops::Shr<Output = Self>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign
    + ops::ShrAssign
{
}

macro_rules! impl_int {
    { $($type:ty),* } => {
        $(
        impl<const N: usize> Int for Simd<$type, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
        }
        )*
    }
}

impl_int! { i8, i16, i32, i64, u8, u16, u32, u64 }

/// A vector of signed integers.
pub trait SignedInt:
    Int + super::Signed + SimdInt<Scalar = <Self as crate::Vector>::Scalar>
{
}

macro_rules! impl_signedint {
    { $($type:ty),* } => {
        $(
        impl<const N: usize> SignedInt for Simd<$type, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
        }
        )*
    }
}

impl_signedint! { i8, i16, i32, i64 }

/// A vector of unsigned integers.
pub trait UnsignedInt:
    Int + super::Unsigned + SimdUint<Scalar = <Self as crate::Vector>::Scalar>
{
}

macro_rules! impl_signedint {
    { $($type:ty),* } => {
        $(
        impl<const N: usize> UnsignedInt for Simd<$type, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
        }
        )*
    }
}

impl_signedint! { u8, u16, u32, u64 }
