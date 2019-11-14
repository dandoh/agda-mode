# agda-mode

[![Crates.io](https://img.shields.io/crates/d/agda-mode.svg)][crates]
[![Crates.io](https://img.shields.io/crates/v/agda-mode.svg)][lib-rs]
[![Crates.io](https://img.shields.io/crates/l/agda-mode.svg)][crates]
[![docs.rs](https://docs.rs/agda-mode/badge.svg)][doc-rs]
[![Actions Status][ga-svg]][ga-url]
[![dep-svg]][dep-rs]

 [crates]: https://crates.io/crates/agda-mode
 [lib-rs]: https://lib.rs/agda-mode
 [doc-rs]: https://docs.rs/agda-mode
 [dep-rs]: https://deps.rs/repo/github/ice1000/agda-mode
 [dep-svg]: https://deps.rs/repo/github/ice1000/agda-mode/status.svg
 [ga-svg]: https://github.com/ice1000/agda-mode/workflows/build/badge.svg
 [ga-url]: https://github.com/ice1000/agda-mode/actions
 [agda-issue]: https://github.com/agda/agda/issues/4183

Accessing Agda's interaction mode via command line.
This library is created for [agda-tac](https://lib.rs/agda-tac).

This crate currently works only with nightly rust because
tokio-process 0.3 haven't reached a stable-compatible release.
Once they release a newer (even alpha) version,
this crate should work for stable rust as well.

Also, this crate will work with master-branch Agda until Agda 2.6.1 is released.
Tracking issue for the feature: [agda#4183][agda-issue].
