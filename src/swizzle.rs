use crate::Vector;
use core::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

pub trait SwizzleIndex {
    const INDEX: &'static [usize];
}

pub trait Swizzle<To>: Vector {
    fn swizzle<I: SwizzleIndex>(self) -> To;
}

impl<T, const N: usize, const M: usize> Swizzle<Simd<T, M>> for Simd<T, N>
where
    T: SimdElement,
    T::Mask: PartialEq,
    LaneCount<N>: SupportedLaneCount,
    LaneCount<M>: SupportedLaneCount,
{
    fn swizzle<I: SwizzleIndex>(self) -> Simd<T, M> {
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

        use core::simd::Swizzle;
        struct Impl<const FROM: usize, const TO: usize, I: SwizzleIndex>(I);
        impl<const FROM: usize, const TO: usize, I: SwizzleIndex> Swizzle<FROM, TO> for Impl<FROM, TO, I> {
            const INDEX: [usize; TO] = to_array(&I::INDEX);
        }
        Impl::<N, M, I>::swizzle(self)
    }
}

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
        Simd::interleave(self, other)
    }

    fn deinterleave(self, other: Self) -> (Self, Self) {
        Simd::deinterleave(self, other)
    }
}
