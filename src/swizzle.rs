//! Operations for rearranging SIMD vectors.

use crate::Vector;
use core::simd::{prelude::*, LaneCount, SimdElement, SupportedLaneCount};

/// Rearrange a vector.
///
/// ```
/// # #![feature(portable_simd)]
/// use core::simd::Simd;
/// use simd_traits::swizzle::swizzle;
///
/// let a = Simd::from_array([1, 2, 3, 4]);
/// let b = Simd::from_array([5, 6, 7, 8]);
///
/// let c = swizzle!(a, &[0, 3, 0, 1]);
/// assert_eq!(c.to_array(), [1, 4, 1, 2]);
///
/// let d = swizzle!(a, b, &[0, 5, 0, 6]);
/// assert_eq!(d.to_array(), [1, 6, 1, 7]);
/// ```
#[macro_export]
macro_rules! swizzle {
    { $vector:expr, $index:expr } => {
        {
            struct __Index;
            impl $crate::swizzle::SwizzleIndex for __Index {
                const INDEX: &'static [usize] = $index;
            }
            $crate::swizzle::Swizzle::swizzle::<__Index>($vector)
        }
    };
    { $vector1:expr, $vector2:expr, $index:expr } => {
        {
            struct __Index;
            impl $crate::swizzle::SwizzleIndex for __Index {
                const INDEX: &'static [usize] = $index;
            }
            $crate::swizzle::Swizzle::concat_swizzle::<__Index>($vector1, $vector2)
        }
    }
}
pub use swizzle;

/// Indices of the source vector used to create a new vector.
pub trait SwizzleIndex {
    const INDEX: &'static [usize];
}

/// Rearrange a vector, possibly changing its size.
pub trait Swizzle<To>: Vector
where
    To: Vector<Scalar = Self::Scalar>,
{
    /// Create a new vector by selecting elements of this vector.
    fn swizzle<I: SwizzleIndex>(self) -> To;

    /// Create a new vector by selecting elements from the concatenation of this vector with
    /// another.
    fn concat_swizzle<I: SwizzleIndex>(self, other: Self) -> To;
}

const fn to_array<const N: usize>(slice: &[usize]) -> [usize; N] {
    assert!(slice.len() == N, "incorrect number of swizzle indices");

    let mut array = [0; N];
    let mut i = 0;
    while i < N {
        array[i] = slice[i];
        i += 1;
    }
    array
}

impl<T, const N: usize, const M: usize> Swizzle<Simd<T, M>> for Simd<T, N>
where
    T: SimdElement,
    T::Mask: PartialEq,
    LaneCount<N>: SupportedLaneCount,
    LaneCount<M>: SupportedLaneCount,
{
    fn swizzle<I: SwizzleIndex>(self) -> Simd<T, M> {
        use core::simd::Swizzle;
        struct Impl<const LEN: usize, I: SwizzleIndex>(I);
        impl<const LEN: usize, I: SwizzleIndex> Swizzle<LEN> for Impl<LEN, I> {
            const INDEX: [usize; LEN] = to_array(I::INDEX);
        }
        Impl::<M, I>::swizzle(self)
    }

    fn concat_swizzle<I: SwizzleIndex>(self, other: Self) -> Simd<T, M> {
        use core::simd::Swizzle;
        struct Impl<const LEN: usize, I: SwizzleIndex>(I);
        impl<const LEN: usize, I: SwizzleIndex> Swizzle<LEN> for Impl<LEN, I> {
            const INDEX: [usize; LEN] = to_array(I::INDEX);
        }
        Impl::<M, I>::concat_swizzle(self, other)
    }
}

/// Rearrange a vector.
pub trait Shuffle: Vector {
    fn reverse(self) -> Self;
    fn interleave(self, other: Self) -> (Self, Self);
    fn deinterleave(self, other: Self) -> (Self, Self);
}

impl<T, const N: usize> Shuffle for Simd<T, N>
where
    T: SimdElement,
    T::Mask: PartialEq,
    LaneCount<N>: SupportedLaneCount,
{
    fn reverse(self) -> Self {
        Simd::reverse(self)
    }

    fn interleave(self, other: Self) -> (Self, Self) {
        // https://github.com/rust-lang/portable-simd/issues/298
        if Self::ELEMENTS == 1 {
            (self, other)
        } else {
            Simd::interleave(self, other)
        }
    }

    fn deinterleave(self, other: Self) -> (Self, Self) {
        // https://github.com/rust-lang/portable-simd/issues/298
        if Self::ELEMENTS == 1 {
            (self, other)
        } else {
            Simd::deinterleave(self, other)
        }
    }
}
