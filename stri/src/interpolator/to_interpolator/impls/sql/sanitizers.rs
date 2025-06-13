//
//
//

use super::Sanitizer;

//
//
//
//
//
//

pub struct SingleQuote;

//
//
//

unsafe impl Sanitizer for SingleQuote {
    #[inline(always)]
    fn extra_capacity(s: &str) -> usize {
        let mut count = 0;
        for b in s.as_bytes().iter() {
            if b.eq(&b'\'') {
                count += 1;
            };
        }

        count
    }

    //
    //
    //
    //
    //
    //
    //

    #[inline(always)]
    fn process(mut p: *mut u8, s: &str) -> *mut u8 {
        //
        //
        //
        unsafe {
            for b in s.as_bytes().iter() {
                if b.eq(&b'\'') {
                    core::ptr::write(p, b'\'');
                    p = p.add(1);
                    core::ptr::write(p, b'\'');
                } else {
                    core::ptr::write(p, *b);
                };
                p = p.add(1);
            }
        };

        p
    }
}

//
//
//
//
//
//
//
//
//
//
//
//
//

pub struct Html;

//
//
//

impl Html {
    /// `&`
    const AMPERSAND: &[u8] = "&amp;".as_bytes();

    /// `'`
    const SINGLE_QUOTE: &[u8] = "&#39;".as_bytes();

    /// `<`
    const LESS_THAN_SIGN: &[u8] = "&lt;".as_bytes();

    /// `>`
    const GREATER_THAN_SIGN: &[u8] = "&gt;".as_bytes();

    /// `"`
    const DOUBLE_QUOTE: &[u8] = "&#34;".as_bytes();
}

//
//
//

unsafe impl Sanitizer for Html {
    //
    //
    //
    #[inline(always)]
    fn extra_capacity(s: &str) -> usize {
        let mut count = 0;

        for b in s.as_bytes().iter() {
            //
            //
            //

            match *b {
                b'&' => count += Html::AMPERSAND.len() - 1,

                b'\'' => count += Html::SINGLE_QUOTE.len() - 1,

                b'<' => count += Html::LESS_THAN_SIGN.len() - 1,

                b'>' => count += Html::GREATER_THAN_SIGN.len() - 1,

                b'"' => count += Html::DOUBLE_QUOTE.len() - 1,

                _ => {}
            };
        }

        count
    }

    //
    //
    //
    //
    //
    //
    //
    #[inline(always)]
    fn process(mut p: *mut u8, s: &str) -> *mut u8 {
        //
        //
        //
        unsafe {
            for b in s.as_bytes().iter() {
                //
                //
                //

                match *b {
                    b'&' => {
                        core::ptr::write(p, Html::AMPERSAND[0]);
                        p = p.add(1);
                        core::ptr::write(p, Html::AMPERSAND[1]);
                        p = p.add(1);
                        core::ptr::write(p, Html::AMPERSAND[2]);
                        p = p.add(1);
                        core::ptr::write(p, Html::AMPERSAND[3]);
                        p = p.add(1);
                        core::ptr::write(p, Html::AMPERSAND[4]);
                    }

                    //
                    //
                    //
                    b'\'' => {
                        core::ptr::write(p, Html::SINGLE_QUOTE[0]);
                        p = p.add(1);
                        core::ptr::write(p, Html::SINGLE_QUOTE[1]);
                        p = p.add(1);
                        core::ptr::write(p, Html::SINGLE_QUOTE[2]);
                        p = p.add(1);
                        core::ptr::write(p, Html::SINGLE_QUOTE[3]);
                        p = p.add(1);
                        core::ptr::write(p, Html::SINGLE_QUOTE[4]);
                    }

                    //
                    //
                    //
                    b'<' => {
                        core::ptr::write(p, Html::LESS_THAN_SIGN[0]);
                        p = p.add(1);
                        core::ptr::write(p, Html::LESS_THAN_SIGN[1]);
                        p = p.add(1);
                        core::ptr::write(p, Html::LESS_THAN_SIGN[2]);
                        p = p.add(1);
                        core::ptr::write(p, Html::LESS_THAN_SIGN[3]);
                    }

                    //
                    //
                    //
                    b'>' => {
                        core::ptr::write(p, Html::GREATER_THAN_SIGN[0]);
                        p = p.add(1);
                        core::ptr::write(p, Html::GREATER_THAN_SIGN[1]);
                        p = p.add(1);
                        core::ptr::write(p, Html::GREATER_THAN_SIGN[2]);
                        p = p.add(1);
                        core::ptr::write(p, Html::GREATER_THAN_SIGN[3]);
                    }

                    //
                    //
                    //
                    b'"' => {
                        core::ptr::write(p, Html::DOUBLE_QUOTE[0]);
                        p = p.add(1);
                        core::ptr::write(p, Html::DOUBLE_QUOTE[1]);
                        p = p.add(1);
                        core::ptr::write(p, Html::DOUBLE_QUOTE[2]);
                        p = p.add(1);
                        core::ptr::write(p, Html::DOUBLE_QUOTE[3]);
                        p = p.add(1);
                        core::ptr::write(p, Html::DOUBLE_QUOTE[4]);
                    }

                    //
                    //
                    //
                    _ => core::ptr::write(p, *b),
                };

                //
                //
                //

                p = p.add(1);
            }
        };

        p
    }
}

/*

    `&`, "&amp;",
    `'`, "&#39;", // "&#39;" is shorter than "&apos;" and apos was not in HTML until HTML5.
    `<`, "&lt;",
    `>`, "&gt;",
    `"`, "&#34;", // "&#34;" is shorter than "&quot;".
*/
