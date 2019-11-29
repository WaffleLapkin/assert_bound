# assert_bound
[![CI status](https://github.com/WaffleLapkin/assert_bound/workflows/Continuous%20integration/badge.svg)](https://github.com/WaffleLapkin/assert_bound/actions)
[![Telegram](https://img.shields.io/badge/tg-WaffleLapkin-9cf?logo=telegram)](https://vee.gg/t/WaffleLapkin)
[![docs.rs](https://img.shields.io/badge/docs.rs-link-blue.svg)](https://docs.rs/assert_bound)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.1.1-orange.svg)](https://crates.io/crates/assert_bound)

Simple rust lib for type assertions.

## Examples

```rust
use assert_bound::assert_bound;

// assert that `()` implements Debug + Ord + PartialEq<()>
assert_bound!(() => std::fmt::Debug + Ord + PartialEq<()>);

// f32/f64 doesn't implement `Eq`, so rustc will 
// fail to compile next line if it you uncomment it
// assert_bound!(0.1 => Eq);
```
