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

/// Unique identifier for a community postal box (CPC).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CpcId(u32);

impl CpcId {
    /// Creates a new `CpcId`.
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

impl TryFrom<u32> for CpcId {
    type Error = CpcIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(CpcIdError::Zero);
        }
        Ok(Self(value))
    }
}

impl FromStr for CpcId {
    type Err = CpcIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .parse::<u32>()
            .map_err(|_| CpcIdError::InvalidFormat(s.to_string()))?;
        Self::try_from(value)
    }
}

impl fmt::Display for CpcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors when parsing or creating a `CpcId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpcIdError {
    /// ID cannot be zero.
    Zero,
    /// String is not a valid number.
    InvalidFormat(String),
}

impl fmt::Display for CpcIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "CPC ID cannot be zero"),
            Self::InvalidFormat(s) => {
                write!(f, "invalid CPC ID format: '{}'", s)
            }
        }
    }
}

impl Error for CpcIdError {}

/// Represents a Community Postal Box (Caixa Postal Comunitária) from the eDONE database.
///
/// CPCs serve rural and peripheral urban areas not covered by home delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cpc {
    /// Unique identifier for the CPC (CPC_NU).
    pub id: CpcId,
    /// Federative unit abbreviation (UFE_SG).
    pub uf: Uf,
    /// Locality ID this CPC belongs to (LOC_NU).
    pub locality_id: LocalityId,
    /// Name of the CPC (CPC_NO).
    pub name: String,
    /// Address of the CPC (CPC_ENDERECO).
    pub address: String,
    /// Postal code (CEP).
    pub cep: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpc_id_new() {
        let id = CpcId::new(1285);
        assert_eq!(id.get(), 1285);
    }

    #[test]
    fn cpc_id_try_from_valid() {
        let id = CpcId::try_from(3788).unwrap();
        assert_eq!(id.get(), 3788);
    }

    #[test]
    fn cpc_id_try_from_zero() {
        let result = CpcId::try_from(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CpcIdError::Zero);
    }

    #[test]
    fn cpc_id_from_str_valid() {
        let id = CpcId::from_str("1285").unwrap();
        assert_eq!(id.get(), 1285);
    }

    #[test]
    fn cpc_id_from_str_invalid() {
        let result = CpcId::from_str("abc");
        assert!(result.is_err());
    }

    #[test]
    fn cpc_id_from_str_zero() {
        let result = CpcId::from_str("0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CpcIdError::Zero);
    }

    #[test]
    fn cpc_id_display() {
        let id = CpcId::new(1285);
        assert_eq!(id.to_string(), "1285");
    }
}
