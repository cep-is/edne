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

/// Unique identifier for a big user.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BigUserId(u32);

impl BigUserId {
    /// Creates a new `BigUserId`.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the inner value.
    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for BigUserId {
    type Error = BigUserIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(BigUserIdError::Zero);
        }
        Ok(Self(value))
    }
}

impl FromStr for BigUserId {
    type Err = BigUserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .parse::<u32>()
            .map_err(|_| BigUserIdError::InvalidFormat(s.to_string()))?;
        Self::try_from(value)
    }
}

impl fmt::Display for BigUserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors when parsing or creating a `BigUserId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigUserIdError {
    Zero,
    InvalidFormat(String),
}

impl fmt::Display for BigUserIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "big user ID cannot be zero"),
            Self::InvalidFormat(s) => {
                write!(f, "invalid big user ID format: '{}'", s)
            }
        }
    }
}

impl Error for BigUserIdError {}

/// Unique identifier for a street (logradouro).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StreetId(u32);

impl StreetId {
    /// Creates a new `StreetId`.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the inner value.
    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for StreetId {
    type Error = StreetIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(StreetIdError::Zero);
        }
        Ok(Self(value))
    }
}

impl FromStr for StreetId {
    type Err = StreetIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .parse::<u32>()
            .map_err(|_| StreetIdError::InvalidFormat(s.to_string()))?;
        Self::try_from(value)
    }
}

impl fmt::Display for StreetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors when parsing or creating a `StreetId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreetIdError {
    Zero,
    InvalidFormat(String),
}

impl fmt::Display for StreetIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "street ID cannot be zero"),
            Self::InvalidFormat(s) => {
                write!(f, "invalid street ID format: '{}'", s)
            }
        }
    }
}

impl Error for StreetIdError {}

/// Represents a big user from the eDNE database.
///
/// Big users are clients with large postal volume (companies, universities,
/// banks, public agencies, etc). For non-coded localities (LOC_IN_SIT=0),
/// the LOG_NU field is empty and GRU_ENDERECO should be used for addressing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUser {
    /// Unique identifier for the big user (GRU_NU).
    pub id: BigUserId,
    /// Federative unit abbreviation (UFE_SG).
    pub uf: Uf,
    /// Locality ID (LOC_NU).
    pub locality_id: LocalityId,
    /// Neighborhood ID (BAI_NU).
    pub neighborhood_id: NeighborhoodId,
    /// Street ID (LOG_NU) - optional, empty for non-coded localities.
    pub street_id: Option<StreetId>,
    /// Name of the big user (GRU_NO).
    pub name: String,
    /// Address of the big user (GRU_ENDERECO).
    pub address: String,
    /// Postal code (CEP).
    pub cep: String,
    /// Abbreviated name (GRU_NO_ABREV) - optional.
    pub abbreviated_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_user_id_new() {
        let id = BigUserId::new(41739);
        assert_eq!(id.get(), 41739);
    }

    #[test]
    fn big_user_id_try_from_zero() {
        let result = BigUserId::try_from(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BigUserIdError::Zero);
    }

    #[test]
    fn street_id_new() {
        let id = StreetId::new(949512);
        assert_eq!(id.get(), 949512);
    }

    #[test]
    fn street_id_try_from_zero() {
        let result = StreetId::try_from(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StreetIdError::Zero);
    }
}
