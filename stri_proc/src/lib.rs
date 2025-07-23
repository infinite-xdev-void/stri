//
//
//

use std::{fmt::Display, fs::ReadDir, os::windows::fs::FileTypeExt};

//
//
//
//
//
//
//

use proc_macro::TokenStream as Ts;

//
//
//
//
//
//
//

use proc_macro2::TokenStream as Ts2;

//
//
//
//
//
//
//

use syn::{LitStr, parse_macro_input, spanned::Spanned};

//
//
//
//
//
//
//

mod format;
use format::{Format, Sql, Str};

//
//
//
//
//
//
//

mod parts;
use parts::Parts;

//
//
//
//
//
//
//

#[proc_macro]
pub fn si(i: Ts) -> Ts {
   From::from(build_str::<Str>(parse_macro_input!(i as LitStr).value()))
}

//
//
//

#[proc_macro]
pub fn sql(i: Ts) -> Ts {
   From::from(build_str::<Sql>(parse_macro_input!(i as LitStr).value()))
}

//
//
//

#[proc_macro]
pub fn file(f: Ts) -> Ts {
   let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect(
      "CARGO_MANIFEST_DIR not set. This macro should be compiled by Cargo.",
   );

   //
   //
   //

   let file_name = parse_macro_input!(f as LitStr).value();

   let path = std::path::PathBuf::from(manifest_dir).join(&file_name);

   //
   //
   //

   let file = std::fs::read_to_string(&path).unwrap();

   if file_name.ends_with(".sql") {
      From::from(build_str::<Sql>(file))
   } else {
      From::from(build_str::<Str>(file))
   }
}

//
//
//
//
//
//
//
#[proc_macro]
pub fn dir(d: Ts) -> Ts {
   let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect(
      "CARGO_MANIFEST_DIR not set. This macro should be compiled by Cargo.",
   );

   let dir_name = parse_macro_input!(d as LitStr).value();

   let path = std::path::PathBuf::from(manifest_dir).join(&dir_name);

   //
   //
   //

   let (val, is_sql) =
      concat_first_lvl_files_in_dir(std::fs::read_dir(path).unwrap());

   //

   From::from(match is_sql {
      true => build_str::<Sql>(val),
      false => build_str::<Str>(val),
   })
}

//
//
//

#[inline(always)]
fn concat_first_lvl_files_in_dir(dir: ReadDir) -> (String, bool) {
   let mut contents = Vec::new();
   let mut all_sql = true;

   let mut file_type;

   for entity in dir {
      let entity = entity.unwrap();
      file_type = entity.file_type().unwrap();
      if file_type.is_file() || file_type.is_symlink_file() {
         let path = entity.path();

         if let Some(e) = path.extension() {
            if e != "sql" {
               all_sql = false;
            }
         } else {
            all_sql = false;
         }

         contents
            .push(std::fs::read_to_string(path).unwrap().trim().to_owned());
      };
   }

   if contents.len() == 0 {
      panic!("there are no files in the first level of the directory.")
   }

   (contents.join("\n"), all_sql)
}

//
//
//
//
//
//
//

fn build_str<F: Format>(val: String) -> Ts2 {
   //
   //
   //

   let mut parts = Parts::<F>::new();

   let bs = val.as_bytes();

   let mut s = Vec::new();
   let mut e = Vec::new();

   //
   //
   //

   let mut i = 0;

   'outer: while i < bs.len() {
      //
      //
      //

      match bs[i] {
         //
         //
         //
         b'{' => {
            let Some(n) = bs.get(i + 1) else {
               return compile_err(
                  &i,
                  "invalid string: expected expression but string was terminated if you intended to add `{`, you can escape it using `{{`",
               );
            };

            //
            //
            //

            if n.eq(&b'{') {
               s.push(b'{');
               i += 2;
               continue 'outer;
            };

            //
            //
            //
            //
            //
            //
            //

            parts.push_str(&s);
            s.clear();

            //
            //
            //

            e.push(*n);
            i += 2;

            //
            //
            //

            let mut depth = 0;

            //
            //
            //

            while let Some(b) = bs.get(i) {
               if b.eq(&b'{') {
                  depth += 1;
               };

               if b.eq(&b'}') {
                  if let Some(n) = bs.get(i + 1) {
                     if n.eq(&b'}') {
                        e.push(b'}');
                        e.push(b'}');
                        i += 2;
                        continue;
                     };
                  };

                  if depth == 0 {
                     parts.add_expr(&e);
                     e.clear();
                     i += 1;
                     continue 'outer;
                  } else {
                     depth -= 1;
                  }
               };

               e.push(*b);
               i += 1;
            }

            //
            //
            //

            return compile_err(
               &val,
               "invalid string: expected `}` but string was terminated",
            );
         }

         b @ _ => s.push(b),
      }

      i += 1;
   }

   parts.push_str(&s);

   parts.build()
}

//
//
//
//
//
//
//

/* #[inline(always)]

fn get_expr<S: Spanned>(s: &S, iter: &mut Iter<'_, u8>) -> Result<Ts2, Ts2> {
    let mut depth = 0;
    let mut var = Vec::<u8>::new();

    while let Some(b) = iter.next() {
        match b {
            b'{' => depth += 1,

            b'}' => {
                if depth != 0 {
                    depth -= 1;
                    continue;
                }

                return Ok(Ts2::from_str(&unsafe { String::from_utf8_unchecked(var) }).unwrap());
            }

            _ => var.push(*b),
        };
    }

    Err(compile_err(
        s,
        "invalid string: expected `}` but string was terminated",
    ))
}
 */
//
//
//
//
//
//
//

#[inline(always)]
fn compile_err<S: Spanned, M: Display>(s: &S, m: M) -> Ts2 {
   syn::Error::new(s.span(), m).to_compile_error()
}
