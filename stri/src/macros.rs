//
//
//

#[macro_export]
macro_rules! impl_to_interpolator_for_output_string {
   ($type: ty) => {
      impl ToInterpolator<Sql> for $type {
         type Out<'a>
            = String
         where
            Self: 'a;

         #[inline]
         fn to_interpolator<'a>(&'a self) -> Self::Out<'a> {
            let s = self.to_string();
            let mut out = String::with_capacity(2 + s.len());
            out.push(QUOTE);
            out.push_str(&s);
            out.push(QUOTE);
            out
         }
      }

      impl ToInterpolator<Str> for $type {
         type Out<'a>
            = String
         where
            Self: 'a;

         #[inline(always)]
         fn to_interpolator<'a>(&'a self) -> Self::Out<'a> {
            self.to_string()
         }
      }
   };
}

pub use impl_to_interpolator_for_output_string;
