//
//
//

use crate::Owned;

//
//
//
//
//
//
//

use super::Sanitizer;

//
//
//
//
//
//
//

/// wrap `&str` with single quote and sanitize the input based on `S` sanitizer.
///
/// # Panic
/// this function will panic if the length of the new `String` exceeded
/// `usize::MAX`
#[inline(always)]
pub fn to_sql_string_interpolator<S: Sanitizer>(s: &str) -> Owned {
    //
    //
    //

    let mut new_capacity = 2usize
        .checked_add(s.len())
        .ok_or("`usize` capacity exceeded")
        .unwrap();

    let extra_needed_cap = S::extra_capacity(s);

    //
    //
    //

    if extra_needed_cap == 0 {
        let mut v = Vec::with_capacity(new_capacity);
        let mut end = v.as_mut_ptr();
        unsafe {
            core::ptr::write(end, b'\'');
            end = end.add(1);

            for b in s.as_bytes() {
                core::ptr::write(end, *b);
                end = end.add(1);
            }

            core::ptr::write(end, b'\'');

            v.set_len(new_capacity);

            return Owned::new(String::from_utf8_unchecked(v));
        };
    };

    //
    //
    //

    new_capacity = new_capacity
        .checked_add(extra_needed_cap)
        .ok_or("`usize` capacity exceeded")
        .unwrap();

    let mut v = Vec::with_capacity(new_capacity);

    //
    //
    //
    //
    //
    //
    //

    let mut end = v.as_mut_ptr();

    //
    //
    //

    unsafe {
        //
        //
        //
        core::ptr::write(end, b'\'');
        end = end.add(1);

        //
        //
        //
        //
        //
        //
        //

        end = S::process(end, s);

        //
        //
        //
        //
        //
        //
        //

        core::ptr::write(end, b'\'');

        v.set_len(2 + s.len() + extra_needed_cap);

        Owned::new(String::from_utf8_unchecked(v))
    }
}
