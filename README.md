<h1 align='center'>b64-rs
</h1>
<p align='center'>
    🏎️ A super speedy* base64 encoder 🏎️
</p>
<sub align='right'>
*according to me
</sub>
<p align="center">
    <a href="https://github.com/BeesBoxler/b64-rs/actions/workflows/run-tests.yaml" alt="Tests">
        <img src="https://img.shields.io/github/actions/workflow/status/beesboxler/b64-rs/run-tests.yaml?style=flat-square&label=tests" />
    </a>
    </p>

## Usage

```rust
use b64_rs::{encode,decode};

let encoded_string = encode("b64-rs goes 🏎️💨");
print!("{encoded_string}");
// => YjY0LXJzIGdvZXMg8J+Pju+4j/Cfkqg=

let decoded_string = decode(&encoded_string);
print!("decoded_string");
// => b64-rs goes 🏎️💨
```
*b64-rs* is lightweight, and requires no dependencies. Ez.


<p align="center">💛</p>