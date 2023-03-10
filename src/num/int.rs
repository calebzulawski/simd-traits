use core::{
    ops,
    simd::{LaneCount, Simd, SupportedLaneCount},
};

/// A vector of integer numbers.
pub trait Int:
    crate::Num
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
