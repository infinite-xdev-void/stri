//
//
//

use criterion::{Criterion, black_box, criterion_group, criterion_main};

//

use rand::Rng;

//

use stri::{si, sql};

//

use rust_decimal::Decimal;

//

use chrono::{DateTime, Local};

//
//
//
//
//

fn format(
   name: &str,
   age: u8,
   salary: u16,
   len: f32,
   very_large_num: i128,
   decimal: Decimal,
   dt: DateTime<Local>,
) -> String {
   format!(
      "name: {}, age: {}, salary: {}, len: {}, very_large_num: {}, decimal: {}, dt: {}",
      name, age, salary, len, very_large_num, decimal, dt,
   )
}

fn si(
   name: &str,
   age: u8,
   salary: u16,
   len: f32,
   very_large_num: i128,
   decimal: Decimal,
   dt: DateTime<Local>,
) -> String {
   si!(
      "name: {name}, age: {age}, salary: {salary}, len: {len}, very_large_num: {very_large_num}, decimal: {decimal}, dt: {dt}"
   )
}

fn sql(
   name: &str,
   age: u8,
   salary: u16,
   len: f32,
   very_large_num: i128,
   decimal: Decimal,
   dt: DateTime<Local>,
) -> String {
   sql!(
      "name: {name}, age: {age}, salary: {salary}, len: {len}, very_large_num: {very_large_num}, decimal: {decimal}, dt: {dt}"
   )
}

fn criterion_benchmark(c: &mut Criterion) {
   let mut rng = rand::thread_rng();

   c.bench_function("si macro", |b| {
      b.iter(|| {
         black_box(si(
            black_box("ali"),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(Decimal::new(rng.r#gen(), 5)),
            black_box(Local::now()),
         ))
      });
   });

   c.bench_function("sql macro", |b| {
      b.iter(|| {
         black_box(sql(
            black_box("ali"),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(Decimal::new(rng.r#gen(), 5)),
            black_box(Local::now()),
         ))
      });
   });

   c.bench_function("format macro", |b| {
      b.iter(|| {
         black_box(format(
            black_box("ali"),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(rng.r#gen()),
            black_box(Decimal::new(rng.r#gen(), 5)),
            black_box(Local::now()),
         ))
      });
   });

   // c.bench_function("sql macro", |b| {
   //    b.iter(|| {
   //       sql(
   //          black_box("ali"),
   //          black_box(8),
   //          black_box(1500),
   //          black_box(180.5),
   //          black_box(57895489754),
   //       )
   //    });
   // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
