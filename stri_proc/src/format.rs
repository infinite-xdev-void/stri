//
//
//

use std::str::FromStr;

//
//
//
//
//
//
//

use quote::quote;

//
//
//

use syn::Ident;

//
//
//
//
//
//
//

use crate::Ts2;

//
//
//
//
//
//
//

pub trait Format {
    fn var_def(ident: &Ident, expr: &str) -> Ts2;
}

//
//
//
//
//
//
//

pub struct Str;

//
//
//

impl Format for Str {
    #[inline(always)]
    fn var_def(ident: &Ident, expr: &str) -> Ts2 {
        let expr = match Ts2::from_str(expr) {
            Ok(expr) => expr,
            Err(e) => {
                panic!("str: {}\n e: {:?}", expr, e);
            }
        };
        quote! {let #ident = ::stri::ToInterpolator::<{::stri::STR}>::to_interpolator(&#expr);}
    }
}

//
//
//
//
//
//
//

pub struct Sql;

impl Format for Sql {
    #[inline(always)]
    fn var_def(ident: &Ident, expr: &str) -> Ts2 {
        match expr.strip_prefix("~html") {
            Some(expr) => {
                let expr = Ts2::from_str(expr.trim()).unwrap();

                quote! {let #ident = ::stri::to_sql_string_interpolator::<::stri::sanitizers::Html>(&#expr);}
            }

            None => {
                let expr = Ts2::from_str(expr).unwrap();
                quote! {let #ident = ::stri::ToInterpolator::<{::stri::SQL}>::to_interpolator(&#expr);}
            }
        }
    }
}
