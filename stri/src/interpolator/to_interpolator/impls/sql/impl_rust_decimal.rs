//
//
//

use rust_decimal::Decimal;

//
//
//
//
//

use crate::{Owned, SQL, ToInterpolator};

//
//
//
//
//
//

impl ToInterpolator<{ SQL }> for Decimal {
    type Out<'a>
        = Owned
    where
        Self: 'a;

    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        Owned::new(self.to_string())
    }
}
