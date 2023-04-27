use std::fs::read_to_string;

extern crate b64_rs;

use b64_rs::{decode, encode};
use std::time::Instant;

const COUNT: usize = 1000;

fn main() {
    let text = load_text();
    let encoded_text = encode(&text);

    println!("\nEncoding {COUNT} times...");
    println!("========================");
    measure(COUNT, || {
        encode(&text);
    });

    println!("\nDecoding {COUNT} times...");
    println!("========================");
    measure(COUNT, || {
        decode(&encoded_text);
    });
}

fn load_text() -> String {
    let path = "examples/frankenstein.txt";

    read_to_string(path).unwrap()
}

fn measure<F: Fn() -> ()>(count: usize, func: F) {
    let mut times = vec![];
    {
        for _ in 0..COUNT as u16 {
            let now = Instant::now();
            func();
            let elapsed = now.elapsed();

            times.push(elapsed.as_micros());
        }
    }

    let unit = "μs";
    let elapsed: u128 = times.iter().sum();
    let max = times.iter().max().unwrap();
    let min = times.iter().min().unwrap();
    let avg = elapsed as f64 / count as f64;
    let sq_difs: Vec<f64> = times
        .iter()
        .map(|v| (*v as i128 - avg as i128).pow(2) as f64)
        .collect();
    let std_dev = f64::sqrt(sq_difs.iter().sum::<f64>() / count as f64);

    println!("Elapsed: {:.2?}s", elapsed as f64 / 1_000_000.0);
    println!("Average: {avg:.2}{unit}");
    println!("Min:     {min:2}{unit}");
    println!("Max:     {max:2}{unit}");
    println!("Std dev: {std_dev:.2}{unit}");
}
