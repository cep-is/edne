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

use crate::models::{LocalityId, NeighborhoodId, Uf};

/// Unique identifier for an address (street/logradouro).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressId(u32);

impl AddressId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for AddressId {
    type Error = AddressIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(AddressIdError::Zero);
        }
        Ok(Self(value))
    }
}

impl FromStr for AddressId {
    type Err = AddressIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .parse::<u32>()
            .map_err(|_| AddressIdError::InvalidFormat(s.to_string()))?;
        Self::try_from(value)
    }
}

impl fmt::Display for AddressId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors when parsing or creating an `AddressId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressIdError {
    Zero,
    InvalidFormat(String),
}

impl fmt::Display for AddressIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "address ID cannot be zero"),
            Self::InvalidFormat(s) => {
                write!(f, "invalid address ID format: '{}'", s)
            }
        }
    }
}

impl Error for AddressIdError {}

/// Street type usage indicator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StreetTypeIndicator {
    /// Use street type (S).
    Yes,
    /// Don't use street type (N).
    No,
}

impl FromStr for StreetTypeIndicator {
    type Err = StreetTypeIndicatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "S" => Ok(Self::Yes),
            "N" => Ok(Self::No),
            other => {
                Err(StreetTypeIndicatorError::InvalidCode(other.to_string()))
            }
        }
    }
}

impl fmt::Display for StreetTypeIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::Yes => "S",
            Self::No => "N",
        };
        write!(f, "{}", code)
    }
}

/// Errors when parsing `StreetTypeIndicator`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreetTypeIndicatorError {
    InvalidCode(String),
}

impl fmt::Display for StreetTypeIndicatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCode(s) => {
                write!(f, "invalid street type indicator code: '{}'", s)
            }
        }
    }
}

impl Error for StreetTypeIndicatorError {}

/// Represents an address (street/logradouro) from the eDNE database.
///
/// This contains records from coded localities (LOC_IN_SIT=1) and
/// localities in coding phase (LOC_IN_SIT=3). To find the neighborhood
/// of the street, use BAI_NU_INI (relates to LOG_BAIRRO, field BAI_NU).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    /// Unique identifier for the address (LOG_NU).
    pub id: AddressId,
    /// Federative unit abbreviation (UFE_SG).
    pub uf: Uf,
    /// Locality ID (LOC_NU).
    pub locality_id: LocalityId,
    /// Initial neighborhood ID of the street (BAI_NU_INI).
    pub neighborhood_id_start: NeighborhoodId,
    /// Final neighborhood ID of the street (BAI_NU_FIM) - optional.
    pub neighborhood_id_end: Option<NeighborhoodId>,
    /// Name of the street (LOG_NO).
    pub name: String,
    /// Complement (LOG_COMPLEMENTO) - optional.
    pub complement: Option<String>,
    /// Postal code (CEP).
    pub cep: String,
    /// Street type (TLO_TX) - e.g., "Rua", "Avenida", "Travessa".
    pub street_type: String,
    /// Indicator to use street type (LOG_STA_TLO) - optional.
    pub street_type_indicator: Option<StreetTypeIndicator>,
    /// Abbreviated name (LOG_NO_ABREV) - optional.
    pub abbreviated_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_id_new() {
        let id = AddressId::new(1);
        assert_eq!(id.get(), 1);
    }

    #[test]
    fn address_id_try_from_zero() {
        let result = AddressId::try_from(0);
        assert!(result.is_err());
    }

    #[test]
    fn street_type_indicator_from_str() {
        assert_eq!(
            StreetTypeIndicator::from_str("S").unwrap(),
            StreetTypeIndicator::Yes
        );
        assert_eq!(
            StreetTypeIndicator::from_str("N").unwrap(),
            StreetTypeIndicator::No
        );
        assert_eq!(
            StreetTypeIndicator::from_str("s").unwrap(),
            StreetTypeIndicator::Yes
        );
    }

    #[test]
    fn street_type_indicator_invalid() {
        let result = StreetTypeIndicator::from_str("X");
        assert!(result.is_err());
    }
}
