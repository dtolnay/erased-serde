Erased Serde
============

[![Build Status](https://api.travis-ci.org/dtolnay/erased-serde.svg?branch=master)](https://travis-ci.org/dtolnay/erased-serde)
[![Latest Version](https://img.shields.io/crates/v/erased-serde.svg)](https://crates.io/crates/erased-serde)

This crate provides type-erased versions of Serde's `Serialize` and `Serializer`
traits that can be used as [trait
objects](https://doc.rust-lang.org/book/trait-objects.html).

- [`erased_serde::Serialize`](https://docs.serde.rs/erased_serde/trait.Serialize.html)
- [`erased_serde::Serializer`](https://docs.serde.rs/erased_serde/trait.Serializer.html)

```rust
extern crate erased_serde;
extern crate serde_json;

use erased_serde::{Serialize, Serializer};

fn main() {
    // This is a type-erased trait object.
    let obj: &Serialize = &vec!["a", "b"];

    let mut buf = Vec::new();

    {
        let mut ser = serde_json::Serializer::new(&mut buf);

        // This is a type-erased trait object.
        let ser: &mut Serializer = &mut ser;

        // Both `obj` and `ser` are trait objects.
        obj.erased_serialize(ser).unwrap();
    }

    assert_eq!(&buf, br#"["a","b"]"#);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
