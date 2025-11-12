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

use edne::models::{
    AddressId, BigUserId, CpcId, LocalityId, NeighborhoodId,
    OperationalUnitId, Uf,
};

/// CEP type (8 digits)
pub type Cep = String;

/// Complete information for a CEP
#[derive(Debug, Clone)]
pub struct CepInfo {
    pub cep: Cep,
    pub uf: Uf,
    pub locality: String,
    pub neighborhood: Option<String>,
    pub address: String,
    pub complement: Option<String>,
    pub type_: CepType,
}

#[derive(Debug, Clone)]
pub enum CepType {
    UncodedLocality,
    Street,
    BigUser,
    OperationalUnit,
    Cpc,
}

/// Main lookup structure
pub struct CepLookup {
    ceps: HashMap<Cep, CepInfo>,
}

impl CepLookup {
    pub fn new() -> Self {
        Self { ceps: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { ceps: HashMap::with_capacity(capacity) }
    }

    pub fn len(&self) -> usize {
        self.ceps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ceps.is_empty()
    }

    pub fn get(&self, cep: &str) -> Option<&CepInfo> {
        self.ceps.get(cep)
    }

    pub fn insert(&mut self, info: CepInfo) {
        self.ceps.insert(info.cep.clone(), info);
    }

    /// Search by CEP following the Correios algorithm
    pub fn lookup(&self, cep: &str) -> Option<&CepInfo> {
        self.get(cep)
    }

    /// Returns all CEPs for a UF
    pub fn by_uf(&self, uf: Uf) -> Vec<&CepInfo> {
        self.ceps.values().filter(|info| info.uf == uf).collect()
    }

    /// Returns all CEPs for a locality
    pub fn by_locality(&self, locality: &str) -> Vec<&CepInfo> {
        self.ceps.values().filter(|info| info.locality == locality).collect()
    }
}

/// Builder to construct CepLookup from eDNE data
pub struct CepLookupBuilder {
    localities: HashMap<LocalityId, edne::models::Locality>,
    neighborhoods: HashMap<NeighborhoodId, edne::models::Neighborhood>,
    addresses: HashMap<AddressId, edne::models::Address>,
    big_users: HashMap<BigUserId, edne::models::BigUser>,
    operational_units:
        HashMap<OperationalUnitId, edne::models::OperationalUnit>,
    cpcs: HashMap<CpcId, edne::models::Cpc>,
}

impl CepLookupBuilder {
    pub fn new() -> Self {
        Self {
            localities: HashMap::new(),
            neighborhoods: HashMap::new(),
            addresses: HashMap::new(),
            big_users: HashMap::new(),
            operational_units: HashMap::new(),
            cpcs: HashMap::new(),
        }
    }

    pub fn add_localities(
        &mut self,
        localities: edne::parser::localities::Localities,
    ) {
        for (id, locality) in localities.iter() {
            self.localities.insert(*id, locality.clone());
        }
    }

    pub fn add_neighborhoods(
        &mut self,
        neighborhoods: edne::parser::neighborhoods::Neighborhoods,
    ) {
        for (id, neighborhood) in neighborhoods.iter() {
            self.neighborhoods.insert(*id, neighborhood.clone());
        }
    }

    pub fn add_addresses(
        &mut self,
        addresses: edne::parser::addresses::Addresses,
    ) {
        for (id, address) in addresses.iter() {
            self.addresses.insert(*id, address.clone());
        }
    }

    pub fn add_big_users(
        &mut self,
        big_users: edne::parser::big_users::BigUsers,
    ) {
        for (id, user) in big_users.iter() {
            self.big_users.insert(*id, user.clone());
        }
    }

    pub fn add_operational_units(
        &mut self,
        units: edne::parser::operational_units::OperationalUnits,
    ) {
        for (id, unit) in units.iter() {
            self.operational_units.insert(*id, unit.clone());
        }
    }

    pub fn add_cpcs(&mut self, cpcs: edne::parser::cpcs::Cpcs) {
        for (id, cpc) in cpcs.iter() {
            self.cpcs.insert(*id, cpc.clone());
        }
    }

    /// Build CepLookup following the Correios algorithm order
    pub fn build(self) -> CepLookup {
        let mut lookup = CepLookup::with_capacity(
            self.localities.len()
                + self.addresses.len()
                + self.big_users.len()
                + self.operational_units.len()
                + self.cpcs.len(),
        );

        // 1. Uncoded localities (general CEP)
        for locality in self.localities.values() {
            if let Some(cep) = &locality.cep {
                let neighborhood =
                    if let Some(sub_id) = locality.subordinate_to {
                        self.localities.get(&sub_id).map(|l| l.name.clone())
                    } else {
                        None
                    };

                lookup.insert(CepInfo {
                    cep: cep.clone(),
                    uf: locality.uf,
                    locality: locality.name.clone(),
                    neighborhood,
                    address: String::new(),
                    complement: None,
                    type_: CepType::UncodedLocality,
                });
            }
        }

        // 2. Streets (ruas, avenidas, etc)
        for address in self.addresses.values() {
            let locality = self
                .localities
                .get(&address.locality_id)
                .map(|l| l.name.clone())
                .unwrap_or_default();

            let neighborhood = self
                .neighborhoods
                .get(&address.neighborhood_id_start)
                .map(|n| n.name.clone());

            let address_str = if address.street_type_indicator
                == Some(edne::models::StreetTypeIndicator::Yes)
            {
                format!("{} {}", address.street_type, address.name)
            } else {
                address.name.clone()
            };

            lookup.insert(CepInfo {
                cep: address.cep.clone(),
                uf: address.uf,
                locality,
                neighborhood,
                address: address_str,
                complement: address.complement.clone(),
                type_: CepType::Street,
            });
        }

        // 3. Big Users
        for user in self.big_users.values() {
            let locality = self
                .localities
                .get(&user.locality_id)
                .map(|l| l.name.clone())
                .unwrap_or_default();

            let neighborhood = self
                .neighborhoods
                .get(&user.neighborhood_id)
                .map(|n| n.name.clone());

            lookup.insert(CepInfo {
                cep: user.cep.clone(),
                uf: user.uf,
                locality,
                neighborhood,
                address: user.address.clone(),
                complement: Some(user.name.clone()),
                type_: CepType::BigUser,
            });
        }

        // 4. Operational Units
        for unit in self.operational_units.values() {
            let locality = self
                .localities
                .get(&unit.locality_id)
                .map(|l| l.name.clone())
                .unwrap_or_default();

            let neighborhood = self
                .neighborhoods
                .get(&unit.neighborhood_id)
                .map(|n| n.name.clone());

            lookup.insert(CepInfo {
                cep: unit.cep.clone(),
                uf: unit.uf,
                locality,
                neighborhood,
                address: unit.address.clone(),
                complement: Some(unit.name.clone()),
                type_: CepType::OperationalUnit,
            });
        }

        // 5. CPCs
        for cpc in self.cpcs.values() {
            let locality = self
                .localities
                .get(&cpc.locality_id)
                .map(|l| l.name.clone())
                .unwrap_or_default();

            lookup.insert(CepInfo {
                cep: cpc.cep.clone(),
                uf: cpc.uf,
                locality,
                neighborhood: None,
                address: cpc.address.clone(),
                complement: Some(cpc.name.clone()),
                type_: CepType::Cpc,
            });
        }

        lookup
    }
}

impl Default for CepLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CepLookupBuilder {
    fn default() -> Self {
        Self::new()
    }
}
