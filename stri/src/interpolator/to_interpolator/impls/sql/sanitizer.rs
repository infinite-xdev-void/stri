/// define a sanitizer to work with

pub unsafe trait Sanitizer {
    /// count the extra capacity needed for sanitizing (replacing). i.e
    /// in sql `'` becomes `''` which means one extra capacity is needed
    /// for each `'` in the statement
    fn extra_capacity(_s: &str) -> usize {
        0
    }

    /// write or replace each byte in `&str` to the `Vec` mut pointer
    fn process(p: *mut u8, _s: &str) -> *mut u8 {
        p
    }
}
