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

use std::collections::HashMap;

// /// Address identifier (opaque newtype).
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct AddressId(String);

// impl AddressId {
//     pub(crate) fn new(id: String) -> Self {
//         Self(id)
//     }
// }

// /// Address entity kept in memory (owns its strings).
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Address {
//     pub id: AddressId, // CEP
//     pub uf: String,
//     pub complement: Option<String>,
//     pub neighborhood: String,
//     pub name: String,
// }

// /// Encapsulated index of addresses.
// /// Keeps invariants and allows changing inner structure later without API
// /// breakage.
// #[derive(Debug, Default)]
// pub struct Addresses(HashMap<AddressId, Address>);

// impl Addresses {
//     pub(crate) fn new(map: HashMap<AddressId, Address>) -> Self {
//         Self(map)
//     }

//     /// Returns the number of localities.
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }

//     /// Returns `true` if there are no items.
//     pub fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }

//     /// Returns a locality by id.
//     pub fn get(&self, id: AddressId) -> Option<&Address> {
//         self.0.get(&id)
//     }

//     /// Iterates (read-only) over all localities.
//     pub fn iter(&self) -> impl Iterator<Item = (&AddressId, &Address)> {
//         self.0.iter()
//     }
// }

/// Locality identifier (opaque newtype).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalityId(u32);

impl LocalityId {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }
}

/// Locality entity kept in memory (owns its strings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locality {
    pub id: LocalityId,
    pub name: String,
}

/// Encapsulated index of localities.
/// Keeps invariants and allows changing inner structure later without API
/// breakage.
#[derive(Debug, Default)]
pub struct Localities(HashMap<LocalityId, Locality>);

impl Localities {
    pub(crate) fn new(map: HashMap<LocalityId, Locality>) -> Self {
        Self(map)
    }

    /// Returns the number of localities.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no items.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a locality by id.
    pub fn get(&self, id: LocalityId) -> Option<&Locality> {
        self.0.get(&id)
    }

    /// Iterates (read-only) over all localities.
    pub fn iter(&self) -> impl Iterator<Item = (&LocalityId, &Locality)> {
        self.0.iter()
    }
}

/// Neighborhood identifier (opaque newtype).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NeighborhoodId(u32);

impl NeighborhoodId {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }
}

/// Neighborhood entity kept in memory (owns its strings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Neighborhood {
    pub id: NeighborhoodId,
    pub name: String,
}

/// Encapsulated index of neighborhoods.
/// Keeps invariants and allows changing inner structure later without API
/// breakage.
#[derive(Debug, Default)]
pub struct Neighborhoods(HashMap<NeighborhoodId, Neighborhood>);

impl Neighborhoods {
    pub(crate) fn new(map: HashMap<NeighborhoodId, Neighborhood>) -> Self {
        Self(map)
    }

    /// Returns the number of neighborhoods.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no items.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a neighborhood by id.
    pub fn get(&self, id: NeighborhoodId) -> Option<&Neighborhood> {
        self.0.get(&id)
    }

    /// Iterates (read-only) over all neighborhoods.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&NeighborhoodId, &Neighborhood)> {
        self.0.iter()
    }
}
