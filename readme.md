# chained_iter

[![Latest Version](https://img.shields.io/crates/v/chained_iter.svg)](https://crates.io/crates/chained_iter) [![Docs](https://docs.rs/chained_iter/badge.svg)](https://docs.rs/chained_iter) ![License](https://img.shields.io/crates/l/chained_iter)

```toml
[dependencies]
chained_iter = "0.1"
```

Provides a small helper macro for creating iterators out of values without allocation.

## Usage
```rust
chained_iter![1, 2, 3, 4].collect::<Vec<_>>()
```
