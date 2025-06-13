//
//
//

pub trait Interpolate {
    fn interpolate(&self, s: &mut String);
}

//
//
//
//
//
//
//

/* impl Interpolate for String {
    #[inline(always)]
    fn interpolate(&self, s: &mut String) {
        s.push_str(self);
    }
} */

//
//
//
//
//
//
//

impl Interpolate for &str {
    #[inline(always)]
    fn interpolate(&self, s: &mut String) {
        s.push_str(*self);
    }
}
