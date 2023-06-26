# Small String Optimization

Bot [TinyVec](https://crates.io/crates/tinyvec) and [SmallVec](https://crates.io/crates/smallvec) have the same minimum size as a `Vac` — 24 bytes on 64-bit platforms. This implementation manages to get that down to 16 bytes. For a large number of strings, this swings adds up. Not just in memory usage, but cache utilizations as well.



## Limitations

Strings that are too large to store inline are stored on the heap, as a `CString`, rather than using a `String` or `Vec` . As such, embedded NUL characters are not allowed in the strings that are to be stored. A  `std::ffi::NulError` will be returned if an attempt is made to construct an `sso::Storage` form an unsupported `str`.

```rust, ignore
pub fn from_str(str: &str) -> Result<Storage, NulError>
```

Using this crate is implicitly assuming g that handling long strings “as slow as C” will be fast enough.



## Safety

This crate uses `unsafe` as it use a Rust [union](https://doc.rust-lang.org/reference/items/unions.html) internally to lower the overhead. However

1. The code is purposefully kept small/simple to simplify manual auditing.
2. [Property testing](https://github.com/BurntSushi/quickcheck#readme) is done to ensure that it works on a large variety of strings.
3. Every test is run under [Miri](https://github.com/rust-lang/miri#readme) [on every push](https://github.com/bwoods/immutable-sso/actions) to help check the vanity of the `unsafe` code.



## Usage

This repo can be added to your `Cargo.toml` file directly.

```yaml
[dependencies.sso]
git = "https://github.com/bwoods/immutable-sso"
tag = "0.3.3"
```

