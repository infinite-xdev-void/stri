//
//
//

use crate::{Owned, ToInterpolator};

//
//
//
//
//
//
//

mod sanitizer;
use sanitizer::Sanitizer;

//
//
//

mod auxs;
pub use auxs::to_sql_string_interpolator;

//
//
//

pub mod sanitizers;

//
//
//
//
//
//
//

impl ToInterpolator<{ crate::SQL }> for String {
    type Out<'a> = Owned;
    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        auxs::to_sql_string_interpolator::<sanitizers::SingleQuote>(self)
    }
}

//
//
//
//
//
//
//

impl ToInterpolator<{ crate::SQL }> for &str {
    type Out<'a>
        = Owned
    where
        Self: 'a;
    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        auxs::to_sql_string_interpolator::<sanitizers::SingleQuote>(self)
    }
}

//
//
//
//
//
//
//

crate::impl_to_interpolator!(
    crate::SQL;
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    bool,
    f32,
    f64
);

//
//
//
//
//
//
//

#[cfg(feature = "chrono")]
mod impl_chrono;

//
//
//
//
//
//
//

#[cfg(feature = "rust_decimal")]
mod impl_rust_decimal;
