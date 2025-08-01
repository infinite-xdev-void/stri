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

use crate::{Owned, ToInterpolator};

//
//
//
//
//
//
//

impl<Tz> ToInterpolator<{ crate::SQL }> for DateTime<Tz>
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
        let mut s = String::with_capacity(34);
        s.push('\'');
        s.push_str(&self.to_rfc3339());
        s.push('\'');
        Owned::new(s)
    }
}

//
//
//
//
//
//
//

crate::impl_to_interpolator!(SQL; NaiveDate, NaiveDateTime, NaiveTime, Duration, FixedOffset, Weekday);

//
//
//
//
//
//
//

/* use chrono::{Month,
OutOfRange,
Utc,
WeekdaySet,} */

//
//
//
//
//
//
//

/* impl ToInterpolator<{ crate::SQL }> for Month {
    type Out<'a>
        = Owned
    where
        Self: 'a;

    #[inline(always)]
    fn to_interpolator(&self) -> Self::Out<'_> {
        let month = self.name();
        let mut s = String::with_capacity(month.len() + 2);
        s.push('\'');
        s.push_str(month);
        s.push('\'');
        Owned::new(s)
    }
} */

//
//
//

/* crate::impl_to_interpolator!(SQL; OutOfRange, Utc, WeekdaySet); */
