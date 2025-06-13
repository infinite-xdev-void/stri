//
//
//

pub trait ToInterpolator<const T: usize> {
    type Out<'a>
    where
        Self: 'a;
    fn to_interpolator(&self) -> Self::Out<'_>;
}

//
//
//
//
//
//
//

mod interpolators;
pub use interpolators::*;

//
//
//

mod impls;
pub use impls::*;
