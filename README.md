[![Crate](https://img.shields.io/crates/v/settimeout.svg)](https://crates.io/crates/settimeout)
[![API](https://docs.rs/settimeout/badge.svg)](https://docs.rs/settimeout)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.39+-green.svg)](https://github.com/rousan/settimeout-rs)

# settimeout-rs

Provides an implementation of `std::future::Future` trait to be ready at some point. Sometimes,
it is needed a `std::future::Future` trait instance for testing purpose in any `async` function.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
settimeout = "0.1.0"
```

## Examples

Create a simple `std::future::Future` implementation:

```rust
use futures::executor::block_on;
use std::time::Duration;
use settimeout::set_timeout;

async fn foo() {
  println!("The Future will be ready after some time");
  set_timeout(Duration::from_secs(5)).await;
  println!("Now, it is ready");
}

fn main() {
   block_on(foo());
}
```

## Contributing

Your PRs and stars are always welcome.