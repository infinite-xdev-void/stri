//
//
//

use chrono::{
    DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday,
};

//
//
//
//
//
//
//

#[allow(unused_imports)]
use crate::{Owned, Refr, STR, ToInterpolator};

//
//
//
//
//
//
//

impl<Tz> ToInterpolator<{ STR }> for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: std::fmt::Display,
{
    type Out<'a>
        = Owned
    where
        Self: 'a;
    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        Owned::new(self.to_rfc3339())
    }
}

//
//
//
//
//
//
//

crate::impl_to_interpolator!(STR; NaiveDate, NaiveDateTime, NaiveTime, Duration, FixedOffset, Weekday);

//
//
//
//
//
//
//

/*
use chrono::{
    //
    Month,
    OutOfRange,
    Utc,
    WeekdaySet,
}; */

/*
impl ToInterpolator<{ STR }> for Month {
    type Out<'a>
        = Refr<'static>
    where
        Self: 'a;
    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        Refr::new(self.name())
    }
}

crate::impl_to_interpolator!(STR; OutOfRange, Utc,WeekdaySet); */
