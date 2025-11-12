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

use std::{error::Error, fmt, str::FromStr};

/// Brazilian federative units (UFE_SG).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Uf {
    AC,
    AL,
    AP,
    AM,
    BA,
    CE,
    DF,
    ES,
    GO,
    MA,
    MT,
    MS,
    MG,
    PA,
    PB,
    PR,
    PE,
    PI,
    RJ,
    RN,
    RS,
    RO,
    RR,
    SC,
    SP,
    SE,
    TO,
}

/// Parsing errors for `Uf`.
#[derive(Debug, PartialEq, Eq)]
pub enum UfParseError {
    /// Empty input after trimming.
    Empty,

    /// Input must have exactly length 2.
    WrongLength(usize),

    /// Code not recognized among official UF codes.
    InvalidCode(String),
}

impl fmt::Display for UfParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UfParseError::*;
        match self {
            Empty => write!(f, "UF code is empty"),
            WrongLength(n) => write!(f, "UF code must have length 2, got {n}"),
            InvalidCode(s) => write!(f, "invalid UF code: {}", s),
        }
    }
}

impl Error for UfParseError {}

impl FromStr for Uf {
    type Err = UfParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Uf::*;

        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(UfParseError::Empty);
        }

        if trimmed.len() != 2 {
            return Err(UfParseError::WrongLength(trimmed.len()));
        }

        let code = trimmed.to_ascii_uppercase();

        let uf = match code.as_str() {
            "AC" => AC,
            "AL" => AL,
            "AP" => AP,
            "AM" => AM,
            "BA" => BA,
            "CE" => CE,
            "DF" => DF,
            "ES" => ES,
            "GO" => GO,
            "MA" => MA,
            "MT" => MT,
            "MS" => MS,
            "MG" => MG,
            "PA" => PA,
            "PB" => PB,
            "PR" => PR,
            "PE" => PE,
            "PI" => PI,
            "RJ" => RJ,
            "RN" => RN,
            "RS" => RS,
            "RO" => RO,
            "RR" => RR,
            "SC" => SC,
            "SP" => SP,
            "SE" => SE,
            "TO" => TO,
            _ => return Err(UfParseError::InvalidCode(code)),
        };

        Ok(uf)
    }
}

impl fmt::Display for Uf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn uf_parses_valid() {
        assert_eq!(Uf::from_str("SP").unwrap(), Uf::SP);
        assert_eq!(Uf::from_str("RJ").unwrap(), Uf::RJ);
        assert_eq!(Uf::from_str("ba").unwrap(), Uf::BA);
    }

    #[test]
    fn uf_trims() {
        assert_eq!(Uf::from_str("  mg ").unwrap(), Uf::MG);
    }

    #[test]
    fn uf_rejects_empty() {
        let err = Uf::from_str("   ").unwrap_err();
        assert_eq!(err, UfParseError::Empty);
    }

    #[test]
    fn uf_rejects_wrong_length() {
        let err = Uf::from_str("S").unwrap_err();
        assert!(matches!(err, UfParseError::WrongLength(1)));

        let err = Uf::from_str("SPO").unwrap_err();
        assert!(matches!(err, UfParseError::WrongLength(3)));
    }

    #[test]
    fn uf_rejects_invalid() {
        let err = Uf::from_str("ZZ").unwrap_err();
        assert_eq!(err, UfParseError::InvalidCode("ZZ".into()));
    }

    #[test]
    fn uf_display_prints_abbreviation() {
        assert_eq!(Uf::SP.to_string(), "SP");
        assert_eq!(Uf::RR.to_string().to_ascii_uppercase(), "RR");
    }
}
