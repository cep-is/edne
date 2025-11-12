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

        match trimmed.to_ascii_uppercase().as_str() {
            "AC" => Ok(AC),
            "AL" => Ok(AL),
            "AP" => Ok(AP),
            "AM" => Ok(AM),
            "BA" => Ok(BA),
            "CE" => Ok(CE),
            "DF" => Ok(DF),
            "ES" => Ok(ES),
            "GO" => Ok(GO),
            "MA" => Ok(MA),
            "MT" => Ok(MT),
            "MS" => Ok(MS),
            "MG" => Ok(MG),
            "PA" => Ok(PA),
            "PB" => Ok(PB),
            "PR" => Ok(PR),
            "PE" => Ok(PE),
            "PI" => Ok(PI),
            "RJ" => Ok(RJ),
            "RN" => Ok(RN),
            "RS" => Ok(RS),
            "RO" => Ok(RO),
            "RR" => Ok(RR),
            "SC" => Ok(SC),
            "SP" => Ok(SP),
            "SE" => Ok(SE),
            "TO" => Ok(TO),
            _ => Err(UfParseError::InvalidCode(trimmed.to_string())),
        }
    }
}

impl fmt::Display for Uf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Uf {
    /// Returns the full Portuguese name of the state.
    pub fn full_name(&self) -> &'static str {
        use Uf::*;
        match self {
            AC => "Acre",
            AL => "Alagoas",
            AP => "Amapá",
            AM => "Amazonas",
            BA => "Bahia",
            CE => "Ceará",
            DF => "Distrito Federal",
            ES => "Espírito Santo",
            GO => "Goiás",
            MA => "Maranhão",
            MT => "Mato Grosso",
            MS => "Mato Grosso do Sul",
            MG => "Minas Gerais",
            PA => "Pará",
            PB => "Paraíba",
            PR => "Paraná",
            PE => "Pernambuco",
            PI => "Piauí",
            RJ => "Rio de Janeiro",
            RN => "Rio Grande do Norte",
            RS => "Rio Grande do Sul",
            RO => "Rondônia",
            RR => "Roraima",
            SC => "Santa Catarina",
            SP => "São Paulo",
            SE => "Sergipe",
            TO => "Tocantins",
        }
    }

    /// Returns an iterator over all `Uf` variants.
    ///
    /// Note: For arrays, `into_iter()` yields items by value, so no `.copied()` is needed.
    #[inline]
    pub fn iter() -> impl Iterator<Item = Uf> {
        [
            Uf::AC,
            Uf::AL,
            Uf::AP,
            Uf::AM,
            Uf::BA,
            Uf::CE,
            Uf::DF,
            Uf::ES,
            Uf::GO,
            Uf::MA,
            Uf::MT,
            Uf::MS,
            Uf::MG,
            Uf::PA,
            Uf::PB,
            Uf::PR,
            Uf::PE,
            Uf::PI,
            Uf::RJ,
            Uf::RN,
            Uf::RS,
            Uf::RO,
            Uf::RR,
            Uf::SC,
            Uf::SP,
            Uf::SE,
            Uf::TO,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Ensures that valid UF codes can be parsed correctly.
    #[test]
    fn uf_parses_valid() {
        assert_eq!(Uf::from_str("SP").unwrap(), Uf::SP);
        assert_eq!(Uf::from_str("rj").unwrap(), Uf::RJ);
        assert_eq!(Uf::from_str(" ba ").unwrap(), Uf::BA);
    }

    /// Ensures that parsing fails for empty strings and invalid codes.
    #[test]
    fn uf_parsing_errors() {
        assert!(matches!(Uf::from_str(""), Err(UfParseError::Empty)));
        assert!(matches!(
            Uf::from_str("S"),
            Err(UfParseError::WrongLength(1))
        ));
        assert!(matches!(
            Uf::from_str("XYZ"),
            Err(UfParseError::WrongLength(3))
        ));
        assert!(matches!(
            Uf::from_str("ZZ"),
            Err(UfParseError::InvalidCode(_))
        ));
    }

    /// Validates the display implementation.
    #[test]
    fn uf_display_abbreviation() {
        assert_eq!(Uf::SP.to_string(), "SP");
        assert_eq!(Uf::RR.to_string(), "RR");
    }

    /// Ensures that the full name mapping works correctly.
    #[test]
    fn uf_full_name_mapping() {
        assert_eq!(Uf::SP.full_name(), "São Paulo");
        assert_eq!(Uf::RJ.full_name(), "Rio de Janeiro");
        assert_eq!(Uf::DF.full_name(), "Distrito Federal");
    }

    /// Validates that iteration covers all 27 federative units with no duplicates.
    #[test]
    fn uf_iteration_covers_all() {
        let all: HashSet<_> = Uf::iter().collect();
        assert_eq!(all.len(), 27, "Expected 27 federative units");
        assert!(all.contains(&Uf::SP));
        assert!(all.contains(&Uf::AC));
        assert!(all.contains(&Uf::TO));
    }
}
