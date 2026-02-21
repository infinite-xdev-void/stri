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

use crate::{
   Sql, Str, ToInterpolator, constants::QUOTE, macros::impl_to_interpolator_for_output_string,
};

//
//
//
//
//
//
//

impl<Tz> ToInterpolator<Sql> for DateTime<Tz>
where
   Tz: TimeZone,
   <Tz as TimeZone>::Offset: std::fmt::Display,
{
   type Buffer = ();
   type Out<'a>
      = String
   where
      Self: 'a;

   #[inline(always)]
   fn to_interpolator(&self, _buf: &mut Self::Buffer) -> Self::Out<'_> {
      let mut s = String::with_capacity(2 + 32);
      s.push(QUOTE);
      s.push_str(&self.to_rfc3339());
      s.push(QUOTE);

      s
   }
}

impl<Tz> ToInterpolator<Str> for DateTime<Tz>
where
   Tz: TimeZone,
   <Tz as TimeZone>::Offset: std::fmt::Display,
{
   type Buffer = ();
   type Out<'a>
      = String
   where
      Self: 'a;

   #[inline(always)]
   fn to_interpolator(&self, _buf: &mut Self::Buffer) -> Self::Out<'_> {
      self.to_string()
   }
}

//
//
//

impl_to_interpolator_for_output_string!(NaiveDateTime);
impl_to_interpolator_for_output_string!(Duration);
impl_to_interpolator_for_output_string!(FixedOffset);
impl_to_interpolator_for_output_string!(NaiveDate);
impl_to_interpolator_for_output_string!(Weekday);
impl_to_interpolator_for_output_string!(NaiveTime);
