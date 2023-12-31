# Mutually Exclusive Features 

[![crates.io](https://img.shields.io/crates/v/mutually_exclusive_features.svg)](https://crates.io/crates/mutually_exclusive_features)
[![Build Status](https://github.com/omid/mutually_exclusive_features/actions/workflows/test.yml/badge.svg)](https://github.com/omid/mutually_exclusive_features/actions/workflows/test.yml)
[![docs](https://docs.rs/mutually_exclusive_features/badge.svg)](https://docs.rs/mutually_exclusive_features)

### Macros to check mutually exclusive `features` in Rust
___

It contains `none_or_one_of` and `exactly_one_of` macros.

Both check mutually exclusive features in Rust,
but `none_or_one_of` allows for no features to be enabled,
while `exactly_one_of` requires exactly one feature to be enabled.

## Usage

---

### none_or_one_of

---

Call it with the list of features you want to be mutually exclusive:
```rust
use mutually_exclusive_features::none_or_one_of;
none_or_one_of!("feature1", "feature2", "feature3");
```

Which will generate the following code:
```rust
#[cfg(all(feature="feature1", feature="feature2"))]
compile_error!("The `feature1` and `feature2` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature1", feature="feature3"))]
compile_error!("The `feature1` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature2", feature="feature3"))]
compile_error!("The `feature2` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");
```

### exactly_one_of

---

It's the same, but requires exactly one feature to be enabled:
```rust
use mutually_exclusive_features::exactly_one_of;
exactly_one_of!("feature1", "feature2", "feature3");
```

Which will generate the following code:
```rust
#[cfg(all(feature="feature1", feature="feature2"))]
compile_error!("The `feature1` and `feature2` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature1", feature="feature3"))]
compile_error!("The `feature1` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature2", feature="feature3"))]
compile_error!("The `feature2` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(not(any(feature="feature1", feature="feature2", feature="feature3")))]
compile_error!("You must enable exactly one of `feature1`, `feature2`, `feature3` features!");
```
