//
//
//

use crate::{Sql, Str, ToInterpolator};

//
//
//
//
//
//
//

macro_rules! impl_int {
   ($type: ty) => {
      impl ToInterpolator<Sql> for $type {
         type Out<'a> = String;

         #[inline(always)]
         fn to_interpolator(&self) -> Self::Out<'_> {
            ::itoa::Buffer::new().format(*self).to_string()
         }
      }

      impl ToInterpolator<Str> for $type {
         type Out<'a> = String;

         #[inline(always)]
         fn to_interpolator(&self) -> Self::Out<'_> {
            ::itoa::Buffer::new().format(*self).to_string()
         }
      }
   };
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u128);
impl_int!(usize);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);

//
//
//

macro_rules! impl_float {
   ($type: ty) => {
      impl ToInterpolator<Sql> for $type {
         type Out<'a> = String;

         #[inline(always)]
         fn to_interpolator(&self) -> Self::Out<'_> {
            zmij::Buffer::new().format(*self).to_string()
         }
      }

      impl ToInterpolator<Str> for $type {
         type Out<'a> = String;

         #[inline(always)]
         fn to_interpolator(&self) -> Self::Out<'_> {
            zmij::Buffer::new().format(*self).to_string()
         }
      }
   };
}

impl_float!(f32);
impl_float!(f64);

//
//
//

impl ToInterpolator<Sql> for bool {
   type Out<'a> = &'a str;

   #[inline]
   fn to_interpolator(&self) -> Self::Out<'_> {
      match self {
         true => "TRUE",
         false => "FALSE",
      }
   }
}

impl ToInterpolator<Str> for bool {
   type Out<'a> = &'a str;

   #[inline]
   fn to_interpolator(&self) -> Self::Out<'_> {
      match self {
         true => "true",
         false => "false",
      }
   }
}

//
//
//

impl ToInterpolator<Sql> for char {
   type Out<'a> = String;

   #[inline]
   fn to_interpolator(&self) -> Self::Out<'_> {
      if self.eq(&'\'') {
         String::from("''''")
      } else {
         let len = 2 + self.len_utf8();
         let mut bytes = vec![b'\''; len];
         self.encode_utf8(&mut bytes[1..len - 1]);
         unsafe { String::from_utf8_unchecked(bytes) }
      }
   }
}

impl ToInterpolator<Str> for char {
   type Out<'a> = String;

   #[inline(always)]
   fn to_interpolator(&self) -> Self::Out<'_> {
      self.to_string()
   }
}
