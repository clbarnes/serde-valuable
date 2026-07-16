# serde-valuable

> serde-valuable is a maintained fork of [arcnmx](https://github.com/arcnmx)'s [serde-value](https://crates.io/crates/serde-value)

[![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

`serde-valuable` provides a way to capture serialization value trees for later processing.

[release-badge]: https://img.shields.io/crates/v/serde-valuable.svg?style=flat-square
[cargo]: https://crates.io/crates/serde-valuable
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://docs.rs/serde-valuable
[license-badge]: https://img.shields.io/badge/license-MIT-lightgray.svg?style=flat-square
[license]: https://github.com/clbarnes/serde-valuable/blob/master/COPYING

## Alternatives

- [serde-content](https://crates.io/crates/serde-content) sticks closer to the internal model used by serde

## PRs from original repo

- [x] Support for nostd: <https://github.com/arcnmx/serde-value/pull/43>
- Bumped ordered-float to v5, superceding:
  - [x] Upgrade to ordered-float v3 <https://github.com/arcnmx/serde-value/pull/35>
  - [x] Upgrade ordered-float to version 4 <https://github.com/arcnmx/serde-value/pull/42>
- [x] Support visiting enums <https://github.com/arcnmx/serde-value/pull/38>
- [ ] Added support for feature = preserve_order <https://github.com/arcnmx/serde-value/pull/21>

## LLM usage statement

Usage of LLMs in the development of this package is restricted to writing small macros and repetitive, minor tasks which are thoroughly reviewed.
