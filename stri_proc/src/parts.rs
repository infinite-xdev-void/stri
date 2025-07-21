//
//
//

use std::{collections::HashMap, marker::PhantomData};

//
//
//
//
//
//
//

use quote::{format_ident, quote};
use syn::Ident;

//
//
//
//
//
//
//

use crate::{Format, Ts2};

//
//
//
//
//
//
//

pub struct Parts<F: Format> {
   s_lens: usize,
   var_idens: HashMap<Vec<u8>, Ident>,
   vars: Ts2,
   v_lens: Ts2,
   v_count: usize,
   pushs: Ts2,
   f: PhantomData<F>,
}

//
//
//

impl<F: Format> Parts<F> {
   //
   //
   //
   #[inline(always)]
   pub fn new() -> Self {
      Self {
         s_lens: 0,
         var_idens: HashMap::new(),
         vars: Ts2::new(),
         v_lens: Ts2::new(),
         v_count: 0,
         pushs: Ts2::new(),
         f: PhantomData,
      }
   }

   //
   //
   //
   //
   //
   //
   //

   #[inline(always)]
   pub fn add_expr(&mut self, expr: &[u8]) {
      //
      //
      //

      match self.var_idens.get(expr) {
         Some(ident) => {
            extend(
               &mut self.v_lens,
               quote! {+ ::stri::Interpolator::len(&#ident)},
            );

            extend(
               &mut self.pushs,
               quote! {::stri::Interpolator::interpolate(&#ident, &mut s);},
            );
         }

         //
         //
         //
         //
         //
         None => {
            let ident = format_ident!("v{}", self.v_count);
            self.var_idens.insert(expr.to_vec(), ident.clone());

            let expr = unsafe { core::str::from_utf8_unchecked(expr) };

            let var_def = F::var_def(&ident, expr.trim());

            extend(&mut self.vars, var_def);
            extend(
               &mut self.v_lens,
               quote! {+ ::stri::Interpolator::len(&#ident)},
            );
            extend(
               &mut self.pushs,
               quote! {::stri::Interpolator::interpolate(&#ident, &mut s);},
            );
            self.v_count += 1;
         }
      };
   }

   //
   //
   //
   //
   //
   //
   //

   #[inline(always)]
   pub fn push_str(&mut self, s: &[u8]) {
      if s.len() == 0 {
         return;
      };

      let s = unsafe { String::from_utf8_unchecked(s.to_vec()) };
      self.s_lens += s.len();
      extend(&mut self.pushs, quote! {s.push_str(#s);});
   }

   //
   //
   //
   //
   //
   //
   //

   #[inline(always)]
   pub fn build(self) -> Ts2 {
      let Self {
         s_lens,
         var_idens: _,
         mut vars,
         v_lens,
         v_count,
         pushs,
         f: _,
      } = self;

      //
      //
      //

      if s_lens == 0 && v_count == 0 {
         return Ts2::new();
      };

      //
      //
      //
      //
      //
      //
      //

      extend(
         &mut vars,
         quote! {let mut s = String::with_capacity(#s_lens #v_lens);},
      );

      //
      //
      //
      //
      //
      //
      //

      quote! {
        {#vars

        #pushs

        s}
      }
   }
}

#[inline(always)]
pub fn extend(target: &mut Ts2, source: Ts2) {
   target.extend(core::iter::once(source));
}

/*
  1- strs == 1, vars == 0
  2- strs == 1, vars == 1
  3- strs ==

*/
