//
//
//

use crate::{Sql, Str, ToInterpolator, utils::count_quotes};

//
//
//
//
//
//
//

const QUOTE_BYTE: u8 = b'\'';

//
//
//
//
//
//
//

macro_rules! impl_for_string {
   ($type: ty) => {
      impl ToInterpolator<Sql> for $type {
         type Out<'a>
            = String
         where
            Self: 'a;

         #[inline(always)]
         fn to_interpolator(&self) -> Self::Out<'_> {
            let mut vec = Vec::with_capacity(2 + self.len() + count_quotes(self));
            vec.push(QUOTE_BYTE);

            for &byte in self.as_bytes() {
               if byte == QUOTE_BYTE {
                  vec.push(QUOTE_BYTE);
                  vec.push(QUOTE_BYTE);
               } else {
                  vec.push(byte);
               }
            }

            vec.push(QUOTE_BYTE);

            unsafe { String::from_utf8_unchecked(vec) }
         }
      }

      impl ToInterpolator<Str> for $type {
         type Out<'a>
            = &'a str
         where
            Self: 'a;

         #[inline(always)]
         fn to_interpolator<'a>(&'a self) -> Self::Out<'a> {
            self
         }
      }
   };
}

impl_for_string!(String);
impl_for_string!(&str);
