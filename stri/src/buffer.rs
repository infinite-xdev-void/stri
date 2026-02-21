//
//
//

#[doc(hidden)]
pub trait Buffer {
   #[doc(hidden)]
   type Buffer;
   #[doc(hidden)]
   fn __stri_new_buffer(&self) -> Self::Buffer;
}

//
//
//

macro_rules! impl_for_int {
   ($type: ty) => {
      #[doc(hidden)]
      impl Buffer for $type {
         #[doc(hidden)]
         type Buffer = ::itoa::Buffer;

         #[inline(always)]
         #[doc(hidden)]
         fn __stri_new_buffer(&self) -> Self::Buffer {
            ::itoa::Buffer::new()
         }
      }
   };
}

impl_for_int!(u8);
impl_for_int!(u16);
impl_for_int!(u32);
impl_for_int!(u64);
impl_for_int!(u128);
impl_for_int!(usize);

impl_for_int!(i8);
impl_for_int!(i16);
impl_for_int!(i32);
impl_for_int!(i64);
impl_for_int!(i128);
impl_for_int!(isize);

//
//
//

macro_rules! impl_for_float {
   ($type: ty) => {
      #[doc(hidden)]
      impl Buffer for $type {
         #[doc(hidden)]
         type Buffer = ::zmij::Buffer;

         #[inline(always)]
         #[doc(hidden)]
         fn __stri_new_buffer(&self) -> Self::Buffer {
            ::zmij::Buffer::new()
         }
      }
   };
}

impl_for_float!(f32);
impl_for_float!(f64);

//
//
//

macro_rules! impl_for_any {
   ($type: ty) => {
      #[doc(hidden)]
      impl Buffer for $type {
         #[doc(hidden)]
         type Buffer = ();

         #[inline(always)]
         #[doc(hidden)]
         fn __stri_new_buffer(&self) -> Self::Buffer {
            ()
         }
      }
   };
}

impl_for_any!(bool);
impl_for_any!(char);
impl_for_any!(&str);
impl_for_any!(String);

#[cfg(feature = "rust_decimal")]
impl_for_any!(::rust_decimal::Decimal);

#[cfg(feature = "chrono")]
mod chrono {
   use super::*;
   use ::chrono::{
      DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday,
   };

   #[doc(hidden)]
   impl<Tz: TimeZone> Buffer for DateTime<Tz> {
      #[doc(hidden)]
      type Buffer = ();
      #[inline(always)]
      #[doc(hidden)]
      fn __stri_new_buffer(&self) -> Self::Buffer {
         ()
      }
   }

   impl_for_any!(Duration);
   impl_for_any!(FixedOffset);
   impl_for_any!(NaiveDate);
   impl_for_any!(NaiveDateTime);
   impl_for_any!(NaiveTime);
   impl_for_any!(Weekday);
}
