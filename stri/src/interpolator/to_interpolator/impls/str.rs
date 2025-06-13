//
//
//

use crate::{Refr, ToInterpolator};

//
//
//
//
//
//
//

impl ToInterpolator<{ crate::STR }> for String {
    type Out<'a> = Refr<'a>;
    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        Refr::new(self)
    }
}

//
//
//
//
//
//
//

impl ToInterpolator<{ crate::STR }> for &str {
    type Out<'a>
        = Refr<'a>
    where
        Self: 'a;
    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        Refr::new(*self)
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
    crate::STR;
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
