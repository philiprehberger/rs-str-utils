# rs-str-utils

[![CI](https://github.com/philiprehberger/rs-str-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-str-utils/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-str-utils.svg)](https://crates.io/crates/philiprehberger-str-utils)
[![License](https://img.shields.io/github/license/philiprehberger/rs-str-utils)](LICENSE)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

String manipulation utilities — truncation, case conversion, padding, and whitespace operations

## Installation

```toml
[dependencies]
philiprehberger-str-utils = "0.1.0"
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

## License

MIT
