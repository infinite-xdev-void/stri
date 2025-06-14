# stri

`stri` is a procedural macro for string interpolation.

## Usage Example

```rust
use stri::{si, sql};

fn main(){
  let name = "Ahmed";
  let age = 63;
  let height = 180.5;


  // `si!`: str interpolation used to interpolate variables normally
  assert_eq!(
    si!("my name is {name}, i am {age} years old and my height is {height}"),
    "my name is Ahmed, i am 63 years old and my height is 180.5",
  );

  //
  //
  //

  let note = r#"My friend's name is Ali"#;

  // but for `sql` where `String` must be wrapped with single quote and each quote
  // inside it must be escaped then `sql!` is used.
  assert_eq!(
    sql!("INSERT INTO users (name, age, height, note) VALUES ({name}, {age}, {height}, {note})"),
    r#"INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, 'My friend''s name is Ali')"#,
  );

  //
  //
  //

  let note = r#"[' " > < &]"#; // these are html special characters: ' " > < &

  // if you want to sanitize html special characters then add `~html` as a suffix to
  // the variable name (works with `&str` and `String` only)
  assert_eq!(
    sql!("INSERT INTO users (name, age, height, note) VALUES ({name}, {age}, {height}, {~html note})"),
    r#"INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, '[&#39; &#34; &gt; &lt; &amp;]')"#,
  );

  //
  //
  //
  //
  //

  // in case of `cargo add stri -F chrono`
  use chrono::NaiveDateTime;

  let dt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();

  assert_eq!(
      si!("date and time is: {dt}"),
      "date and time is: 2015-09-05 23:56:04"
  );

  assert_eq!(
      sql!("SELECT {dt}::DATETIME AS date_time"),
      "SELECT '2015-09-05 23:56:04'::DATETIME AS date_time"
  );
}
```

## Contribution

If you encounter any issues or want to suggest a feature, please open an [issue](https://github.com/infinite-xdev-void/stri/issues) in github.

## License Information

"stri" is licensed under Ethical Use License (EUL v1.0). see LICENSE for full license details.
