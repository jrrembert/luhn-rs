# luhn_rs

[![CI](https://github.com/jrrembert/luhn_rs/workflows/CI/badge.svg)](https://github.com/jrrembert/luhn_rs/actions)
[![codecov](https://codecov.io/gh/jrrembert/luhn_rs/branch/main/graph/badge.svg)](https://codecov.io/gh/jrrembert/luhn_rs)
[![Crates.io](https://img.shields.io/crates/v/luhn_tools.svg)](https://crates.io/crates/luhn_tools)
[![Documentation](https://docs.rs/luhn_tools/badge.svg)](https://docs.rs/luhn_tools)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.65.0-blue.svg)](https://github.com/jrrembert/luhn_rs)

A fast, minimal implementation of the Luhn algorithm.

- Generate checksums for Luhn numbers
- Validate Luhn numbers
- Generate random valid Luhn numbers
- No dependencies (optionally includes `rand` for random generation)
- Comprehensive error handling
- Tested and benchmarked

## Installation

Add one of these to your `Cargo.toml`:

```toml
# Option 1: Use defaults (includes `std`)
[dependencies]
luhn_algo = "0.3.0"

# Option 2: Include `random` feature
[dependencies]
luhn_algo = { version = "0.3.0", features = ["random"] }
```

## Usage

```rust
use luhn_tools::{generate, validate, random, GenerateOptions};

// Generate a checksum and return new Luhn number
let result = generate("7992739871", None).unwrap();
assert_eq!(result, "79927398713");

// Generate only the checksum
let options = Some(GenerateOptions { checksum_only: true });
let checksum = generate("7992739871", options).unwrap();
assert_eq!(checksum, "3");

// Validate a Luhn number
assert!(validate("79927398713").unwrap());

// Generate a random valid Luhn number of length 10
let random_number = random("10").unwrap();
assert!(validate(&random_number).unwrap());
```

```bash
# view docs
$ cargo doc --no-deps --open

# run tests
$ cargo test --all-features

# run benchmarks
$ cargo bench
```

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## License

MIT