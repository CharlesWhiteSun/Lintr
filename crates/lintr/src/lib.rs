//! Lintr — Rust-based Python static analyzer.
//!
//! # Public API
//!
//! ```rust,ignore
//! use lintr::{lint, Config, Diagnostic};
//!
//! let diagnostics = lint("x == None\n", &Config::default());
//! ```
//!
//! Phase 5 will expose the full `lint()` and `lint_file()` API.
