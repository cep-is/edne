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

use crate::models::{LocalityId, NeighborhoodId, StreetId, Uf};

/// Unique identifier for an operational unit.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationalUnitId(u32);

impl OperationalUnitId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for OperationalUnitId {
    type Error = OperationalUnitIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(OperationalUnitIdError::Zero);
        }
        Ok(Self(value))
    }
}

impl FromStr for OperationalUnitId {
    type Err = OperationalUnitIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim().parse::<u32>().map_err(|_| {
            OperationalUnitIdError::InvalidFormat(s.to_string())
        })?;
        Self::try_from(value)
    }
}

impl fmt::Display for OperationalUnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors when parsing or creating an `OperationalUnitId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalUnitIdError {
    Zero,
    InvalidFormat(String),
}

impl fmt::Display for OperationalUnitIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "operational unit ID cannot be zero"),
            Self::InvalidFormat(s) => {
                write!(f, "invalid operational unit ID format: '{}'", s)
            }
        }
    }
}

impl Error for OperationalUnitIdError {}

/// Post box indicator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PostBoxIndicator {
    /// Has post box (S).
    Yes,
    /// No post box (N).
    No,
}

impl FromStr for PostBoxIndicator {
    type Err = PostBoxIndicatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "S" => Ok(Self::Yes),
            "N" => Ok(Self::No),
            other => {
                Err(PostBoxIndicatorError::InvalidCode(other.to_string()))
            }
        }
    }
}

impl fmt::Display for PostBoxIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::Yes => "S",
            Self::No => "N",
        };
        write!(f, "{}", code)
    }
}

/// Errors when parsing `PostBoxIndicator`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostBoxIndicatorError {
    InvalidCode(String),
}

impl fmt::Display for PostBoxIndicatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCode(s) => {
                write!(f, "invalid post box indicator code: '{}'", s)
            }
        }
    }
}

impl Error for PostBoxIndicatorError {}

/// Represents an operational unit from the eDNE database.
///
/// Operational units are postal offices (own or franchised), distribution
/// centers, etc. For non-coded localities (LOC_IN_SIT=0), the LOG_NU field
/// is empty and UOP_ENDERECO should be used for addressing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalUnit {
    /// Unique identifier for the operational unit (UOP_NU).
    pub id: OperationalUnitId,
    /// Federative unit abbreviation (UFE_SG).
    pub uf: Uf,
    /// Locality ID (LOC_NU).
    pub locality_id: LocalityId,
    /// Neighborhood ID (BAI_NU).
    pub neighborhood_id: NeighborhoodId,
    /// Street ID (LOG_NU) - optional, empty for non-coded localities.
    pub street_id: Option<StreetId>,
    /// Name of the operational unit (UOP_NO).
    pub name: String,
    /// Address of the operational unit (UOP_ENDERECO).
    pub address: String,
    /// Postal code (CEP).
    pub cep: String,
    /// Post box indicator (UOP_IN_CP).
    pub post_box_indicator: PostBoxIndicator,
    /// Abbreviated name (UOP_NO_ABREV) - optional.
    pub abbreviated_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operational_unit_id_new() {
        let id = OperationalUnitId::new(48437);
        assert_eq!(id.get(), 48437);
    }

    #[test]
    fn operational_unit_id_try_from_zero() {
        let result = OperationalUnitId::try_from(0);
        assert!(result.is_err());
    }

    #[test]
    fn post_box_indicator_from_str() {
        assert_eq!(
            PostBoxIndicator::from_str("S").unwrap(),
            PostBoxIndicator::Yes
        );
        assert_eq!(
            PostBoxIndicator::from_str("N").unwrap(),
            PostBoxIndicator::No
        );
        assert_eq!(
            PostBoxIndicator::from_str("s").unwrap(),
            PostBoxIndicator::Yes
        );
        assert_eq!(
            PostBoxIndicator::from_str("n").unwrap(),
            PostBoxIndicator::No
        );
    }

    #[test]
    fn post_box_indicator_invalid() {
        let result = PostBoxIndicator::from_str("X");
        assert!(result.is_err());
    }
}
