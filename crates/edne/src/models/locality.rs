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

use crate::models::Uf;

/// Unique identifier for a locality.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalityId(u32);

impl LocalityId {
    /// Creates a new `LocalityId`.
    ///
    /// # Arguments
    ///
    /// * `id` - The numeric identifier
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the inner value.
    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for LocalityId {
    type Error = LocalityIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(LocalityIdError::Zero);
        }
        Ok(Self(value))
    }
}

impl FromStr for LocalityId {
    type Err = LocalityIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .parse::<u32>()
            .map_err(|_| LocalityIdError::InvalidFormat(s.to_string()))?;
        Self::try_from(value)
    }
}

impl fmt::Display for LocalityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors when parsing or creating a `LocalityId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalityIdError {
    /// ID cannot be zero.
    Zero,
    /// String is not a valid number.
    InvalidFormat(String),
}

impl fmt::Display for LocalityIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "locality ID cannot be zero"),
            Self::InvalidFormat(s) => {
                write!(f, "invalid locality ID format: '{}'", s)
            }
        }
    }
}

impl Error for LocalityIdError {}

/// Locality situation status.
///
/// Indicates the coding level of the locality:
/// - `NotCoded`: Locality not coded at street level
/// - `Coded`: Locality coded at street level
/// - `DistrictOrVillage`: District or village inserted in street-level coding
/// - `CodingInProgress`: Locality in street-level coding phase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LocalitySituation {
    /// Locality not coded at street level (LOC_IN_SIT = 0).
    NotCoded,
    /// Locality coded at street level (LOC_IN_SIT = 1).
    Coded,
    /// District or village in street-level coding (LOC_IN_SIT = 2).
    DistrictOrVillage,
    /// Locality in coding phase (LOC_IN_SIT = 3).
    CodingInProgress,
}

impl FromStr for LocalitySituation {
    type Err = LocalitySituationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "0" => Ok(Self::NotCoded),
            "1" => Ok(Self::Coded),
            "2" => Ok(Self::DistrictOrVillage),
            "3" => Ok(Self::CodingInProgress),
            other => {
                Err(LocalitySituationError::InvalidCode(other.to_string()))
            }
        }
    }
}

impl fmt::Display for LocalitySituation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::NotCoded => "0",
            Self::Coded => "1",
            Self::DistrictOrVillage => "2",
            Self::CodingInProgress => "3",
        };
        write!(f, "{}", code)
    }
}

/// Errors when parsing `LocalitySituation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalitySituationError {
    /// Invalid situation code.
    InvalidCode(String),
}

impl fmt::Display for LocalitySituationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCode(s) => {
                write!(f, "invalid locality situation code: '{}'", s)
            }
        }
    }
}

impl Error for LocalitySituationError {}

/// Type of locality.
///
/// - `District`: A district (D)
/// - `Municipality`: A municipality (M)
/// - `Village`: A village or settlement (P - Povoado)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LocalityType {
    /// District (LOC_IN_TIPO_LOC = D).
    District,
    /// Municipality (LOC_IN_TIPO_LOC = M).
    Municipality,
    /// Village/Settlement (LOC_IN_TIPO_LOC = P).
    Village,
}

impl FromStr for LocalityType {
    type Err = LocalityTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "D" => Ok(Self::District),
            "M" => Ok(Self::Municipality),
            "P" => Ok(Self::Village),
            other => Err(LocalityTypeError::InvalidCode(other.to_string())),
        }
    }
}

impl fmt::Display for LocalityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::District => "D",
            Self::Municipality => "M",
            Self::Village => "P",
        };
        write!(f, "{}", code)
    }
}

/// Errors when parsing `LocalityType`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalityTypeError {
    /// Invalid locality type code.
    InvalidCode(String),
}

impl fmt::Display for LocalityTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCode(s) => {
                write!(f, "invalid locality type code: '{}'", s)
            }
        }
    }
}

impl Error for LocalityTypeError {}

/// Represents a Brazilian locality from the eDNE database.
///
/// A locality can be a municipality, district, or village with associated
/// postal code information and geographic data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locality {
    /// Unique identifier for the locality (LOC_NU).
    pub id: LocalityId,
    /// Federative unit abbreviation (UFE_SG).
    pub uf: Uf,
    /// Name of the locality (LOC_NO).
    pub name: String,
    /// Postal code for non-coded localities (CEP).
    /// Only present when `situation` is `NotCoded`.
    pub cep: Option<String>,
    /// Coding situation of the locality (LOC_IN_SIT).
    pub situation: LocalitySituation,
    /// Type of locality (LOC_IN_TIPO_LOC).
    pub locality_type: LocalityType,
    /// ID of the parent locality (LOC_NU_SUB).
    /// Present when this locality is subordinate to another.
    pub subordinate_to: Option<LocalityId>,
    /// Abbreviated name of the locality (LOC_NO_ABREV).
    pub abbreviated_name: Option<String>,
    /// IBGE municipality code (MUN_NU).
    pub ibge_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locality_id_new() {
        let id = LocalityId::new(12345);
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn locality_id_try_from_valid() {
        let id = LocalityId::try_from(100).unwrap();
        assert_eq!(id.get(), 100);
    }

    #[test]
    fn locality_id_try_from_zero() {
        let result = LocalityId::try_from(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LocalityIdError::Zero);
    }

    #[test]
    fn locality_id_from_str_valid() {
        let id = LocalityId::from_str("12345").unwrap();
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn locality_id_from_str_invalid() {
        let result = LocalityId::from_str("abc");
        assert!(result.is_err());
    }

    #[test]
    fn locality_id_from_str_zero() {
        let result = LocalityId::from_str("0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LocalityIdError::Zero);
    }

    #[test]
    fn locality_situation_from_str_valid() {
        assert_eq!(
            LocalitySituation::from_str("0").unwrap(),
            LocalitySituation::NotCoded
        );
        assert_eq!(
            LocalitySituation::from_str("1").unwrap(),
            LocalitySituation::Coded
        );
        assert_eq!(
            LocalitySituation::from_str("2").unwrap(),
            LocalitySituation::DistrictOrVillage
        );
        assert_eq!(
            LocalitySituation::from_str("3").unwrap(),
            LocalitySituation::CodingInProgress
        );
    }

    #[test]
    fn locality_situation_from_str_invalid() {
        let result = LocalitySituation::from_str("4");
        assert!(result.is_err());
    }

    #[test]
    fn locality_type_from_str_valid() {
        assert_eq!(
            LocalityType::from_str("D").unwrap(),
            LocalityType::District
        );
        assert_eq!(
            LocalityType::from_str("M").unwrap(),
            LocalityType::Municipality
        );
        assert_eq!(
            LocalityType::from_str("P").unwrap(),
            LocalityType::Village
        );
        assert_eq!(
            LocalityType::from_str("d").unwrap(),
            LocalityType::District
        );
        assert_eq!(
            LocalityType::from_str("m").unwrap(),
            LocalityType::Municipality
        );
    }

    #[test]
    fn locality_type_from_str_invalid() {
        let result = LocalityType::from_str("X");
        assert!(result.is_err());
    }

    #[test]
    fn locality_id_display() {
        let id = LocalityId::new(12345);
        assert_eq!(id.to_string(), "12345");
    }

    #[test]
    fn locality_situation_display() {
        assert_eq!(LocalitySituation::NotCoded.to_string(), "0");
        assert_eq!(LocalitySituation::Coded.to_string(), "1");
    }

    #[test]
    fn locality_type_display() {
        assert_eq!(LocalityType::District.to_string(), "D");
        assert_eq!(LocalityType::Municipality.to_string(), "M");
        assert_eq!(LocalityType::Village.to_string(), "P");
    }
}
