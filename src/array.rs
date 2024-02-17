use core::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

/// A vector that has array-like layout.
///
/// # Safety
/// The layout of this vector must be array-like.  More specifically, references to this vector
/// must be transmutable to slices.
pub unsafe trait Array: crate::Vector + AsRef<[Self::Scalar]> + AsMut<[Self::Scalar]> {}

unsafe impl<T, const N: usize> Array for Simd<T, N>
where
    T: SimdElement,
    T::Mask: PartialEq,
    LaneCount<N>: SupportedLaneCount,
{
}
