#![feature(test)]

use rand::Rng;

pub fn random_vector(i: i32) -> Vec<i32> {
  let mut numbers: Vec<i32> = Vec::new();
  let mut rng = rand::thread_rng();
  for _k in 0..i {
    numbers.push(rng.gen());
  }
  numbers
}

pub fn simple_stock_span(quotes: &Vec<i32>) -> Vec<i32> {
  let n: i32 = quotes.len() as i32;
  let mut spans: Vec<i32> = Vec::with_capacity(n as usize);
  spans.resize(n as usize, 0);
  for i in 0..n {
    let mut k: i32 = 1;
    let mut span_end = true;
    while i - k >= 0 && !span_end {
      if quotes[(i - k) as usize] <= quotes[i as usize] {
        k = k + 1;
      } else {
        span_end = true;
      }
    }
    spans[i as usize] = k as i32;
  }

  spans
}

extern crate test;
use test::Bencher;

#[bench]
fn bench_simple_stock_span(b: &mut Bencher) {
  b.iter(|| {
    let numbers: Vec<i32> = random_vector(100);
    simple_stock_span(&numbers)
  });
}

fn main() {
  println!("Hello, world!");
}
