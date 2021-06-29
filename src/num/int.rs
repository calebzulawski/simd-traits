use core::ops;

pub trait Int:
    super::Num
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::BitXor<Output = Self>
    + ops::Shl<Output = Self>
    + ops::Shr<Output = Self>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign
    + ops::ShrAssign
{
}
