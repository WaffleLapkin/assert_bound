# assert_bound
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
