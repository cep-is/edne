//
// Copyright (c) 2025 murilo ijanc' <murilo@ijanc.org>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

use std::{error::Error, fmt};

/// Field separator used in eDONE files.
pub const FIELD_SEPARATOR: char = '@';

/// Errors that can occur during eDONE parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Failed to decode ISO-8859-1 bytes.
    EncodingError(String),
    /// Expected a specific number of fields but got a different count.
    FieldCount { expected: usize, got: usize, line_number: usize },
    /// Field is empty but was required.
    EmptyField { field_name: &'static str, line_number: usize },
    /// Failed to parse a numeric field.
    InvalidNumber {
        field_name: &'static str,
        value: String,
        line_number: usize,
    },
    /// Field value is invalid according to domain rules.
    InvalidValue {
        field_name: &'static str,
        value: String,
        reason: String,
        line_number: usize,
    },
    /// Generic parsing error with context.
    ParseFailed { message: String, line_number: usize },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncodingError(msg) => write!(f, "encoding error: {}", msg),
            Self::FieldCount { expected, got, line_number } => write!(
                f,
                "line {}: expected {} fields, got {}",
                line_number, expected, got
            ),
            Self::EmptyField { field_name, line_number } => write!(
                f,
                "line {}: field '{}' is empty",
                line_number, field_name
            ),
            Self::InvalidNumber { field_name, value, line_number } => write!(
                f,
                "line {}: field '{}' has invalid number: '{}'",
                line_number, field_name, value
            ),
            Self::InvalidValue { field_name, value, reason, line_number } => {
                write!(
                    f,
                    "line {}: field '{}' has invalid value '{}': {}",
                    line_number, field_name, value, reason
                )
            }
            Self::ParseFailed { message, line_number } => {
                write!(f, "line {}: {}", line_number, message)
            }
        }
    }
}

impl Error for ParseError {}

/// Generic parser for eDONE text files.
///
/// This parser handles the common structure of eDONE files:
/// - ISO-8859-1 encoding
/// - One record per line
/// - Fields separated by '@'
/// - Optional trailing separator
pub struct EdneParser {
    content: String,
    separator: char,
}

impl EdneParser {
    /// Creates a new parser from ISO-8859-1 encoded bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Raw bytes in ISO-8859-1 encoding
    ///
    /// # Errors
    ///
    /// Returns `ParseError::EncodingError` if bytes cannot be decoded.
    pub fn from_iso8859_1(bytes: &[u8]) -> Result<Self, ParseError> {
        let content = Self::decode_iso8859_1(bytes)?;
        Ok(Self { content, separator: FIELD_SEPARATOR })
    }

    /// Creates a new parser from UTF-8 string (for testing).
    pub fn from_utf8(content: String) -> Self {
        Self { content, separator: FIELD_SEPARATOR }
    }

    /// Decodes ISO-8859-1 bytes to UTF-8 string.
    ///
    /// ISO-8859-1 is a single-byte encoding where each byte maps directly
    /// to a Unicode code point in the range 0x00-0xFF.
    fn decode_iso8859_1(bytes: &[u8]) -> Result<String, ParseError> {
        let mut result = String::with_capacity(bytes.len());
        for &byte in bytes {
            result.push(byte as char);
        }
        Ok(result)
    }

    /// Returns an iterator over non-empty lines.
    pub fn lines(&self) -> impl Iterator<Item = (usize, &str)> {
        self.content
            .lines()
            .enumerate()
            .map(|(idx, line)| (idx + 1, line))
            .filter(|(_, line)| !line.trim().is_empty())
    }

    /// Parses a single line into fields.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to parse
    ///
    /// # Returns
    ///
    /// A vector of field strings. Empty fields are preserved as empty strings.
    /// Trailing separators are handled correctly (they create an empty last field).
    pub fn parse_line<'a>(&self, line: &'a str) -> Vec<&'a str> {
        line.split(self.separator).collect()
    }

    /// Parses a line and validates the field count.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to parse
    /// * `expected_count` - Expected number of fields
    /// * `line_number` - Line number for error reporting
    ///
    /// # Errors
    ///
    /// Returns `ParseError::FieldCount` if the number of fields doesn't match.
    pub fn parse_line_checked<'a>(
        &self,
        line: &'a str,
        expected_count: usize,
        line_number: usize,
    ) -> Result<Vec<&'a str>, ParseError> {
        let fields = self.parse_line(line);
        if fields.len() != expected_count {
            return Err(ParseError::FieldCount {
                expected: expected_count,
                got: fields.len(),
                line_number,
            });
        }
        Ok(fields)
    }

    /// Extracts a required field from the fields array.
    ///
    /// # Errors
    ///
    /// Returns `ParseError::EmptyField` if the field is empty.
    pub fn required_field(
        field: &str,
        field_name: &'static str,
        line_number: usize,
    ) -> Result<String, ParseError> {
        if field.trim().is_empty() {
            return Err(ParseError::EmptyField { field_name, line_number });
        }
        Ok(field.to_string())
    }

    /// Extracts an optional field from the fields array.
    ///
    /// Returns `None` if the field is empty, `Some(String)` otherwise.
    pub fn optional_field(field: &str) -> Option<String> {
        if field.trim().is_empty() { None } else { Some(field.to_string()) }
    }

    /// Parses a required numeric field.
    ///
    /// # Errors
    ///
    /// Returns `ParseError::InvalidNumber` if parsing fails.
    pub fn parse_number<T>(
        field: &str,
        field_name: &'static str,
        line_number: usize,
    ) -> Result<T, ParseError>
    where
        T: std::str::FromStr,
    {
        field.trim().parse::<T>().map_err(|_| ParseError::InvalidNumber {
            field_name,
            value: field.to_string(),
            line_number,
        })
    }

    /// Parses an optional numeric field.
    ///
    /// # Errors
    ///
    /// Returns `ParseError::InvalidNumber` if the field is non-empty but parsing fails.
    pub fn parse_optional_number<T>(
        field: &str,
        field_name: &'static str,
        line_number: usize,
    ) -> Result<Option<T>, ParseError>
    where
        T: std::str::FromStr,
    {
        if field.trim().is_empty() {
            return Ok(None);
        }
        Self::parse_number(field, field_name, line_number).map(Some)
    }

    /// Returns the full content as a string slice.
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_iso8859_1_basic() {
        let bytes = b"Hello World";
        let result = EdneParser::decode_iso8859_1(bytes).unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn decode_iso8859_1_with_accents() {
        // "São Paulo" in ISO-8859-1
        let bytes = &[0x53, 0xE3, 0x6F, 0x20, 0x50, 0x61, 0x75, 0x6C, 0x6F];
        let result = EdneParser::decode_iso8859_1(bytes).unwrap();
        assert_eq!(result, "São Paulo");
    }

    #[test]
    fn parse_line_basic() {
        let parser = EdneParser::from_utf8("field1@field2@field3".to_string());
        let fields = parser.parse_line("field1@field2@field3");
        assert_eq!(fields, vec!["field1", "field2", "field3"]);
    }

    #[test]
    fn parse_line_with_trailing_separator() {
        let parser = EdneParser::from_utf8("field1@field2@".to_string());
        let fields = parser.parse_line("field1@field2@");
        assert_eq!(fields, vec!["field1", "field2", ""]);
    }

    #[test]
    fn parse_line_with_empty_fields() {
        let parser = EdneParser::from_utf8("field1@@field3".to_string());
        let fields = parser.parse_line("field1@@field3");
        assert_eq!(fields, vec!["field1", "", "field3"]);
    }

    #[test]
    fn parse_line_checked_success() {
        let parser = EdneParser::from_utf8("a@b@c".to_string());
        let result = parser.parse_line_checked("a@b@c", 3, 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["a", "b", "c"]);
    }

    #[test]
    fn parse_line_checked_wrong_count() {
        let parser = EdneParser::from_utf8("a@b".to_string());
        let result = parser.parse_line_checked("a@b", 3, 1);
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::FieldCount { expected, got, line_number } => {
                assert_eq!(expected, 3);
                assert_eq!(got, 2);
                assert_eq!(line_number, 1);
            }
            _ => panic!("Expected FieldCount error"),
        }
    }

    #[test]
    fn required_field_success() {
        let result = EdneParser::required_field("value", "test_field", 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "value");
    }

    #[test]
    fn required_field_empty() {
        let result = EdneParser::required_field("  ", "test_field", 1);
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::EmptyField { field_name, line_number } => {
                assert_eq!(field_name, "test_field");
                assert_eq!(line_number, 1);
            }
            _ => panic!("Expected EmptyField error"),
        }
    }

    #[test]
    fn optional_field_with_value() {
        let result = EdneParser::optional_field("value");
        assert_eq!(result, Some("value".to_string()));
    }

    #[test]
    fn optional_field_empty() {
        let result = EdneParser::optional_field("  ");
        assert_eq!(result, None);
    }

    #[test]
    fn parse_number_success() {
        let result = EdneParser::parse_number::<u32>("12345", "id", 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12345);
    }

    #[test]
    fn parse_number_invalid() {
        let result = EdneParser::parse_number::<u32>("abc", "id", 1);
        assert!(result.is_err());
    }

    #[test]
    fn parse_optional_number_with_value() {
        let result =
            EdneParser::parse_optional_number::<u32>("12345", "id", 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(12345));
    }

    #[test]
    fn parse_optional_number_empty() {
        let result = EdneParser::parse_optional_number::<u32>("", "id", 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn lines_iterator_skips_empty() {
        let content = "line1\n\nline2\n  \nline3".to_string();
        let parser = EdneParser::from_utf8(content);
        let lines: Vec<_> = parser.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], (1, "line1"));
        assert_eq!(lines[1], (3, "line2"));
        assert_eq!(lines[2], (5, "line3"));
    }
}
