# rs-str-utils

[![CI](https://github.com/philiprehberger/rs-str-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-str-utils/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-str-utils.svg)](https://crates.io/crates/philiprehberger-str-utils)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-str-utils)](https://github.com/philiprehberger/rs-str-utils/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-str-utils)](https://github.com/philiprehberger/rs-str-utils/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-str-utils)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-str-utils/bug)](https://github.com/philiprehberger/rs-str-utils/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-str-utils/enhancement)](https://github.com/philiprehberger/rs-str-utils/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

String manipulation utilities — truncation, case conversion, padding, and whitespace operations

## Installation

```toml
[dependencies]
philiprehberger-str-utils = "0.1.1"
```

## Usage

```rust
use philiprehberger_str_utils::StrExt;

// Truncation
assert_eq!("Hello, World!".truncate_ellipsis(8), "Hello...");

// Case conversion
assert_eq!("hello world".to_camel_case(), "helloWorld");
assert_eq!("helloWorld".to_snake_case(), "hello_world");
assert_eq!("foo_bar".to_pascal_case(), "FooBar");
assert_eq!("hello world".to_kebab_case(), "hello-world");
assert_eq!("hello world".to_title_case(), "Hello World");

// Padding
assert_eq!("hi".pad_left(5, ' '), "   hi");
assert_eq!("hi".pad_right(5, '.'), "hi...");
assert_eq!("hi".pad_center(6, '-'), "--hi--");

// Whitespace
assert_eq!("  hello   world  ".squish(), "hello world");
```

## API

| Function | Description |
|----------|-------------|
| `truncate_ellipsis(max)` | Truncate with "..." suffix |
| `truncate_with(max, suffix)` | Truncate with custom suffix |
| `to_camel_case()` | Convert to camelCase |
| `to_pascal_case()` | Convert to PascalCase |
| `to_snake_case()` | Convert to snake_case |
| `to_kebab_case()` | Convert to kebab-case |
| `to_screaming_snake()` | Convert to SCREAMING_SNAKE_CASE |
| `to_title_case()` | Convert to Title Case |
| `pad_left(width, fill)` | Left-pad to width |
| `pad_right(width, fill)` | Right-pad to width |
| `pad_center(width, fill)` | Center-pad to width |
| `squish()` | Collapse whitespace |
| `dedent()` | Remove common indentation |
| `indent(prefix)` | Add prefix to each line |

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
