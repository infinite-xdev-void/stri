//
//
//

pub trait Interpolate {
   fn len(&self) -> usize;
   fn interpolate(&self, target: &mut String);
}

//
//
//

impl Interpolate for String {
   #[inline(always)]
   fn len(&self) -> usize {
      // calling `self.len()` will case `recursive call site` because it will call the `len` method of the `Interpolate` trait not the `String` struct
      <String>::len(self)
   }

   #[inline(always)]
   fn interpolate(&self, target: &mut String) {
      target.push_str(self)
   }
}

impl Interpolate for &str {
   #[inline(always)]
   fn len(&self) -> usize {
      // calling `self.len()` will case `recursive call site` because it will call the `len` method of the `Interpolate` trait not the `str` struct
      <str>::len(*self)
   }

   #[inline(always)]
   fn interpolate(&self, target: &mut String) {
      target.push_str(*self)
   }
}
