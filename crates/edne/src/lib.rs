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

//! # eDNE Parser
//!
//! Fast, low-memory parser for Brazilian National Address Directory (eDNE) files.
//!
//! eDNE files are text files encoded in ISO-8859-1, where each line represents
//! a record and fields are separated by the '@' character.
//!
//! ## Features
//!
//! - Zero-copy parsing where possible
//! - Automatic ISO-8859-1 to UTF-8 conversion
//! - Type-safe models with validation
//! - Efficient HashMap-based collections
//! - Comprehensive error handling
//!
//! ## Example
//!
//! ```rust
//! use edne::parser::localities::Localities;
//!
// //! # fn main() -> Result<(), Box<dyn std::error::Error>> {
// //! // Parse from ISO-8859-1 encoded bytes
// //! let file_bytes = std::fs::read("localities.txt")?;
// //! let localities = Localities::from_iso8859_1(&file_bytes)?;
// //!
// //! println!("Parsed {} localities", localities.len());
// //! # Ok(())
// //! # }
// //! ```

pub mod error;
pub mod models;
pub mod parser;

pub use error::ParseError;
pub use models::{
    Address, AddressId, BigUser, BigUserId, Cpc, CpcId, Locality, LocalityId,
    Neighborhood, NeighborhoodId, OperationalUnit, OperationalUnitId,
    PostBoxIndicator, StreetId, StreetTypeIndicator, Uf,
};
