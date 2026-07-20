# Changelog

## Unreleased

### Added

- Numeric methods, checking for number types and `TryFrom<&Value>`
- Unwrapping methods which consume the `Value` (e.g. `into_seq(self) -> Option<Vec<Value>>`)

## 0.8.1 - 2026-07-16

### Added

- Documentation

## 0.8.0 - 2026-07-16

### Removed

- BREAKING: Drop support for rust < 1.85

### Changed

- BREAKING: Rename crate from serde-value to serde-valuable
- Upgrade ordered-float dependency to v5
- Set edition to 2024
- Exposed `crate::Map`, the map type used internally
- Move from travis CI to github actions

### Added

- Support for nostd
- Support for visiting enums
- Changelog
- `From` impls for built-in types, including arrays and some tuples
- `FromIterator` impls for sequences and maps
- BREAKING: 128-bit integers
- `as_{type}(_mut)` methods, like `serde_json`

## 0.7.0 - 2020-06-17
