// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}
impl SaturatingU16 {
    fn new(value: u16) -> Self {
        Self { value }
    }
}

impl From<u16> for SaturatingU16 {
    // add code here
    fn from(value: u16) -> Self {
        // Self::new(value.into())
        Self {
            value: value.into(),
        }
    }
}
impl From<u8> for SaturatingU16 {
    // add code here
    fn from(value: u8) -> Self {
        // Self::new(value.into())
        Self {
            value: value.into(),
        }
    }
}
impl From<&u8> for SaturatingU16 {
    // add code here
    fn from(value: &u8) -> Self {
        Self {
            value: (*value).into(),
        }
    }
}
impl From<&u16> for SaturatingU16 {
    // add code here
    fn from(value: &u16) -> Self {
        Self::new((*value).into())
    }
}

impl Add for SaturatingU16 {
    // add code here
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value
        // Self {
        //     value: self.value + rhs.value,
        // }
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    // add code here
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        self + *rhs
    }
}
impl Add<u16> for SaturatingU16 {
    // add code here
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl PartialEq for SaturatingU16 {
    // add code here

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialEq<u16> for SaturatingU16 {
    // add code here
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
