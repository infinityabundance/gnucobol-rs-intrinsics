# gnucobol-rs-intrinsics

An ergonomic `FUNCTION`-intrinsic API over the oracle-proven [`gnucobol-rs`](https://github.com/infinityabundance/gnucobol-rs) core.

Each function is a thin, value-faithful wrapper around a primitive already proven **byte/value-identical to
GnuCOBOL 3.2** by a sealed court (the `GNURUST.INTRINSIC.*` campaigns). This crate adds only the user-facing
shape — strings / `Decimal` in, strings / `Decimal` / integers out.

```rust
use gnucobol_rs_intrinsics as f;

assert_eq!(f::upper_case("abc"), "ABC");
assert_eq!(f::numval("123.45").unscaled_i128(), Some(12345)); // scale 2 -> 123.45
assert_eq!(f::modulo(-17, 5), 3);    // FUNCTION MOD  — divisor sign
assert_eq!(f::remainder(-17, 5), -2); // FUNCTION REM  — dividend sign
assert_eq!(f::ord(b'A'), 66);
```

Provides: `NUMVAL`, `NUMVAL-C`, `INTEGER` (FLOOR), `INTEGER-PART` (TRUNC), `MOD`, `REM`, `UPPER-CASE`,
`LOWER-CASE`, `REVERSE`, `ORD`, `CHAR`, `LENGTH`, and the date intrinsics (`INTEGER-OF-DATE` /
`DATE-OF-INTEGER` / `INTEGER-OF-DAY` / `DAY-OF-INTEGER`).

## Architecture
- `gnucobol-rs` (separate crate) — the oracle-proven semantic primitive layer.
- `gnucobol-rs-*` — faithful-port satellites; they depend on the core, the core does not depend on them.

## License
LGPL-3.0-or-later — a faithful derivative of GnuCOBOL/libcob (FSF copyright retained). See COPYING.LESSER (+ COPYING).
