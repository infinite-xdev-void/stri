//
//
//

pub trait Interpolator {
    fn len(&self) -> usize;
    fn interpolate(&self, s: &mut String);
}

//
//
//

mod refr;
pub use refr::*;

//
//
//

mod owned;
pub use owned::*;
