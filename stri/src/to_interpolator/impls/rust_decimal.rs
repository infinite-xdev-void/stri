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
   type Buffer = ();
   type Out<'a> = String;

   #[inline(always)]
   fn to_interpolator(&self, _buf: &mut Self::Buffer) -> Self::Out<'_> {
      self.to_string()
   }
}

impl ToInterpolator<Str> for Decimal {
   type Buffer = ();
   type Out<'a> = String;

   #[inline(always)]
   fn to_interpolator(&self, _buf: &mut Self::Buffer) -> Self::Out<'_> {
      self.to_string()
   }
}
