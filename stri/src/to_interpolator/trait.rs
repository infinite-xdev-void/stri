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
   type Buffer;
   type Out<'a>: Interpolate
   where
      Self: 'a;

   fn to_interpolator<'a>(&'a self, buf: &'a mut Self::Buffer) -> Self::Out<'a>;
}
