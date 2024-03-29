use crate::Vector;
use core::{
    ops,
    simd::{LaneCount, MaskElement, SupportedLaneCount},
};

/// A vector mask.
pub trait Mask:
    Vector<Scalar = bool>
    + PartialEq
    + ops::Not<Output = Self>
    + ops::BitAnd<Output = Self>
    + ops::BitAndAssign
    + ops::BitOr<Output = Self>
    + ops::BitOrAssign
    + ops::BitXor<Output = Self>
    + ops::BitXorAssign
{
    /// Conditionally select from two vectors.
    ///
    /// For every true value, select from `true_values`, otherwise select from
    /// `false_values`.
    fn select<V>(&self, true_value: V, false_value: V) -> V
    where
        V: Vector<Mask = Self>,
    {
        Vector::select(self, true_value, false_value)
    }
}

impl<T, const N: usize> Mask for core::simd::Mask<T, N>
where
    T: MaskElement + PartialEq,
    LaneCount<N>: SupportedLaneCount,
{
}
