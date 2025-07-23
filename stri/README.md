# stri

`stri` is a set of procedural macros for string interpolation.

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

  // If the `chrono` feature is enabled

  // Note: chrono::{
  //    DateTime, Duration, FixedOffset, NaiveDate,
  //    NaiveDateTime, NaiveTime, TimeZone, Weekday
  // };
  // are the only supported types

  use chrono::NaiveDateTime;

  let dt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();

  assert_eq!(
      si!("date and time is: {dt}"),
      "date and time is: 2015-09-05 23:56:04"
  );

  assert_eq!(
      sql!("SELECT {dt}::TIMESTAMP AS date_time"),
      "SELECT '2015-09-05 23:56:04'::TIMESTAMP AS date_time"
  );

  //
  //
  //
  //
  //

  // if the `rust_decimal` feature is enabled

  use rust_decimal::Decimal;

  let d = Decimal::new(1225, 2); // 12.25;

  assert_eq!(si!("Decimal is: {d}"), "Decimal is: 12.25");

  assert_eq!(
      sql!("SELECT {d}::DECIMAL AS d"),
      "SELECT 12.25::DECIMAL AS d"
  );
}
```

### `file!` macro

For longer `string` or `sql` content, you can externalize it into a separate file and use the `file!` macro to include it. The `file!` macro intelligently applies the logic of the `si!` or `sql!` macros internally, determined by the extension of the specified file.

#### Example

**File: `src/example.txt`**

```txt
my name is {name}, i am {age} years old and my height is {height}
```

**File: `src/example.sql`**

```sql
INSERT INTO users (name, age, height, note) VALUES ({name}, {age}, {height}, {note})
```

```rust
use stri::file;
fn main(){
  let name = "Ahmed";
  let age = 63;
  let height = 180.5;
  let note = r#"My friend's name is Ali"#;

  //
  //
  //

  assert_eq!(
    file!("src/example.txt"),
    "my name is Ahmed, i am 63 years old and my height is 180.5",
  );

  //
  //
  //

  assert_eq!(
    file!("src/example.sql"),
    r#"INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, 'My friend''s name is Ali')"#,
  );
}
```

### `dir!` macro

The `dir!` macro allows you to concatenate the content of multiple files within a specified directory. It reads all files only at the first level of the given directory, joins their content with a newline character (`\n`), and intelligently processes them based on the file extensions within the directory:

- **If all files have the .sql extension:** `sql!` macro's logic will be applied.

- **Otherwise:** `si!` macro's logic will be applied.

**Note:** The order in which files are read and concatenated is dependent on the the behavior of `std::fs::ReadDir`.

#### Example

**Dir: `src`**

src/
├── sql
│ ├── 1_insert.sql
│ └── 2_delete.sql
├── txt
│ ├── 1_name.txt
│ └── 2_age.txt
└── main.rs

**File: `src/sql/1_insert.sql`**

```sql
INSERT INTO users (name, age, height, note) VALUES ({name}, {age}, {height}, {note});
```

**File: `src/sql/2_insert.sql`**

```sql
DELETE FROM users WHERE name={name};
```

**File: `src/txt/1_name.txt`**

```sql
My name is {name}
```

**File: `src/txt/2_age.txt`**

```sql
I am {age} years old
```

**File `src/main.rs`**

```rust
use stri::dir;
fn main(){
  let name = "Ahmed";
  let age = 63;

  //
  //
  //

  assert_eq!(
      dir!("src/sql"),
      "INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, 'My friend''s name is Ali');\nDELETE FROM users WHERE name='Ahmed';"
   );

   //
   //
   //

   assert_eq!(dir!("src/txt"), "My name is Ahmed\nI am 63 years old")
}
```

## Contribution

If you encounter any issues or want to suggest a feature, please open an [issue](https://github.com/infinite-xdev-void/stri/issues) in github.

## License Information

"stri" is licensed under Ethical Use License (EUL v1.0). see [LICENSE](https://github.com/infinite-xdev-void/stri/blob/main/LICENSE) for full license details.
