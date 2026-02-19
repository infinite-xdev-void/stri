//
//
//

use crate::constants::QUOTE_BYTE;

//
//
//
//
//
//
//

#[inline(always)]
pub fn count_quotes(s: &str) -> usize {
   let mut count: usize = 0;
   for byte in s.as_bytes() {
      if byte.eq(&QUOTE_BYTE) {
         count += 1;
      }
   }
   count
}

// //
// //
// //

// use std::{
//    hint::unreachable_unchecked,
//    mem::{MaybeUninit, transmute},
// };

// //
// //
// //
// //
// //
// //
// //

// macro_rules! define_unsigned_counter_fn {
//    ($name: ident, $type:ty, $digits_counter_len: expr, $digits_counter: expr $(,)?) => {
//       #[inline]
//       pub fn $name(num: $type) -> usize {
//          const DIGITS_COUNTER: [$type; $digits_counter_len] = $digits_counter;
//          (match DIGITS_COUNTER.binary_search(&num) {
//             Ok(i) => i, // exact power-> digits = index+1
//             Err(i) => i,
//          }) + 1
//       }
//    };
// }

// #[inline]
// pub fn count_u8_digits(num: u8) -> usize {
//    match (num > 99, num > 9) {
//       (true, _) => 3,
//       (false, true) => 2,
//       (false, false) => 1,
//    }
// }

// define_unsigned_counter_fn!(count_u16_digits, u16, 4, [9, 99, 999, 9_999]);

// define_unsigned_counter_fn!(
//    count_u32_digits,
//    u32,
//    9,
//    [
//       9,
//       99,
//       999,
//       9_999,
//       99_999,
//       999_999,
//       9_999_999,
//       99_999_999,
//       999_999_999,
//    ],
// );

// define_unsigned_counter_fn!(
//    count_u64_digits,
//    u64,
//    19,
//    [
//       9,
//       99,
//       999,
//       9_999,
//       99_999,
//       999_999,
//       9_999_999,
//       99_999_999,
//       999_999_999,
//       9_999_999_999,
//       99_999_999_999,
//       999_999_999_999,
//       9_999_999_999_999,
//       99_999_999_999_999,
//       999_999_999_999_999,
//       9_999_999_999_999_999,
//       99_999_999_999_999_999,
//       999_999_999_999_999_999,
//       9_999_999_999_999_999_999
//    ],
// );

// define_unsigned_counter_fn!(
//    count_u128_digits,
//    u128,
//    38,
//    [
//       9,
//       99,
//       999,
//       9_999,
//       99_999,
//       999_999,
//       9_999_999,
//       99_999_999,
//       999_999_999,
//       9_999_999_999,
//       99_999_999_999,
//       999_999_999_999,
//       9_999_999_999_999,
//       99_999_999_999_999,
//       999_999_999_999_999,
//       9_999_999_999_999_999,
//       99_999_999_999_999_999,
//       999_999_999_999_999_999,
//       9_999_999_999_999_999_999,
//       99_999_999_999_999_999_999,
//       999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999_999_999_999,
//    ],
// );

// #[cfg(target_pointer_width = "16")]
// #[inline(always)]
// pub fn count_usize_digits(num: usize) -> usize {
//    count_u16_digits(num as u16)
// }

// #[cfg(target_pointer_width = "32")]
// #[inline(always)]
// pub fn count_usize_digits(num: usize) -> usize {
//    count_u32_digits(num as u32)
// }

// #[cfg(target_pointer_width = "64")]
// #[inline(always)]
// pub fn count_usize_digits(num: usize) -> usize {
//    count_u64_digits(num as u64)
// }

// //
// //
// //

// macro_rules! define_unsigned_convert_fn {
//    ($name: ident, $type: ty, $counter: ident) => {
//       #[inline]
//       pub fn $name(mut num: $type) -> String {
//          let mut i = $counter(num);

//          match i {
//             // the value of `i` can in no way be equal to zero.
//             0 => unsafe { unreachable_unchecked() },

//             1 => unsafe { String::from_utf8_unchecked(vec![num as u8 + 48]) },

//             _ => {
//                let mut vec = vec![MaybeUninit::<u8>::uninit(); i];
//                // to use as index
//                i -= 1;
//                vec[i].write((num % 10) as u8 + 48);
//                num /= 10;

//                while i > 0 {
//                   i -= 1;
//                   vec[i].write((num % 10) as u8 + 48);
//                   num /= 10;
//                }

//                unsafe { String::from_utf8_unchecked(transmute::<_, Vec<u8>>(vec)) }
//             }
//          }
//       }
//    };
// }

// define_unsigned_convert_fn!(u8_to_string, u8, count_u8_digits);
// define_unsigned_convert_fn!(u16_to_string, u16, count_u16_digits);
// define_unsigned_convert_fn!(u32_to_string, u32, count_u32_digits);
// define_unsigned_convert_fn!(u64_to_string, u64, count_u64_digits);
// define_unsigned_convert_fn!(u128_to_string, u128, count_u128_digits);
// define_unsigned_convert_fn!(usize_to_string, usize, count_usize_digits);

// //
// //
// //
// //
// //
// //
// //

// macro_rules! define_signed_counter_fn {
//    ($name: ident, $type:ty, $digits_counter_len: expr, $digits_counter: expr $(,)?) => {
//       #[inline]
//       /// this function count the `sign` if the number is negative
//       pub fn $name(num: $type) -> usize {
//          const DIGITS_COUNTER: [$type; $digits_counter_len] = $digits_counter;
//          let i = match DIGITS_COUNTER.binary_search(&num) {
//             Ok(i) => i,
//             Err(i) => i,
//          };

//          if i > DIGITS_COUNTER.len() / 2 {
//             i - DIGITS_COUNTER.len() / 2
//          } else {
//             usize::MAX
//                .wrapping_sub(i.wrapping_sub(DIGITS_COUNTER.len() / 2 + 1))
//                .wrapping_add(2)
//          }
//       }
//    };
// }

// define_signed_counter_fn!(count_i8_digits, i8, 5, [-100, -10, -1, 9, 99]);
// define_signed_counter_fn!(
//    count_i16_digits,
//    i16,
//    9,
//    [-10_000, -1_000, -100, -10, -1, 9, 99, 999, 9_999]
// );

// define_signed_counter_fn!(
//    count_i32_digits,
//    i32,
//    19,
//    [
//       -1_000_000_000,
//       -100_000_000,
//       -10_000_000,
//       -1_000_000,
//       -100_000,
//       -10_000,
//       -1_000,
//       -100,
//       -10,
//       -1,
//       9,
//       99,
//       999,
//       9_999,
//       99_999,
//       999_999,
//       9_999_999,
//       99_999_999,
//       999_999_999,
//    ]
// );

// define_signed_counter_fn!(
//    count_i64_digits,
//    i64,
//    37,
//    [
//       -1_000_000_000_000_000_000,
//       -100_000_000_000_000_000,
//       -10_000_000_000_000_000,
//       -1_000_000_000_000_000,
//       -100_000_000_000_000,
//       -10_000_000_000_000,
//       -1_000_000_000_000,
//       -100_000_000_000,
//       -10_000_000_000,
//       -1_000_000_000,
//       -100_000_000,
//       -10_000_000,
//       -1_000_000,
//       -100_000,
//       -10_000,
//       -1_000,
//       -100,
//       -10,
//       -1,
//       9,
//       99,
//       999,
//       9_999,
//       99_999,
//       999_999,
//       9_999_999,
//       99_999_999,
//       999_999_999,
//       9_999_999_999,
//       99_999_999_999,
//       999_999_999_999,
//       9_999_999_999_999,
//       99_999_999_999_999,
//       999_999_999_999_999,
//       9_999_999_999_999_999,
//       99_999_999_999_999_999,
//       999_999_999_999_999_999,
//    ]
// );

// define_signed_counter_fn!(
//    count_i128_digits,
//    i128,
//    77,
//    [
//       -100_000_000_000_000_000_000_000_000_000_000_000_000,
//       -10_000_000_000_000_000_000_000_000_000_000_000_000,
//       -1_000_000_000_000_000_000_000_000_000_000_000_000,
//       -100_000_000_000_000_000_000_000_000_000_000_000,
//       -10_000_000_000_000_000_000_000_000_000_000_000,
//       -1_000_000_000_000_000_000_000_000_000_000_000,
//       -100_000_000_000_000_000_000_000_000_000_000,
//       -10_000_000_000_000_000_000_000_000_000_000,
//       -1_000_000_000_000_000_000_000_000_000_000,
//       -100_000_000_000_000_000_000_000_000_000,
//       -10_000_000_000_000_000_000_000_000_000,
//       -1_000_000_000_000_000_000_000_000_000,
//       -100_000_000_000_000_000_000_000_000,
//       -10_000_000_000_000_000_000_000_000,
//       -1_000_000_000_000_000_000_000_000,
//       -100_000_000_000_000_000_000_000,
//       -10_000_000_000_000_000_000_000,
//       -1_000_000_000_000_000_000_000,
//       -100_000_000_000_000_000_000,
//       -10_000_000_000_000_000_000,
//       -1_000_000_000_000_000_000,
//       -100_000_000_000_000_000,
//       -10_000_000_000_000_000,
//       -1_000_000_000_000_000,
//       -100_000_000_000_000,
//       -10_000_000_000_000,
//       -1_000_000_000_000,
//       -100_000_000_000,
//       -10_000_000_000,
//       -1_000_000_000,
//       -100_000_000,
//       -10_000_000,
//       -1_000_000,
//       -100_000,
//       -10_000,
//       -1_000,
//       -100,
//       -10,
//       -1,
//       9,
//       99,
//       999,
//       9_999,
//       99_999,
//       999_999,
//       9_999_999,
//       99_999_999,
//       999_999_999,
//       9_999_999_999,
//       99_999_999_999,
//       999_999_999_999,
//       9_999_999_999_999,
//       99_999_999_999_999,
//       999_999_999_999_999,
//       9_999_999_999_999_999,
//       99_999_999_999_999_999,
//       999_999_999_999_999_999,
//       9_999_999_999_999_999_999,
//       99_999_999_999_999_999_999,
//       999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999_999_999,
//       999_999_999_999_999_999_999_999_999_999_999_999,
//       9_999_999_999_999_999_999_999_999_999_999_999_999,
//       99_999_999_999_999_999_999_999_999_999_999_999_999,
//    ]
// );

// #[cfg(target_pointer_width = "16")]
// #[inline(always)]
// pub fn count_isize_digits(num: isize) -> usize {
//    count_i16_digits(num as i16)
// }

// #[cfg(target_pointer_width = "32")]
// #[inline(always)]
// pub fn count_isize_digits(num: isize) -> usize {
//    count_i32_digits(num as i32)
// }

// #[cfg(target_pointer_width = "64")]
// #[inline(always)]
// pub fn count_isize_digits(num: isize) -> usize {
//    count_i64_digits(num as i64)
// }

// //
// //
// //

// macro_rules! define_signed_convert_fn {
//    ($name: ident, $type: ty, $counter: ident) => {
//       #[inline]
//       pub fn $name(mut num: $type) -> String {
//          let mut i = $counter(num);
//          match i {
//             // the value of `i` can in no way be equal to zero
//             0 => unsafe { unreachable_unchecked() },

//             // if the count is 1 then the number is positive (negative number count is at least 2)
//             1 => unsafe { String::from_utf8_unchecked(vec![(num as u8 + 48)]) },

//             _ => {
//                let mut vec = vec![MaybeUninit::<u8>::uninit(); i];

//                match num.is_negative() {
//                   true => {
//                      vec[0].write(b'-');
//                      let mut num = num.unsigned_abs();

//                      i -= 1;
//                      vec[i].write((num % 10) as u8 + 48);
//                      num /= 10;
//                      while i > 1 {
//                         i -= 1;
//                         vec[i].write((num % 10) as u8 + 48);
//                         num /= 10;
//                      }
//                   }

//                   false => {
//                      i -= 1;
//                      vec[i].write((num % 10) as u8 + 48);
//                      num /= 10;
//                      while i > 0 {
//                         i -= 1;
//                         vec[i].write((num % 10) as u8 + 48);
//                         num /= 10;
//                      }
//                   }
//                }

//                unsafe { String::from_utf8_unchecked(transmute::<_, Vec<u8>>(vec)) }
//             }
//          }
//       }
//    };
// }

// define_signed_convert_fn!(i8_to_string, i8, count_i8_digits);
// define_signed_convert_fn!(i16_to_string, i16, count_i16_digits);
// define_signed_convert_fn!(i32_to_string, i32, count_i32_digits);
// define_signed_convert_fn!(i64_to_string, i64, count_i64_digits);
// define_signed_convert_fn!(i128_to_string, i128, count_i128_digits);
// define_signed_convert_fn!(isize_to_string, isize, count_isize_digits);

// //
// //
// //
// //
// //
// //
// //
// //
// //
// //
// //
// //
// //

// /// it might be possible to test all number of `u8`, `u16`, `i8` and `i16` but for larger types it is not, so instead of that a test for critical points only made (for example [[-11, -10, -9], [-1, 0, 1], [9, 10, 11]], [[99, 100, 101]], ...etc) those points determine if a number has more digits or not
// #[cfg(test)]
// mod tests {
//    use super::*;

//    macro_rules! generate_test_for_unsigned_counter {
//       ($test_name: ident, $type: ident, $test_target: ident) => {
//          #[test]
//          fn $test_name() {
//             assert_eq!($test_target(0), 1);
//             assert_eq!($test_target(1), 1);
//             assert_eq!($test_target($type::MAX), $type::MAX.to_string().len());

//             let mut num: $type = 10;
//             let mut overflowed = false;

//             while !overflowed {
//                assert_eq!($test_target(num - 1), (num - 1).to_string().len());
//                assert_eq!($test_target(num), num.to_string().len());
//                assert_eq!($test_target(num + 1), (num + 1).to_string().len());

//                (num, overflowed) = num.overflowing_mul(10);
//             }
//          }
//       };
//    }

//    generate_test_for_unsigned_counter!(test_count_u8_digits, u8, count_u8_digits);
//    generate_test_for_unsigned_counter!(test_count_u16_digits, u16, count_u16_digits);
//    generate_test_for_unsigned_counter!(test_count_u32_digits, u32, count_u32_digits);
//    generate_test_for_unsigned_counter!(test_count_u64_digits, u64, count_u64_digits);
//    generate_test_for_unsigned_counter!(test_count_u128_digits, u128, count_u128_digits);
//    generate_test_for_unsigned_counter!(test_count_usize_digits, usize, count_usize_digits);

//    //
//    //
//    //

//    macro_rules! generate_test_for_unsigned_string_converter {
//       ($test_name: ident, $type: ident, $test_target: ident) => {
//          #[test]
//          fn $test_name() {
//             assert_eq!($test_target(0), "0");
//             assert_eq!($test_target(1), "1");
//             assert_eq!($test_target($type::MAX), $type::MAX.to_string());

//             let mut num: $type = 10;
//             let mut overflowed = false;

//             while !overflowed {
//                assert_eq!($test_target(num - 1), (num - 1).to_string());
//                assert_eq!($test_target(num), num.to_string());

//                (num, overflowed) = num.overflowing_mul(10);
//             }
//          }
//       };
//    }

//    generate_test_for_unsigned_string_converter!(test_u8_to_string, u8, u8_to_string);
//    generate_test_for_unsigned_string_converter!(test_u16_to_string, u16, u16_to_string);
//    generate_test_for_unsigned_string_converter!(test_u32_to_string, u32, u32_to_string);
//    generate_test_for_unsigned_string_converter!(test_u64_to_string, u64, u64_to_string);
//    generate_test_for_unsigned_string_converter!(test_u128_to_string, u128, u128_to_string);
//    generate_test_for_unsigned_string_converter!(test_usize_to_string, usize, usize_to_string);

//    //
//    //
//    //
//    //
//    //
//    //
//    //

//    macro_rules! generate_test_for_signed_counter {
//       ($test_name: ident, $type: ident, $test_target: ident) => {
//          #[test]
//          fn $test_name() {
//             assert_eq!($test_target($type::MAX), $type::MAX.to_string().len());
//             assert_eq!($test_target(-1), 2);
//             assert_eq!($test_target(0), 1);
//             assert_eq!($test_target(1), 1);
//             assert_eq!($test_target($type::MIN), $type::MIN.to_string().len());

//             let mut pos: $type = 10;
//             let mut neg: $type = -10;
//             let mut overflowed = false;

//             while !overflowed {
//                assert_eq!($test_target(pos - 1), (pos - 1).to_string().len());
//                assert_eq!($test_target(pos), (pos).to_string().len());
//                assert_eq!($test_target(pos + 1), (pos + 1).to_string().len());

//                assert_eq!($test_target(neg + 1), (neg + 1).to_string().len());
//                assert_eq!($test_target(neg), neg.to_string().len());
//                assert_eq!($test_target(neg - 1), (neg - 1).to_string().len());

//                (pos, overflowed) = pos.overflowing_mul(10);
//                neg = neg.wrapping_mul(10);
//             }
//          }
//       };
//    }

//    generate_test_for_signed_counter!(test_count_i8_digits, i8, count_i8_digits);
//    generate_test_for_signed_counter!(test_count_i16_digits, i16, count_i16_digits);
//    generate_test_for_signed_counter!(test_count_i32_digits, i32, count_i32_digits);
//    generate_test_for_signed_counter!(test_count_i64_digits, i64, count_i64_digits);
//    generate_test_for_signed_counter!(test_count_i128_digits, i128, count_i128_digits);
//    generate_test_for_signed_counter!(test_count_isize_digits, isize, count_isize_digits);

//    //
//    //
//    //

//    macro_rules! generate_test_for_signed_string_converter {
//       ($test_name: ident, $type: ident, $test_target: ident) => {
//          #[test]
//          fn $test_name() {
//             assert_eq!($test_target($type::MIN), $type::MIN.to_string());
//             assert_eq!($test_target(-1), "-1");
//             assert_eq!($test_target(0), "0");
//             assert_eq!($test_target(1), "1");
//             assert_eq!($test_target($type::MAX), $type::MAX.to_string());

//             let mut pos: $type = 10;
//             let mut neg: $type = -10;
//             let mut overflowed = false;

//             while !overflowed {
//                assert_eq!($test_target(pos - 1), (pos - 1).to_string());
//                assert_eq!($test_target(pos), (pos).to_string());
//                assert_eq!($test_target(pos + 1), (pos + 1).to_string());

//                assert_eq!($test_target(neg + 1), (neg + 1).to_string());
//                assert_eq!($test_target(neg), neg.to_string());
//                assert_eq!($test_target(neg - 1), (neg - 1).to_string());

//                (pos, overflowed) = pos.overflowing_mul(10);
//                neg = neg.wrapping_mul(10);
//             }
//          }
//       };
//    }

//    generate_test_for_signed_string_converter!(test_i8_to_string, i8, i8_to_string);
//    generate_test_for_signed_string_converter!(test_i16_to_string, i16, i16_to_string);
//    generate_test_for_signed_string_converter!(test_i32_to_string, i32, i32_to_string);
//    generate_test_for_signed_string_converter!(test_i64_to_string, i64, i64_to_string);
//    generate_test_for_signed_string_converter!(test_i128_to_string, i128, i128_to_string);
//    generate_test_for_signed_string_converter!(test_isize_to_string, isize, isize_to_string);
// }
