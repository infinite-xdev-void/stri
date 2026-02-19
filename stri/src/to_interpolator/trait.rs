//
//
//

use crate::{Format, Interpolate};

//
//
//
//
//
//
//

pub trait ToInterpolator<F: Format> {
   type Out<'a>: Interpolate
   where
      Self: 'a;

   fn to_interpolator(&self) -> Self::Out<'_>;
}

//
//
//
//
//
//
//

#[doc(hidden)]
pub mod __private {
   use super::ToInterpolator;
   use crate::{Sql, Str};

   //
   //
   //

   pub trait ToInterpolatorForSql {
      type SqlOut<'a>
      where
         Self: 'a;

      fn __to_interpolator_for_sql(&self) -> Self::SqlOut<'_>;
   }

   impl<T> ToInterpolatorForSql for T
   where
      T: ToInterpolator<Sql>,
   {
      type SqlOut<'a>
         = T::Out<'a>
      where
         Self: 'a;

      #[doc(hidden)]
      #[inline(always)]
      fn __to_interpolator_for_sql<'a>(&'a self) -> Self::SqlOut<'a> {
         <Self as ToInterpolator<Sql>>::to_interpolator(self)
      }
   }

   //

   pub trait ToInterpolatorForStr {
      type StrOut<'a>
      where
         Self: 'a;

      fn __to_interpolator_for_str(&self) -> Self::StrOut<'_>;
   }

   impl<T> ToInterpolatorForStr for T
   where
      T: ToInterpolator<Str>,
   {
      type StrOut<'a>
         = T::Out<'a>
      where
         Self: 'a;

      #[doc(hidden)]
      #[inline(always)]
      fn __to_interpolator_for_str<'a>(&'a self) -> Self::StrOut<'a> {
         <Self as ToInterpolator<Str>>::to_interpolator(self)
      }
   }
}
