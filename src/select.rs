use crate::{Mask, Vector};
use core::simd::{LaneCount, MaskElement, Simd, SimdElement, SupportedLaneCount};

/// Select vector elements via a mask.
pub trait Select<V>: Mask
where
    V: Vector,
{
    fn select(&self, true_values: V, false_values: V) -> V;
}

impl<T, const N: usize> Select<Simd<T, N>> for core::simd::Mask<T::Mask, N>
where
    T: SimdElement,
    T::Mask: PartialEq,
    LaneCount<N>: SupportedLaneCount,
{
    fn select(&self, true_values: Simd<T, N>, false_values: Simd<T, N>) -> Simd<T, N> {
        core::simd::Mask::select(*self, true_values, false_values)
    }
}

impl<T, const N: usize> Select<Self> for core::simd::Mask<T, N>
where
    T: MaskElement + PartialEq,
    LaneCount<N>: SupportedLaneCount,
{
    fn select(&self, true_values: Self, false_values: Self) -> Self {
        self.select_mask(true_values, false_values)
    }
}
