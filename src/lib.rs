#![no_std]
#![forbid(unsafe_code)]
//! # gnucobol-rs-intrinsics
//!
//! gnucobol-rs intrinsic functions: pure stateless numeric/string/date intrinsics.
//!
//! A faithful-port satellite of the **gnucobol-rs** ecosystem (an oracle-first Rust compatibility court
//! for GnuCOBOL 3.2). Ports: intrinsic functions (libcob/intrinsic.c). Intended profile: no_std / no_alloc / no_unsafe.
//!
//! LICENSE: LGPL-3.0-or-later (faithful derivative of GnuCOBOL/libcob; FSF copyright retained). See
//! COPYING.LESSER (+ COPYING). Ecosystem rule: gnucobol-rs-* depend on the gnucobol-rs core; the core does
//! not depend on the satellites; kobold-* (Apache-2.0, separate) is the forensic-intelligence layer above.
//!
//! Status: SCAFFOLD. Implementation follows the split/planning pass, statement-by-statement against the
//! admitted GnuCOBOL 3.2 oracle (byte-exact, fail-closed), with the existing court/receipt discipline.

/// Crate scaffold marker; replace with the real public API as the implementation lands.
pub const GNUCOBOL_RS_SATELLITE: &str = "gnucobol-rs-intrinsics";
