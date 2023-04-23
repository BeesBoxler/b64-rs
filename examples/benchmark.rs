use std::fs::read_to_string;

extern crate b64_rs;

use b64_rs::{encode, decode};
use std::time::Instant;

const COUNT: u16 = 1000;

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
        for _ in 0..COUNT {
            func();
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Average: {:2?}ms", elapsed.as_millis() / COUNT as u128);
}
