pub use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
pub use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};


pub trait MathAble: Add + Div + Mul + Neg + Rem + Sub + AddAssign + DivAssign + MulAssign + RemAssign + SubAssign + Sized {}
impl<T: Add + Div + Mul + Neg + Rem + Sub + AddAssign + DivAssign + MulAssign + RemAssign + SubAssign + Sized> MathAble for T {}