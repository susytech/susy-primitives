# bigint

[![Build Status](https://travis-ci.org/susytech/susy-primitives.svg?branch=master)](https://travis-ci.org/susytech/susy-primitives)

Fixed-sized integers arithmetic

To specify a dependency, add to `Cargo.toml`

```toml
[dependencies]
sophon-types = "0.3"
```

Little example

```rust
extern crate sophon_types;
use sophon_types::U256;

fn main() {
	let mut val: U256 = 1023.into();
	for _ in 0..200 { val = val * 2u32 }
	assert_eq!(
		&format!("{}", val),
		"1643897619276947051879427220465009342380213662639797070513307648"
	);
}
```

### `no_std` crates

This crate has a feature, `std`, that is enabled by default. To use this crate
in a `no_std` context, add the following to your `Cargo.toml`:

```toml
[dependencies]
sophon-types = { version = "0.3", default-features = false }
```
