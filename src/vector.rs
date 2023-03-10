use core::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

/// A SIMD vector.
///
/// This is not necessarily a specific hardware type, but any kind of parallel collection.
pub trait Vector: Sized {
    type Scalar;
    const LANES: usize;

    fn splat(value: Self::Scalar) -> Self;

    unsafe fn extract_unchecked(&self, lane: usize) -> Self::Scalar;
    unsafe fn insert_unchecked(&mut self, lane: usize, value: Self::Scalar);

    fn extract(&self, lane: usize) -> Self::Scalar {
        assert!(lane < Self::LANES);
        unsafe { self.extract_unchecked(lane) }
    }

    fn insert(&mut self, lane: usize, value: Self::Scalar) {
        assert!(lane < Self::LANES);
        unsafe { self.insert_unchecked(lane, value) }
    }
}

impl<T, const N: usize> Vector for Simd<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Scalar = T;
    const LANES: usize = N;

    fn splat(value: Self::Scalar) -> Self {
        Simd::splat(value)
    }

    unsafe fn extract_unchecked(&self, lane: usize) -> Self::Scalar {
        *self.as_array().get_unchecked(lane)
    }

    unsafe fn insert_unchecked(&mut self, lane: usize, value: Self::Scalar) {
        *self.as_mut_array().get_unchecked_mut(lane) = value;
    }
}

impl<T, const N: usize> Vector for Mask<T, N>
where
    T: MaskElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Scalar = bool;
    const LANES: usize = N;

    fn splat(value: Self::Scalar) -> Self {
        Mask::splat(value)
    }

    unsafe fn extract_unchecked(&self, lane: usize) -> Self::Scalar {
        self.test_unchecked(lane)
    }

    unsafe fn insert_unchecked(&mut self, lane: usize, value: Self::Scalar) {
        self.set_unchecked(lane, value)
    }
}
