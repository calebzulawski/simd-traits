pub mod float;
pub mod int;

use core::ops;

pub trait Num:
    crate::vector::Array
    + Default
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + ops::AddAssign
    + ops::SubAssign
    + ops::MulAssign
    + ops::DivAssign
    + ops::RemAssign
{
    fn zero() -> Self {
        Default::default()
    }

    fn one() -> Self;
}

pub trait Signed: Num + ops::Neg<Output = Self> {}

pub trait Unsigned: Num {}
