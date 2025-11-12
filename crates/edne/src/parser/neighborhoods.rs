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

use std::{collections::HashMap, str::FromStr};

use crate::{
    models::{
        LocalityId, Uf,
        neighborhood::{Neighborhood, NeighborhoodId},
    },
    parser::base::{EdneParser, ParseError},
};

/// Expected number of fields in a neighborhood record.
const NEIGHBORHOOD_FIELD_COUNT: usize = 5;

/// Collection of neighborhoods indexed by their ID.
#[derive(Debug, Clone)]
pub struct Neighborhoods(HashMap<NeighborhoodId, Neighborhood>);

impl Neighborhoods {
    /// Creates a new empty collection.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates a collection with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns the number of neighborhoods.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets a neighborhood by ID.
    pub fn get(&self, id: &NeighborhoodId) -> Option<&Neighborhood> {
        self.0.get(id)
    }

    /// Inserts a neighborhood into the collection.
    pub fn insert(
        &mut self,
        neighborhood: Neighborhood,
    ) -> Option<Neighborhood> {
        self.0.insert(neighborhood.id, neighborhood)
    }

    /// Returns an iterator over all neighborhoods.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&NeighborhoodId, &Neighborhood)> {
        self.0.iter()
    }

    /// Parses neighborhoods from ISO-8859-1 encoded bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Raw file content in ISO-8859-1 encoding
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if encoding fails or any line has invalid data.
    pub fn from_iso8859_1(bytes: &[u8]) -> Result<Self, ParseError> {
        let parser = EdneParser::from_iso8859_1(bytes)?;
        Self::parse_with_parser(&parser)
    }

    /// Parses neighborhoods from UTF-8 string (for testing).
    pub fn from_utf8(content: String) -> Result<Self, ParseError> {
        let parser = EdneParser::from_utf8(content);
        Self::parse_with_parser(&parser)
    }

    /// Internal method to parse neighborhoods using a configured parser.
    fn parse_with_parser(parser: &EdneParser) -> Result<Self, ParseError> {
        let lines: Vec<_> = parser.lines().collect();
        let mut neighborhoods = Self::with_capacity(lines.len());

        for (line_number, line) in lines {
            let neighborhood =
                parse_neighborhood_line(parser, line, line_number)?;
            neighborhoods.insert(neighborhood);
        }

        Ok(neighborhoods)
    }
}

impl Default for Neighborhoods {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a single neighborhood line into a `Neighborhood` struct.
///
/// # Field order (5 fields):
/// 1. BAI_NU - Neighborhood ID
/// 2. UFE_SG - UF code
/// 3. LOC_NU - Locality ID
/// 4. BAI_NO - Neighborhood name
/// 5. BAI_NO_ABREV - Abbreviated name (optional)
fn parse_neighborhood_line(
    parser: &EdneParser,
    line: &str,
    line_number: usize,
) -> Result<Neighborhood, ParseError> {
    let fields = parser.parse_line_checked(
        line,
        NEIGHBORHOOD_FIELD_COUNT,
        line_number,
    )?;

    // Parse required fields
    let id_str = EdneParser::required_field(fields[0], "BAI_NU", line_number)?;
    let id = NeighborhoodId::from_str(&id_str).map_err(|e| {
        ParseError::InvalidValue {
            field_name: "BAI_NU",
            value: id_str,
            reason: e.to_string(),
            line_number,
        }
    })?;

    let uf_str = EdneParser::required_field(fields[1], "UFE_SG", line_number)?;
    let uf = Uf::from_str(&uf_str).map_err(|e| ParseError::InvalidValue {
        field_name: "UFE_SG",
        value: uf_str,
        reason: e.to_string(),
        line_number,
    })?;

    let loc_id_str =
        EdneParser::required_field(fields[2], "LOC_NU", line_number)?;
    let locality_id = LocalityId::from_str(&loc_id_str).map_err(|e| {
        ParseError::InvalidValue {
            field_name: "LOC_NU",
            value: loc_id_str,
            reason: e.to_string(),
            line_number,
        }
    })?;

    let name = EdneParser::required_field(fields[3], "BAI_NO", line_number)?;

    // Parse optional field
    let abbreviated_name = EdneParser::optional_field(fields[4]);

    Ok(Neighborhood { id, uf, locality_id, name, abbreviated_name })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "\
55400@AC@16@Loteamento Jaguar@Lot Jaguar
55402@AC@16@Loteamento Santa Luzia@Lot Sta Luzia
55403@AC@16@Habitasa@Habitasa
55404@AC@16@Baixada da Habitasa@Baixada Habitasa
55405@AC@16@Baixada da Cadeia Velha@Baixada C Velha
39321@AC@4@Centro@Centro
39322@AC@14@Centro@Centro
39323@AC@5@Centro@Centro
39324@AC@20@Centro@Centro
39325@AC@13@Centro@Centro
39326@AC@22@Centro@Centro
39327@AC@3@Centro@Centro
39328@AC@7@Centro@Centro
39329@AC@2@Centro@Centro
39330@AC@19@Centro@Centro";

    #[test]
    fn parse_sample_data() {
        let neighborhoods =
            Neighborhoods::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        assert_eq!(neighborhoods.len(), 15);
    }

    #[test]
    fn parse_neighborhood_with_all_fields() {
        let neighborhoods =
            Neighborhoods::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = NeighborhoodId::new(55400);
        let neighborhood = neighborhoods.get(&id).unwrap();

        assert_eq!(neighborhood.id, id);
        assert_eq!(neighborhood.uf, Uf::AC);
        assert_eq!(neighborhood.locality_id, LocalityId::new(16));
        assert_eq!(neighborhood.name, "Loteamento Jaguar");
        assert_eq!(
            neighborhood.abbreviated_name,
            Some("Lot Jaguar".to_string())
        );
    }

    #[test]
    fn parse_neighborhood_centro() {
        let neighborhoods =
            Neighborhoods::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = NeighborhoodId::new(39321);
        let neighborhood = neighborhoods.get(&id).unwrap();

        assert_eq!(neighborhood.id, id);
        assert_eq!(neighborhood.uf, Uf::AC);
        assert_eq!(neighborhood.locality_id, LocalityId::new(4));
        assert_eq!(neighborhood.name, "Centro");
        assert_eq!(neighborhood.abbreviated_name, Some("Centro".to_string()));
    }

    #[test]
    fn parse_all_centro_neighborhoods() {
        let neighborhoods =
            Neighborhoods::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let centro_count =
            neighborhoods.iter().filter(|(_, n)| n.name == "Centro").count();
        assert_eq!(centro_count, 10);
    }

    #[test]
    fn parse_neighborhood_same_locality() {
        let neighborhoods =
            Neighborhoods::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let loc_16_count = neighborhoods
            .iter()
            .filter(|(_, n)| n.locality_id == LocalityId::new(16))
            .count();
        assert_eq!(loc_16_count, 5);
    }

    #[test]
    fn parse_invalid_field_count() {
        let invalid = "55400@AC@16@Loteamento Jaguar";
        let result = Neighborhoods::from_utf8(invalid.to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::FieldCount { expected, got, .. } => {
                assert_eq!(expected, 5);
                assert_eq!(got, 4);
            }
            _ => panic!("Expected FieldCount error"),
        }
    }

    #[test]
    fn parse_invalid_id() {
        let invalid = "abc@AC@16@Loteamento Jaguar@Lot Jaguar";
        let result = Neighborhoods::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_uf() {
        let invalid = "55400@ZZ@16@Loteamento Jaguar@Lot Jaguar";
        let result = Neighborhoods::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_locality_id() {
        let invalid = "55400@AC@abc@Loteamento Jaguar@Lot Jaguar";
        let result = Neighborhoods::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_required_field() {
        let invalid = "55400@AC@16@@Lot Jaguar";
        let result = Neighborhoods::from_utf8(invalid.to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::EmptyField { field_name, .. } => {
                assert_eq!(field_name, "BAI_NO");
            }
            _ => panic!("Expected EmptyField error"),
        }
    }

    #[test]
    fn parse_without_abbreviation() {
        let data = "55400@AC@16@Loteamento Jaguar@";
        let neighborhoods =
            Neighborhoods::from_utf8(data.to_string()).unwrap();
        let id = NeighborhoodId::new(55400);
        let neighborhood = neighborhoods.get(&id).unwrap();
        assert_eq!(neighborhood.abbreviated_name, None);
    }

    #[test]
    fn neighborhoods_iterator() {
        let neighborhoods =
            Neighborhoods::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let count = neighborhoods.iter().count();
        assert_eq!(count, 15);
    }

    #[test]
    fn neighborhoods_get_nonexistent() {
        let neighborhoods =
            Neighborhoods::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let result = neighborhoods.get(&NeighborhoodId::new(99999));
        assert!(result.is_none());
    }
}
