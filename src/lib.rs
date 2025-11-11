#![no_std]
use core::any::Any;

pub use dyn_partial_eq_derive::*;

pub trait DynPartialEq {
  fn box_eq(&self, other: &dyn Any) -> bool;
  fn as_any(&self) -> &dyn Any;
}

// 为基本类型实现 DynPartialEq
macro_rules! impl_dyn_partial_eq_for_primitive {
    ($($t:ty),*) => {
        $(
            impl DynPartialEq for $t {
                fn as_any(&self) -> &dyn Any {
                    self
                }
                fn box_eq(&self, other: &dyn Any) -> bool {
                    other.downcast_ref::<Self>().map_or(false, |a| self == a)
                }
            }
        )*
    };
}

// 为所有基本类型实现 DynPartialEq
impl_dyn_partial_eq_for_primitive!(
  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char
);

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
impl DynPartialEq for std::string::String {
  fn as_any(&self) -> &dyn Any {
    self
  }
  fn box_eq(&self, other: &dyn Any) -> bool {
    other.downcast_ref::<Self>().map_or(false, |a| self == a)
  }
}
