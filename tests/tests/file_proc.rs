//
//
//

use stri_proc::file;

//
//
//

#[test]
fn file_txt() {
    let name = "Ahmed";
    let age = 63;
    let height = 180.5;

    assert_eq!(
        file!("tests/file_proc/file.txt"),
        "my name is Ahmed, i am 63 years old and my height is 180.5",
    )
}

//
//
//
//
//
//
//

#[test]
fn file_sql() {
    let name = "Ahmed";
    let age = 63;
    let height = 180.5;
    let note = r#"[' " > < &]"#;

    assert_eq!(
        file!("tests/file_proc/file.sql"),
        r#"INSERT INTO users (name, age, height, note) VALUES ('Ahmed', 63, 180.5, '[&#39; &#34; &gt; &lt; &amp;]')"#,
    );
}
