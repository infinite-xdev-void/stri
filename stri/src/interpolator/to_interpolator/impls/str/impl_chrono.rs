//
//
//

use chrono::{
    DateTime, Duration, FixedOffset, Month, NaiveDate, NaiveDateTime, NaiveTime, OutOfRange,
    TimeZone, Utc, Weekday, WeekdaySet,
};

//
//
//
//
//
//
//

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

//
//
//
//
//
//
//

crate::impl_to_interpolator!(STR; NaiveDate, NaiveDateTime, NaiveTime, Duration, FixedOffset, OutOfRange, Utc, Weekday,WeekdaySet);
