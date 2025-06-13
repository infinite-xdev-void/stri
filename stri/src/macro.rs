//
//
//

/// Implements the `Interpolate` trait for a given list of types.
///
/// This macro provides a convenient way to implement `Interpolate` for multiple types
/// by leveraging their existing `ToString` implementation. The generated `Interpolate`
/// methods will internally call `to_string()` on the target type.

#[macro_export]
macro_rules! impl_to_interpolator {
    ($c: path; $($t: ty),+) => {
        $(
          impl $crate::ToInterpolator<{$c}> for $t {
            type Out<'a> = $crate::Owned where Self: 'a;
            #[inline(always)]
            fn to_interpolator(&self) -> $crate::Owned {
              $crate::Owned::new(self.to_string())
            }
          }
        )+
    };
}

/*
///
///
///
/// # Usage
/// ```rust
/// use stri::impl_interpolate;
///
/// // Example of type that implement ToString
/// struct MyStruct(u32);
/// impl ToString for MyStruct {
///     fn to_string(&self) -> String {
///         format!("MyStruct value: {}", self.0)
///     }
/// }
///
///
/// // Apply the macro to multiple types:
/// impl_interpolate!(MyStruct);
///

*/
