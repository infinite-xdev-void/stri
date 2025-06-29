//
//
//

use rust_decimal::Decimal;

//
//
//
//
//

use crate::{Owned, STR, ToInterpolator};

//
//
//
//
//
//

impl ToInterpolator<{ STR }> for Decimal {
    type Out<'a>
        = Owned
    where
        Self: 'a;

    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        Owned::new(self.to_string())
    }
}
