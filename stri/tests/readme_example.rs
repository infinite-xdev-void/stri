//
//
//

use stri::{dir, file, si, sql};

//
//
//
//
//
//
//

#[test]
fn test() {
   let name = "Ahmed";
   let age = 63;
   let height = 180.5;

   // `si!`: str interpolation used to interpolate variables normally
   assert_eq!(
      si!(
         "my name is {name}, i am {age} years old and my height is {height}"
      ),
      "my name is Ahmed, i am 63 years old and my height is 180.5",
   );

   //
   //
   //

   let note = r#"My friend's name is Ali"#;

   // but for `sql` where `String` must be wrapped with single quote and each quote
   // inside it must be escaped then `sql!` is used.
   assert_eq!(
      sql!(
         "INSERT INTO users (name, age, height, note) VALUES ({name}, {age}, {height}, {note})"
      ),
      r#"INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, 'My friend''s name is Ali')"#,
   );

   //
   //
   //

   let note = r#"[' " > < &]"#; // these are html special characters: ' " > < &

   // if you want to sanitize html special characters then add `~html` as a suffix to
   // the variable name (works with `&str` and `String` only)
   assert_eq!(
      sql!(
         "INSERT INTO users (name, age, height, note) VALUES ({name}, {age}, {height}, {~html note})"
      ),
      r#"INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, '[&#39; &#34; &gt; &lt; &amp;]')"#,
   );

   //
   //
   //

   // `file!` example

   let note = r#"My friend's name is Ali"#;

   //
   //
   //

   assert_eq!(
      file!("tests/example.txt"),
      "my name is Ahmed, i am 63 years old and my height is 180.5",
   );

   //
   //
   //

   assert_eq!(
      file!("tests/example.sql"),
      r#"INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, 'My friend''s name is Ali')"#,
   );

   //
   //
   //

   // `dir!` example

   assert_eq!(
      dir!("tests/sqls"),
      "INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, 'My friend''s name is Ali');\nDELETE FROM users WHERE name='Ahmed';"
   );

   //
   //
   //

   assert_eq!(dir!("tests/txts"), "My name is Ahmed\nI am 63 years old")
}

//
//
//
//
//
//
//

#[cfg(feature = "chrono")]
#[test]
fn chrono() {
   use chrono::NaiveDateTime;

   //
   //
   //

   let dt = NaiveDateTime::parse_from_str(
      "2015-09-05 23:56:04",
      "%Y-%m-%d %H:%M:%S",
   )
   .unwrap();

   assert_eq!(
      si!("date and time is: {dt}"),
      "date and time is: 2015-09-05 23:56:04"
   );

   assert_eq!(
      sql!("SELECT {dt}::TIMESTAMP AS date_time"),
      "SELECT '2015-09-05 23:56:04'::TIMESTAMP AS date_time"
   );
}

//
//
//
//
//
//
//

#[cfg(feature = "rust_decimal")]
#[test]
fn rust_decimal() {
   use rust_decimal::Decimal;

   //
   //
   //

   let d = Decimal::new(1225, 2); // 12.25;

   assert_eq!(si!("Decimal is: {d}"), "Decimal is: 12.25");

   assert_eq!(
      sql!("SELECT {d}::DECIMAL AS d"),
      "SELECT 12.25::DECIMAL AS d"
   );
}
