# rolodex
  **[WIP]** A forgivable VCard parser for rust based on [RFC6350](https://tools.ietf.org/html/rfc6350).
### Crate features
All library features are enabled by default
  - `serde` serde support
  - `typed-builder` typed builder support
### Example

```rust
use rolodex::{VCard, Parse};

const DATA: &'static str = "BEGIN:VCARD
VERSION:4.0
N:Gump;Forrest;;Mr.;
FN:Forrest Gump
ORG:Bubba Gump Shrimp Co.
END:VCARD";

let vcard: VCard<'static> = Parse::parse(DATA)?;
```
### Alternatives

  #### [vcard](https://docs.rs/vcard/0.4.8/vcard/)
  This library is great but i require a vcard parser that hopefully will not fail
  even when receiving malformed input, and the way the library is designed doesn't
  facilitate this.
