# Small String Optimization

A  “string container” that starts hold short string inline, but will fallback to storing longer strings on the heap.

Both [TinyVec](https://crates.io/crates/tinyvec) and [SmallVec](https://crates.io/crates/smallvec) have the same minimum size as a `Vac` — 24 bytes on 64-bit platforms. This implementation manages to get that down to 16 bytes. For a large number of strings, this savings adds up. Not just in memory usage, but **cache utilization** as well.

The capabilities of the crate are purposely kept small. The main use for this type of element is for keys in other data structures, so the only traits defined for the type are those that are useful for that purpose.

| Default | Deref | PartialOrd | Ord  | PartialEq |  Eq  | Hash |
| :-----: | :---: | :--------: | :--: | :-------: | :--: | :--: |
|    ✔︎    |   ✔︎   |     ✔︎      |  ✔︎   |     ✔︎     |  ✔︎   |  ✔︎   |

Anything more is just an `as_str()` or `Deref` away.



## Limitations

Strings that are too large to store inline are stored on the heap as a `CString`, rather than using a `String` or `Vec` . As such, embedded NUL characters are not allowed in the strings that are to be stored. A  `std::ffi::NulError` will be returned if an attempt is made to construct an `sso::Storage` from an unsupported `str`.

The basic tradeoff inherent in the design is the use of a NUL sentinel value to compute the length of (long) strings vs. the inline space that would be required to explicitly store it.

Using this crate is implicitly stating that handling long strings “as slow as C does” will be fast enough. Or that short strings are so prevalent that the higher overhead for longer string will never come to dominate performance.



## Safety

This crate uses `unsafe` as it use a Rust [`union`](https://doc.rust-lang.org/reference/items/unions.html) internally to lower the overhead. However

1. The code is purposefully kept [small/simple](src/sso/mod.rs) to simplify manual auditing.
2. [Property testing](https://github.com/BurntSushi/quickcheck#readme) is done to ensure that it works on a large variety of strings.
3. Every test is run under [Miri](https://github.com/rust-lang/miri#readme) [on every push](https://github.com/bwoods/immutable-sso/actions) to help check the vanity of the `unsafe` code.

![](https://github.com/bwoods//immutable-sso/actions/workflows/miri.yml/badge.svg)



## Usage

This repo can be added to your `Cargo.toml` file directly.

```yaml
[dependencies.sso]
git = "https://github.com/bwoods/immutable-sso"
tag = "0.3.5"
```



## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE.md) and [LICENSE-MIT](LICENSE-MIT.md) for details.
