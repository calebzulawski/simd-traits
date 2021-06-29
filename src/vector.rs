pub trait Vector:
    Copy + Clone + core::fmt::Debug + PartialEq + PartialOrd + Eq + Ord + core::hash::Hash
{
    type Scalar;
    const LANES: usize;

    fn splat(value: Self::Scalar) -> Self;

    unsafe fn get_unchecked(&self, lane: usize) -> Self::Scalar;
    unsafe fn set_unchecked(&mut self, lane: usize, value: Self::Scalar);

    fn get(&self, lane: usize) -> Self::Scalar {
        assert!(lane < Self::LANES);
        unsafe { self.get_unchecked(lane) }
    }

    fn set(&mut self, lane: usize, value: Self::Scalar) {
        assert!(lane < Self::LANES);
        unsafe { self.set_unchecked(lane, value) }
    }
}

pub trait Array: Vector + AsRef<[Self::Scalar]> + AsMut<[Self::Scalar]> {}
