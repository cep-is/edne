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

use crate::models::{LocalityId, Uf};

/// Unique identifier for a neighborhood.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NeighborhoodId(u32);

impl NeighborhoodId {
    /// Creates a new `NeighborhoodId`.
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

impl TryFrom<u32> for NeighborhoodId {
    type Error = NeighborhoodIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(NeighborhoodIdError::Zero);
        }
        Ok(Self(value))
    }
}

impl FromStr for NeighborhoodId {
    type Err = NeighborhoodIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .parse::<u32>()
            .map_err(|_| NeighborhoodIdError::InvalidFormat(s.to_string()))?;
        Self::try_from(value)
    }
}

impl fmt::Display for NeighborhoodId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors when parsing or creating a `NeighborhoodId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeighborhoodIdError {
    /// ID cannot be zero.
    Zero,
    /// String is not a valid number.
    InvalidFormat(String),
}

impl fmt::Display for NeighborhoodIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "neighborhood ID cannot be zero"),
            Self::InvalidFormat(s) => {
                write!(f, "invalid neighborhood ID format: '{}'", s)
            }
        }
    }
}

impl Error for NeighborhoodIdError {}

/// Represents a neighborhood from the eDNE database.
///
/// A neighborhood (bairro) is a subdivision within a locality,
/// with optional abbreviated name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Neighborhood {
    /// Unique identifier for the neighborhood (BAI_NU).
    pub id: NeighborhoodId,
    /// Federative unit abbreviation (UFE_SG).
    pub uf: Uf,
    /// Locality ID this neighborhood belongs to (LOC_NU).
    pub locality_id: LocalityId,
    /// Name of the neighborhood (BAI_NO).
    pub name: String,
    /// Abbreviated name of the neighborhood (BAI_NO_ABREV).
    pub abbreviated_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighborhood_id_new() {
        let id = NeighborhoodId::new(55400);
        assert_eq!(id.get(), 55400);
    }

    #[test]
    fn neighborhood_id_try_from_valid() {
        let id = NeighborhoodId::try_from(39321).unwrap();
        assert_eq!(id.get(), 39321);
    }

    #[test]
    fn neighborhood_id_try_from_zero() {
        let result = NeighborhoodId::try_from(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NeighborhoodIdError::Zero);
    }

    #[test]
    fn neighborhood_id_from_str_valid() {
        let id = NeighborhoodId::from_str("55400").unwrap();
        assert_eq!(id.get(), 55400);
    }

    #[test]
    fn neighborhood_id_from_str_invalid() {
        let result = NeighborhoodId::from_str("abc");
        assert!(result.is_err());
    }

    #[test]
    fn neighborhood_id_from_str_zero() {
        let result = NeighborhoodId::from_str("0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NeighborhoodIdError::Zero);
    }

    #[test]
    fn neighborhood_id_display() {
        let id = NeighborhoodId::new(55400);
        assert_eq!(id.to_string(), "55400");
    }
}
