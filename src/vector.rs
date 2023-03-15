use core::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

/// A SIMD vector.
///
/// This is not necessarily a specific hardware type, but any kind of parallel collection.
///
/// # Safety
/// `ELEMENTS` must correspond to the number of elements in the vector.
pub unsafe trait Vector: Sized {
    /// The scalar type contained by this vector.
    type Scalar;

    /// The type that masks this vector.
    type Mask: crate::Mask;

    /// The number of elements contained by this vector.
    const ELEMENTS: usize;

    /// Create a new vector filled with this scalar.
    fn splat(value: Self::Scalar) -> Self;

    /// Extract an element.
    ///
    /// # Safety
    /// `index` must be less than `ELEMENTS`.
    unsafe fn extract_unchecked(&self, index: usize) -> Self::Scalar;

    /// Replace an element.
    ///
    /// # Safety
    /// `index` must be less than `ELEMENTS`.
    unsafe fn insert_unchecked(&mut self, index: usize, value: Self::Scalar);

    /// Extract an element.
    fn extract(&self, index: usize) -> Self::Scalar {
        assert!(index < Self::ELEMENTS);
        unsafe { self.extract_unchecked(index) }
    }

    /// Replace an element.
    fn insert(&mut self, index: usize, value: Self::Scalar) {
        assert!(index < Self::ELEMENTS);
        unsafe { self.insert_unchecked(index, value) }
    }

    /// Create a new vector by selecting conditionally from two vectors.
    ///
    /// For every true value in `mask`, select from `true_values`, otherwise select from
    /// `false_values`.
    ///
    /// See [`Mask::select`], which may be more ergonomic.
    fn select(mask: &Self::Mask, true_values: Self, false_values: Self) -> Self;
}

unsafe impl<T, const N: usize> Vector for Simd<T, N>
where
    T: SimdElement,
    T::Mask: PartialEq,
    LaneCount<N>: SupportedLaneCount,
{
    type Scalar = T;
    type Mask = Mask<T::Mask, N>;
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

    fn select(mask: &Self::Mask, true_values: Self, false_values: Self) -> Self {
        Mask::select(*mask, true_values, false_values)
    }
}

unsafe impl<T, const N: usize> Vector for Mask<T, N>
where
    T: MaskElement + PartialEq,
    LaneCount<N>: SupportedLaneCount,
{
    type Scalar = bool;
    type Mask = Self;
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

    fn select(mask: &Self::Mask, true_values: Self, false_values: Self) -> Self {
        Mask::select_mask(*mask, true_values, false_values)
    }
}
