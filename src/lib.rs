#![forbid(unsafe_code)]
//! # gnucobol-rs-intrinsics
//!
//! An ergonomic `FUNCTION`-intrinsic API over the oracle-proven [`gnucobol-rs`](https://crates.io/crates/gnucobol-rs)
//! core primitives. Each function here is a thin, value-faithful wrapper around a primitive that was already
//! proven **byte/value-identical to GnuCOBOL 3.2** by a sealed court (the `GNURUST.INTRINSIC.*` campaigns);
//! this crate adds only the user-facing shape (strings/`Decimal` in, strings/`Decimal`/integers out).
//!
//! Faithful-port satellite of the gnucobol-rs ecosystem. LGPL-3.0-or-later (faithful derivative of
//! GnuCOBOL/libcob; FSF copyright retained). Ecosystem rule: gnucobol-rs-* depend on the gnucobol-rs core;
//! the core does not depend on the satellites.
//!
//! ```
//! use gnucobol_rs_intrinsics as f;
//! assert_eq!(f::upper_case("abc"), "ABC");
//! assert_eq!(f::modulo(-17, 5), 3);   // FUNCTION MOD: divisor sign
//! assert_eq!(f::remainder(-17, 5), -2); // FUNCTION REM: dividend sign
//! assert_eq!(f::ord(b'A'), 66);
//! ```

use gnucobol_rs::intrinsic as core;
use gnucobol_rs::{Decimal, PicError, Usage};

fn numval_to_decimal(nv: core::Numval) -> Decimal {
    let digits: Vec<u8> = if nv.scaled == 0 {
        vec![0]
    } else {
        nv.scaled.to_string().bytes().map(|b| b - b'0').collect()
    };
    Decimal { negative: nv.negative, digits, scale: nv.scale as i16 }
}

/// `FUNCTION NUMVAL(s)` — parse the narrow admitted numeric string form (sign, spaces, `CR`/`DB`,
/// decimal point) into a [`Decimal`].
pub fn numval(s: &str) -> Decimal {
    numval_to_decimal(core::intrinsic_numval(s))
}

/// `FUNCTION NUMVAL-C(s)` — like [`numval`] but also strips a leading currency `$` and thousands commas.
pub fn numval_c(s: &str) -> Decimal {
    numval_to_decimal(core::intrinsic_numval_c(s))
}

/// `FUNCTION INTEGER(x)` — the greatest integer not greater than `x` (FLOOR). `None` if `x` has no
/// representable unscaled magnitude.
pub fn integer(value: &Decimal) -> Option<i128> {
    value
        .unscaled_i128()
        .map(|m| core::intrinsic_integer(m, value.scale.max(0) as u32))
}

/// `FUNCTION INTEGER-PART(x)` — the integer part of `x` toward zero (TRUNCATE).
pub fn integer_part(value: &Decimal) -> Option<i128> {
    value
        .unscaled_i128()
        .map(|m| core::intrinsic_integer_part(m, value.scale.max(0) as u32))
}

/// `FUNCTION MOD(a, b)` — modulo carrying the **divisor's** sign (mathematical modulo).
pub fn modulo(a: i128, b: i128) -> i128 {
    core::intrinsic_mod(a, b)
}

/// `FUNCTION REM(a, b)` — remainder carrying the **dividend's** sign (C-style remainder).
pub fn remainder(a: i128, b: i128) -> i128 {
    core::intrinsic_rem(a, b)
}

/// `FUNCTION UPPER-CASE(s)` — ASCII upper-case fold.
pub fn upper_case(s: &str) -> String {
    String::from_utf8_lossy(&core::intrinsic_upper_case(s.as_bytes())).into_owned()
}

/// `FUNCTION LOWER-CASE(s)` — ASCII lower-case fold.
pub fn lower_case(s: &str) -> String {
    String::from_utf8_lossy(&core::intrinsic_lower_case(s.as_bytes())).into_owned()
}

/// `FUNCTION REVERSE(s)` — byte reversal.
pub fn reverse(s: &str) -> String {
    String::from_utf8_lossy(&core::intrinsic_reverse(s.as_bytes())).into_owned()
}

/// `FUNCTION ORD(c)` — 1-based position in the (ASCII) collating sequence; `ORD('A') == 66`.
pub fn ord(c: u8) -> u32 {
    core::intrinsic_ord(c)
}

/// `FUNCTION CHAR(n)` — the inverse of [`ord`].
pub fn char(n: u32) -> u8 {
    core::intrinsic_char(n)
}

/// `FUNCTION LENGTH(field)` — the storage byte length of a PICTURE under a [`Usage`].
pub fn length(pic: &str, usage: Usage) -> Result<usize, PicError> {
    core::intrinsic_length(pic, usage)
}

/// `FUNCTION INTEGER-OF-DATE(YYYYMMDD)`.
pub fn integer_of_date(yyyymmdd: u32) -> i64 {
    core::intrinsic_integer_of_date(yyyymmdd)
}

/// `FUNCTION DATE-OF-INTEGER(n)` — the inverse of [`integer_of_date`].
pub fn date_of_integer(n: i64) -> u32 {
    core::intrinsic_date_of_integer(n)
}

/// `FUNCTION INTEGER-OF-DAY(YYYYDDD)`.
pub fn integer_of_day(yyyyddd: u32) -> i64 {
    core::intrinsic_integer_of_day(yyyyddd)
}

/// `FUNCTION DAY-OF-INTEGER(n)` — the inverse of [`integer_of_day`].
pub fn day_of_integer(n: i64) -> u32 {
    core::intrinsic_day_of_integer(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gnucobol_rs::Decimal;

    fn dec(neg: bool, ds: &[u8], sc: i16) -> Decimal {
        Decimal { negative: neg, digits: ds.to_vec(), scale: sc }
    }

    #[test]
    fn numeric_intrinsics_match_the_sealed_courts() {
        assert_eq!(numval("123.45"), dec(false, &[1, 2, 3, 4, 5], 2)); // GNURUST.INTRINSIC.NUMVAL.1
        assert_eq!(numval_c("$1,234.56"), dec(false, &[1, 2, 3, 4, 5, 6], 2)); // NUMVAL-C
        // INTEGER (FLOOR) vs INTEGER-PART (TRUNC) on 3.7 / -3.7 — GNURUST.INTRINSIC.INTEGER.1
        assert_eq!(integer(&dec(false, &[3, 7], 1)), Some(3));
        assert_eq!(integer(&dec(true, &[3, 7], 1)), Some(-4));
        assert_eq!(integer_part(&dec(true, &[3, 7], 1)), Some(-3));
        // MOD divisor-sign vs REM dividend-sign — GNURUST.INTRINSIC.MOD-REM.1
        assert_eq!(modulo(17, 5), 2);
        assert_eq!(modulo(-17, 5), 3);
        assert_eq!(remainder(17, 5), 2);
        assert_eq!(remainder(-17, 5), -2);
    }

    #[test]
    fn string_and_char_intrinsics() {
        assert_eq!(upper_case("abc"), "ABC");
        assert_eq!(lower_case("ABC"), "abc");
        assert_eq!(reverse("abcd"), "dcba");
        assert_eq!(ord(b'A'), 66);
        assert_eq!(char(66), b'A');
    }

    #[test]
    fn length_and_dates() {
        assert_eq!(length("X(5)", Usage::Display).unwrap(), 5);
        let n = integer_of_date(20200101);
        assert_eq!(date_of_integer(n), 20200101);
    }
}
