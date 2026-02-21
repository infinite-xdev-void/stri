//
//
//

use crate::{Sql, Str, ToInterpolator};

//
//
//

pub trait ToInterpolatorForSql {
   type Buffer;
   type SqlOut<'a>
   where
      Self: 'a;

   fn __stri_to_interpolator_for_sql<'a>(&'a self, buf: &'a mut Self::Buffer) -> Self::SqlOut<'a>;
}

impl<T> ToInterpolatorForSql for T
where
   T: ToInterpolator<Sql>,
{
   type Buffer = T::Buffer;
   type SqlOut<'a>
      = T::Out<'a>
   where
      Self: 'a;

   #[doc(hidden)]
   #[inline(always)]
   fn __stri_to_interpolator_for_sql<'a>(&'a self, buf: &'a mut Self::Buffer) -> Self::SqlOut<'a> {
      <Self as ToInterpolator<Sql>>::to_interpolator(self, buf)
   }
}

//

pub trait ToInterpolatorForStr {
   type Buffer;
   type StrOut<'a>
   where
      Self: 'a;

   fn __stri_to_interpolator_for_str<'a>(&'a self, buf: &'a mut Self::Buffer) -> Self::StrOut<'a>;
}

impl<T> ToInterpolatorForStr for T
where
   T: ToInterpolator<Str>,
{
   type Buffer = T::Buffer;
   type StrOut<'a>
      = T::Out<'a>
   where
      Self: 'a;

   #[doc(hidden)]
   #[inline(always)]
   fn __stri_to_interpolator_for_str<'a>(&'a self, buf: &'a mut Self::Buffer) -> Self::StrOut<'a> {
      <Self as ToInterpolator<Str>>::to_interpolator(self, buf)
   }
}
