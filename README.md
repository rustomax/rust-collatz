# rust-collatz
Rust iterator for [Collatz conjecture](http://en.wikipedia.org/wiki/Collatz_conjecture)

[![Build Status](https://travis-ci.org/rustomax/rust-collatz.svg?branch=master)](https://travis-ci.org/rustomax/rust-collatz)

How to compile and run sample code:

```sh
git clone https://github.com/rustomax/rust-collatz.git
cd rust-collatz
cargo run
```

## Usage
Import the `collatz` module:

```rust
mod collatz;
use collatz::Collatz;
```

Instantiate the Iterator

```rust
let c = Collatz::new(100);
```

Iterate through values:

```rust
for i in c {
    print!("{} ", i);
}
// output: 100 50 25 76 38 19 58 29 88 44 22 11 34 17 52 26 13 40 20 10 5 16 8 4 2 1
```
