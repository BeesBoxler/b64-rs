use std::fs::read_to_string;

extern crate b64_rs;

use b64_rs::{decode, encode};
use std::time::Instant;

const COUNT: f64 = 1000.0;

fn main() {
    let text = load_text();
    let encoded_text = encode(&text);

    println!("\nEncoding {COUNT} times...");
    measure(|| {
        encode(&text);
    });

    println!("\nDecoding {COUNT} times...");
    measure(|| {
        decode(&encoded_text);
    });
}

fn load_text() -> String {
    let path = "examples/frankenstein.txt";

    read_to_string(path).unwrap()
}

fn measure<F: Fn() -> ()>(func: F) {
    let now = Instant::now();
    {
        for _ in 0..COUNT as u16 {
            func();
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Average: {:2?}ms", elapsed.as_millis() as f64 / COUNT);
}
