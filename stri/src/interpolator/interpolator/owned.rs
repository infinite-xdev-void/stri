//
//
//

pub struct Owned(String);

//
//
//

impl Owned {
    #[inline(always)]
    pub const fn new(s: String) -> Self {
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

impl super::Interpolator for Owned {
    //
    //
    //
    //
    //
    //
    //

    #[inline(always)]
    fn interpolate(&self, s: &mut String) {
        s.push_str(&self.0);
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
