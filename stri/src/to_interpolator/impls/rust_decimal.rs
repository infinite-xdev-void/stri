//
//
//

use rust_decimal::Decimal;

//
//
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

impl ToInterpolator<Sql> for Decimal {
   type Out<'a> = String;

   #[inline(always)]
   fn to_interpolator(&self) -> Self::Out<'_> {
      self.to_string()
   }
}

impl ToInterpolator<Str> for Decimal {
   type Out<'a> = String;

   #[inline(always)]
   fn to_interpolator(&self) -> Self::Out<'_> {
      self.to_string()
   }
}
