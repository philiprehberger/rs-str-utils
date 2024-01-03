//! String manipulation utilities — truncation, case conversion, padding, and whitespace operations.
//!
//! This crate provides a [`StrExt`] trait that extends `&str` and `String` with convenient
//! string manipulation methods.
//!
//! # Usage
//!
//! ```
//! use philiprehberger_str_utils::StrExt;
//!
//! assert_eq!("Hello, World!".truncate_ellipsis(8), "Hello...");
//! assert_eq!("hello world".to_camel_case(), "helloWorld");
//! assert_eq!("hi".pad_left(5, ' '), "   hi");
//! assert_eq!("  hello   world  ".squish(), "hello world");
//! ```

use unicode_width::UnicodeWidthStr;

/// Split a string into words by detecting boundaries at underscores, hyphens, spaces,
/// and camelCase transitions.
fn split_words(s: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();

    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    let mut i = 0;
    while i < len {
        let c = chars[i];

        // Delimiters: split and skip
        if c == '_' || c == '-' || c == ' ' {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            i += 1;
            continue;
        }

        if c.is_uppercase() {
            // Check if this starts a new word
            if !current.is_empty() {
                // Look ahead: if we have consecutive uppercase followed by lowercase,
                // e.g., "HTMLParser" at 'P', the previous uppercase run is one word.
                let prev_is_upper = i > 0 && chars[i - 1].is_uppercase();
                if prev_is_upper {
                    // We're in an uppercase run. If next char is lowercase, the current
                    // uppercase char starts a new word (e.g., "HTML|Parser").
                    if i + 1 < len && chars[i + 1].is_lowercase() {
                        // Split: everything before this char is one word
                        words.push(current.clone());
                        current.clear();
                        current.push(c);
                    } else {
                        // Continue the uppercase run
                        current.push(c);
                    }
                } else {
                    // Transition from lowercase to uppercase — new word
                    words.push(current.clone());
                    current.clear();
                    current.push(c);
                }
            } else {
                current.push(c);
            }
        } else {
            current.push(c);
        }

        i += 1;
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

/// Extension trait providing string manipulation methods.
///
/// Implemented for `&str` and `String`.
pub trait StrExt {
    /// Returns the string slice to operate on.
    fn as_str_ext(&self) -> &str;

    /// Truncate to `max_len` characters, appending "..." if truncated.
    ///
    /// Unicode-safe: operates on char boundaries.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("Hello, World!".truncate_ellipsis(8), "Hello...");
    /// assert_eq!("Short".truncate_ellipsis(10), "Short");
    /// ```
    fn truncate_ellipsis(&self, max_len: usize) -> String {
        self.truncate_with(max_len, "...")
    }

    /// Truncate to `max_len` characters, appending `suffix` if truncated.
    ///
    /// If the string is already within `max_len`, it is returned unchanged.
    /// The suffix is included in the `max_len` budget.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("Hello, World!".truncate_with(8, "~~"), "Hello,~~");
    /// ```
    fn truncate_with(&self, max_len: usize, suffix: &str) -> String {
        let s = self.as_str_ext();
        let char_count = s.chars().count();
        if char_count <= max_len {
            return s.to_string();
        }
        let suffix_len = suffix.chars().count();
        if max_len <= suffix_len {
            return suffix.chars().take(max_len).collect();
        }
        let keep = max_len - suffix_len;
        let mut result: String = s.chars().take(keep).collect();
        result.push_str(suffix);
        result
    }

    /// Convert to camelCase.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("hello world".to_camel_case(), "helloWorld");
    /// assert_eq!("foo_bar".to_camel_case(), "fooBar");
    /// ```
    fn to_camel_case(&self) -> String {
        let words = split_words(self.as_str_ext());
        let mut result = String::new();
        for (i, word) in words.iter().enumerate() {
            if i == 0 {
                result.push_str(&word.to_lowercase());
            } else {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    result.extend(first.to_uppercase());
                    result.push_str(&chars.as_str().to_lowercase());
                }
            }
        }
        result
    }

    /// Convert to PascalCase.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("hello world".to_pascal_case(), "HelloWorld");
    /// ```
    fn to_pascal_case(&self) -> String {
        let words = split_words(self.as_str_ext());
        let mut result = String::new();
        for word in &words {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.extend(first.to_uppercase());
                result.push_str(&chars.as_str().to_lowercase());
            }
        }
        result
    }

    /// Convert to snake_case.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("helloWorld".to_snake_case(), "hello_world");
    /// assert_eq!("Hello World".to_snake_case(), "hello_world");
    /// ```
    fn to_snake_case(&self) -> String {
        let words = split_words(self.as_str_ext());
        words
            .iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    /// Convert to kebab-case.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("helloWorld".to_kebab_case(), "hello-world");
    /// ```
    fn to_kebab_case(&self) -> String {
        let words = split_words(self.as_str_ext());
        words
            .iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Convert to SCREAMING_SNAKE_CASE.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("helloWorld".to_screaming_snake(), "HELLO_WORLD");
    /// ```
    fn to_screaming_snake(&self) -> String {
        let words = split_words(self.as_str_ext());
        words
            .iter()
            .map(|w| w.to_uppercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    /// Convert to Title Case.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("hello world".to_title_case(), "Hello World");
    /// ```
    fn to_title_case(&self) -> String {
        let words = split_words(self.as_str_ext());
        let mut parts = Vec::new();
        for word in &words {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                let mut titled = String::new();
                titled.extend(first.to_uppercase());
                titled.push_str(&chars.as_str().to_lowercase());
                parts.push(titled);
            }
        }
        parts.join(" ")
    }

    /// Left-pad the string to `width` using `fill`, based on Unicode display width.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("hi".pad_left(5, ' '), "   hi");
    /// ```
    fn pad_left(&self, width: usize, fill: char) -> String {
        let s = self.as_str_ext();
        let current_width = UnicodeWidthStr::width(s);
        if current_width >= width {
            return s.to_string();
        }
        let padding = width - current_width;
        let mut result = String::new();
        for _ in 0..padding {
            result.push(fill);
        }
        result.push_str(s);
        result
    }

    /// Right-pad the string to `width` using `fill`, based on Unicode display width.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("hi".pad_right(5, '.'), "hi...");
    /// ```
    fn pad_right(&self, width: usize, fill: char) -> String {
        let s = self.as_str_ext();
        let current_width = UnicodeWidthStr::width(s);
        if current_width >= width {
            return s.to_string();
        }
        let padding = width - current_width;
        let mut result = s.to_string();
        for _ in 0..padding {
            result.push(fill);
        }
        result
    }

    /// Center-pad the string to `width` using `fill`, based on Unicode display width.
    ///
    /// If the padding is odd, the extra character goes on the right.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("hi".pad_center(6, '-'), "--hi--");
    /// ```
    fn pad_center(&self, width: usize, fill: char) -> String {
        let s = self.as_str_ext();
        let current_width = UnicodeWidthStr::width(s);
        if current_width >= width {
            return s.to_string();
        }
        let total_padding = width - current_width;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;
        let mut result = String::new();
        for _ in 0..left_padding {
            result.push(fill);
        }
        result.push_str(s);
        for _ in 0..right_padding {
            result.push(fill);
        }
        result
    }

    /// Collapse all consecutive whitespace to a single space and trim.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("  hello   world  ".squish(), "hello world");
    /// ```
    fn squish(&self) -> String {
        let s = self.as_str_ext();
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Remove common leading whitespace from all non-empty lines.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// let text = "    hello\n    world";
    /// assert_eq!(text.dedent(), "hello\nworld");
    /// ```
    fn dedent(&self) -> String {
        let s = self.as_str_ext();
        let lines: Vec<&str> = s.lines().collect();

        // Find minimum indentation among non-empty lines
        let min_indent = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        lines
            .iter()
            .map(|line| {
                if line.len() >= min_indent {
                    &line[min_indent..]
                } else {
                    line.trim()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Prepend `prefix` to every line.
    ///
    /// ```
    /// use philiprehberger_str_utils::StrExt;
    /// assert_eq!("hello\nworld".indent("  "), "  hello\n  world");
    /// ```
    fn indent(&self, prefix: &str) -> String {
        let s = self.as_str_ext();
        s.lines()
            .map(|line| format!("{}{}", prefix, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl StrExt for str {
    fn as_str_ext(&self) -> &str {
        self
    }
}

impl StrExt for String {
    fn as_str_ext(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Truncation ---

    #[test]
    fn truncate_ellipsis_basic() {
        assert_eq!("Hello, World!".truncate_ellipsis(8), "Hello...");
    }

    #[test]
    fn truncate_ellipsis_no_truncation_needed() {
        assert_eq!("Short".truncate_ellipsis(10), "Short");
    }

    #[test]
    fn truncate_ellipsis_exact_length() {
        assert_eq!("Hello".truncate_ellipsis(5), "Hello");
    }

    #[test]
    fn truncate_ellipsis_empty_string() {
        assert_eq!("".truncate_ellipsis(5), "");
    }

    #[test]
    fn truncate_ellipsis_max_len_less_than_suffix() {
        assert_eq!("Hello".truncate_ellipsis(2), "..");
    }

    #[test]
    fn truncate_with_custom_suffix() {
        assert_eq!("Hello, World!".truncate_with(7, "~"), "Hello,~");
    }

    #[test]
    fn truncate_unicode_emoji() {
        // Emoji are single chars
        let s = "Hello \u{1F600} World";
        let result = s.truncate_ellipsis(9);
        // 9 chars: "Hello " (6) + emoji (1) + " W" would be 9, but we need room for "..."
        // keep = 9 - 3 = 6, so "Hello " + "..."
        assert_eq!(result, "Hello ...");
    }

    #[test]
    fn truncate_cjk() {
        let s = "\u{4F60}\u{597D}\u{4E16}\u{754C}"; // 4 CJK chars
        assert_eq!(s.truncate_ellipsis(10), s); // 4 chars <= 10
        assert_eq!(s.truncate_ellipsis(4), s); // exactly 4 chars
        assert_eq!(s.truncate_ellipsis(3), "..."); // 3 - 3 = 0 keep, just "..."
    }

    #[test]
    fn truncate_single_char() {
        assert_eq!("A".truncate_ellipsis(1), "A");
    }

    // --- Case conversion ---

    #[test]
    fn camel_case_from_spaces() {
        assert_eq!("hello world".to_camel_case(), "helloWorld");
    }

    #[test]
    fn camel_case_from_snake() {
        assert_eq!("foo_bar".to_camel_case(), "fooBar");
    }

    #[test]
    fn camel_case_from_kebab() {
        assert_eq!("foo-bar-baz".to_camel_case(), "fooBarBaz");
    }

    #[test]
    fn camel_case_empty() {
        assert_eq!("".to_camel_case(), "");
    }

    #[test]
    fn pascal_case_basic() {
        assert_eq!("hello world".to_pascal_case(), "HelloWorld");
        assert_eq!("foo_bar".to_pascal_case(), "FooBar");
    }

    #[test]
    fn snake_case_from_camel() {
        assert_eq!("helloWorld".to_snake_case(), "hello_world");
    }

    #[test]
    fn snake_case_from_spaces() {
        assert_eq!("Hello World".to_snake_case(), "hello_world");
    }

    #[test]
    fn kebab_case_from_camel() {
        assert_eq!("helloWorld".to_kebab_case(), "hello-world");
    }

    #[test]
    fn kebab_case_from_spaces() {
        assert_eq!("hello world".to_kebab_case(), "hello-world");
    }

    #[test]
    fn screaming_snake_basic() {
        assert_eq!("helloWorld".to_screaming_snake(), "HELLO_WORLD");
        assert_eq!("foo_bar".to_screaming_snake(), "FOO_BAR");
    }

    #[test]
    fn title_case_basic() {
        assert_eq!("hello world".to_title_case(), "Hello World");
        assert_eq!("foo_bar".to_title_case(), "Foo Bar");
    }

    #[test]
    fn title_case_single_word() {
        assert_eq!("hello".to_title_case(), "Hello");
    }

    // --- Word splitting edge cases ---

    #[test]
    fn split_words_html_parser() {
        let words = split_words("HTMLParser");
        assert_eq!(
            words.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>(),
            vec!["html", "parser"]
        );
    }

    #[test]
    fn split_words_get_https_response() {
        let words = split_words("getHTTPSResponse");
        assert_eq!(
            words.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>(),
            vec!["get", "https", "response"]
        );
    }

    #[test]
    fn split_words_all_caps() {
        let words = split_words("HTTP");
        assert_eq!(words, vec!["HTTP"]);
    }

    #[test]
    fn split_words_mixed_delimiters() {
        let words = split_words("foo_bar-baz qux");
        assert_eq!(words, vec!["foo", "bar", "baz", "qux"]);
    }

    // --- Padding ---

    #[test]
    fn pad_left_basic() {
        assert_eq!("hi".pad_left(5, ' '), "   hi");
    }

    #[test]
    fn pad_left_no_padding_needed() {
        assert_eq!("hello".pad_left(3, ' '), "hello");
    }

    #[test]
    fn pad_right_basic() {
        assert_eq!("hi".pad_right(5, '.'), "hi...");
    }

    #[test]
    fn pad_center_even() {
        assert_eq!("hi".pad_center(6, '-'), "--hi--");
    }

    #[test]
    fn pad_center_odd() {
        assert_eq!("hi".pad_center(7, '-'), "--hi---");
    }

    #[test]
    fn pad_empty_string() {
        assert_eq!("".pad_left(3, '*'), "***");
        assert_eq!("".pad_right(3, '*'), "***");
        assert_eq!("".pad_center(3, '*'), "***");
    }

    #[test]
    fn pad_cjk_width() {
        // CJK characters take 2 columns each
        let s = "\u{4F60}\u{597D}"; // 2 chars, 4 columns wide
        assert_eq!(s.pad_right(6, '.'), "\u{4F60}\u{597D}..");
        assert_eq!(s.pad_left(6, '.'), "..\u{4F60}\u{597D}");
    }

    // --- Whitespace utilities ---

    #[test]
    fn squish_basic() {
        assert_eq!("  hello   world  ".squish(), "hello world");
    }

    #[test]
    fn squish_tabs_and_newlines() {
        assert_eq!("hello\t\t  world\n\nfoo".squish(), "hello world foo");
    }

    #[test]
    fn squish_empty() {
        assert_eq!("".squish(), "");
    }

    #[test]
    fn squish_only_whitespace() {
        assert_eq!("   \t\n  ".squish(), "");
    }

    #[test]
    fn dedent_basic() {
        let text = "    hello\n    world";
        assert_eq!(text.dedent(), "hello\nworld");
    }

    #[test]
    fn dedent_mixed_indent() {
        let text = "    hello\n      world";
        assert_eq!(text.dedent(), "hello\n  world");
    }

    #[test]
    fn dedent_with_empty_lines() {
        let text = "    hello\n\n    world";
        assert_eq!(text.dedent(), "hello\n\nworld");
    }

    #[test]
    fn dedent_no_indent() {
        let text = "hello\nworld";
        assert_eq!(text.dedent(), "hello\nworld");
    }

    #[test]
    fn indent_basic() {
        assert_eq!("hello\nworld".indent("  "), "  hello\n  world");
    }

    #[test]
    fn indent_single_line() {
        assert_eq!("hello".indent(">>> "), ">>> hello");
    }

    #[test]
    fn indent_empty_string() {
        // "".lines() yields no items, so join produces ""
        assert_eq!("".indent("  "), "");
    }

    // --- String type support ---

    #[test]
    fn works_with_string_type() {
        let s = String::from("hello world");
        assert_eq!(s.to_camel_case(), "helloWorld");
        assert_eq!(s.squish(), "hello world");
        assert_eq!(s.pad_left(15, '.'), "....hello world");
    }
}
