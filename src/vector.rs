use core::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

/// A SIMD vector.
///
/// This is not necessarily a specific hardware type, but any kind of parallel collection.
///
/// # Safety
/// `ELEMENTS` must correspond to the number of elements in the vector.
pub unsafe trait Vector: Sized {
    type Scalar;
    const ELEMENTS: usize;

    fn splat(value: Self::Scalar) -> Self;

    unsafe fn extract_unchecked(&self, index: usize) -> Self::Scalar;
    unsafe fn insert_unchecked(&mut self, index: usize, value: Self::Scalar);

    fn extract(&self, index: usize) -> Self::Scalar {
        assert!(index < Self::ELEMENTS);
        unsafe { self.extract_unchecked(index) }
    }

    fn insert(&mut self, index: usize, value: Self::Scalar) {
        assert!(index < Self::ELEMENTS);
        unsafe { self.insert_unchecked(index, value) }
    }
}

unsafe impl<T, const N: usize> Vector for Simd<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Scalar = T;
    const ELEMENTS: usize = N;

    fn splat(value: Self::Scalar) -> Self {
        Simd::splat(value)
    }

    unsafe fn extract_unchecked(&self, index: usize) -> Self::Scalar {
        *self.as_array().get_unchecked(index)
    }

    unsafe fn insert_unchecked(&mut self, index: usize, value: Self::Scalar) {
        *self.as_mut_array().get_unchecked_mut(index) = value;
    }
}

unsafe impl<T, const N: usize> Vector for Mask<T, N>
where
    T: MaskElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Scalar = bool;
    const ELEMENTS: usize = N;

    fn splat(value: Self::Scalar) -> Self {
        Mask::splat(value)
    }

    unsafe fn extract_unchecked(&self, index: usize) -> Self::Scalar {
        self.test_unchecked(index)
    }

    unsafe fn insert_unchecked(&mut self, index: usize, value: Self::Scalar) {
        self.set_unchecked(index, value)
    }
}
