//
//
//

pub struct Refr<'a>(&'a str);

//
//
//

impl<'a> Refr<'a> {
    //
    //
    //

    #[inline(always)]
    pub const fn new(s: &'a str) -> Self {
        Self(s)
    }
}

//
//
//
//
//
//
//

impl<'a> super::Interpolator for Refr<'a> {
    //
    //
    //
    //
    //
    //
    //

    #[inline(always)]
    fn interpolate(&self, s: &mut String) {
        s.push_str(self.0);
    }

    //
    //
    //
    //
    //
    //
    //

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}
