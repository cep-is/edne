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
        Uf,
        locality::{Locality, LocalityId, LocalitySituation, LocalityType},
    },
    parser::base::{EdneParser, ParseError},
};

/// Expected number of fields in a locality record.
const LOCALITY_FIELD_COUNT: usize = 9;

/// Collection of localities indexed by their ID.
#[derive(Debug, Clone)]
pub struct Localities(HashMap<LocalityId, Locality>);

impl Localities {
    /// Creates a new empty collection.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates a collection with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns the number of localities.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets a locality by ID.
    pub fn get(&self, id: &LocalityId) -> Option<&Locality> {
        self.0.get(id)
    }

    /// Inserts a locality into the collection.
    pub fn insert(&mut self, locality: Locality) -> Option<Locality> {
        self.0.insert(locality.id, locality)
    }

    /// Returns an iterator over all localities.
    pub fn iter(&self) -> impl Iterator<Item = (&LocalityId, &Locality)> {
        self.0.iter()
    }

    /// Parses localities from ISO-8859-1 encoded bytes.
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

    /// Parses localities from UTF-8 string (for testing).
    pub fn from_utf8(content: String) -> Result<Self, ParseError> {
        let parser = EdneParser::from_utf8(content);
        Self::parse_with_parser(&parser)
    }

    /// Internal method to parse localities using a configured parser.
    fn parse_with_parser(parser: &EdneParser) -> Result<Self, ParseError> {
        let lines: Vec<_> = parser.lines().collect();
        let mut localities = Self::with_capacity(lines.len());

        for (line_number, line) in lines {
            let locality = parse_locality_line(parser, line, line_number)?;
            localities.insert(locality);
        }

        Ok(localities)
    }
}

impl Default for Localities {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a single locality line into a `Locality` struct.
///
/// # Field order (9 fields):
/// 1. LOC_NU - Locality ID
/// 2. UFE_SG - UF code
/// 3. LOC_NO - Locality name
/// 4. CEP - Postal code (optional)
/// 5. LOC_IN_SIT - Situation
/// 6. LOC_IN_TIPO_LOC - Locality type
/// 7. LOC_NU_SUB - Parent locality ID (optional)
/// 8. LOC_NO_ABREV - Abbreviated name (optional)
/// 9. MUN_NU - IBGE code (optional)
fn parse_locality_line(
    parser: &EdneParser,
    line: &str,
    line_number: usize,
) -> Result<Locality, ParseError> {
    let fields =
        parser.parse_line_checked(line, LOCALITY_FIELD_COUNT, line_number)?;

    // Parse required fields
    let id_str = EdneParser::required_field(fields[0], "LOC_NU", line_number)?;
    let id = LocalityId::from_str(&id_str).map_err(|e| {
        ParseError::InvalidValue {
            field_name: "LOC_NU",
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

    let name = EdneParser::required_field(fields[2], "LOC_NO", line_number)?;

    let situation_str =
        EdneParser::required_field(fields[4], "LOC_IN_SIT", line_number)?;
    let situation =
        LocalitySituation::from_str(&situation_str).map_err(|e| {
            ParseError::InvalidValue {
                field_name: "LOC_IN_SIT",
                value: situation_str,
                reason: e.to_string(),
                line_number,
            }
        })?;

    let type_str =
        EdneParser::required_field(fields[5], "LOC_IN_TIPO_LOC", line_number)?;
    let locality_type = LocalityType::from_str(&type_str).map_err(|e| {
        ParseError::InvalidValue {
            field_name: "LOC_IN_TIPO_LOC",
            value: type_str,
            reason: e.to_string(),
            line_number,
        }
    })?;

    // Parse optional fields
    let cep = EdneParser::optional_field(fields[3]);

    let subordinate_to =
        if let Some(sub_id_str) = EdneParser::optional_field(fields[6]) {
            Some(LocalityId::from_str(&sub_id_str).map_err(|e| {
                ParseError::InvalidValue {
                    field_name: "LOC_NU_SUB",
                    value: sub_id_str,
                    reason: e.to_string(),
                    line_number,
                }
            })?)
        } else {
            None
        };

    let abbreviated_name = EdneParser::optional_field(fields[7]);
    let ibge_code = EdneParser::optional_field(fields[8]);

    Ok(Locality {
        id,
        uf,
        name,
        cep,
        situation,
        locality_type,
        subordinate_to,
        abbreviated_name,
        ibge_code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "\
15321@AC@Terra Indgena Mamoadate@69939810@0@P@2@Terra Ind Mamoadate@
13@AC@Plcido de Castro@69928000@0@M@@Plcido Castro@1200385
15323@AC@Terra Indgena Kampa e Isolados do Rio Envira@69969820@0@P@8@Terra Ind K I R Envira@
16@AC@Rio Branco@@1@M@@Rio Branco@1200401
12@AC@Marechal Thaumaturgo@69983000@0@M@@Mal Thaumaturgo@1200351";

    #[test]
    fn parse_sample_data() {
        let localities =
            Localities::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        assert_eq!(localities.len(), 5);
    }

    #[test]
    fn parse_locality_with_all_fields() {
        let localities =
            Localities::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = LocalityId::new(13);
        let locality = localities.get(&id).unwrap();

        assert_eq!(locality.id, id);
        assert_eq!(locality.uf, Uf::AC);
        assert_eq!(locality.name, "Plcido de Castro");
        assert_eq!(locality.cep, Some("69928000".to_string()));
        assert_eq!(locality.situation, LocalitySituation::NotCoded);
        assert_eq!(locality.locality_type, LocalityType::Municipality);
        assert_eq!(locality.subordinate_to, None);
        assert_eq!(
            locality.abbreviated_name,
            Some("Plcido Castro".to_string())
        );
        assert_eq!(locality.ibge_code, Some("1200385".to_string()));
    }

    #[test]
    fn parse_locality_with_optional_fields_empty() {
        let localities =
            Localities::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = LocalityId::new(16);
        let locality = localities.get(&id).unwrap();

        assert_eq!(locality.id, id);
        assert_eq!(locality.uf, Uf::AC);
        assert_eq!(locality.name, "Rio Branco");
        assert_eq!(locality.cep, None);
        assert_eq!(locality.situation, LocalitySituation::Coded);
        assert_eq!(locality.locality_type, LocalityType::Municipality);
        assert_eq!(locality.subordinate_to, None);
        assert_eq!(locality.abbreviated_name, Some("Rio Branco".to_string()));
        assert_eq!(locality.ibge_code, Some("1200401".to_string()));
    }

    #[test]
    fn parse_locality_with_subordinate() {
        let localities =
            Localities::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = LocalityId::new(15321);
        let locality = localities.get(&id).unwrap();

        assert_eq!(locality.subordinate_to, Some(LocalityId::new(2)));
        assert_eq!(locality.locality_type, LocalityType::Village);
    }

    #[test]
    fn parse_invalid_field_count() {
        let invalid = "15321@AC@Terra Indgena@69939810@0@P";
        let result = Localities::from_utf8(invalid.to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::FieldCount { expected, got, .. } => {
                assert_eq!(expected, 9);
                assert_eq!(got, 6);
            }
            _ => panic!("Expected FieldCount error"),
        }
    }

    #[test]
    fn parse_invalid_id() {
        let invalid = "abc@AC@Terra Indgena@69939810@0@P@2@Terra Ind@@";
        let result = Localities::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_uf() {
        let invalid = "15321@ZZ@Terra Indgena@69939810@0@P@2@Terra Ind@@";
        let result = Localities::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_situation() {
        let invalid = "15321@AC@Terra Indgena@69939810@9@P@2@Terra Ind@@";
        let result = Localities::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_type() {
        let invalid = "15321@AC@Terra Indgena@69939810@0@X@2@Terra Ind@@";
        let result = Localities::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_required_field() {
        let invalid = "15321@@Terra Indgena@69939810@0@P@2@Terra Ind@";
        let result = Localities::from_utf8(invalid.to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::EmptyField { field_name, .. } => {
                assert_eq!(field_name, "UFE_SG");
            }
            _ => panic!("Expected EmptyField error"),
        }
    }

    #[test]
    fn localities_iterator() {
        let localities =
            Localities::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let count = localities.iter().count();
        assert_eq!(count, 5);
    }

    #[test]
    fn localities_get_nonexistent() {
        let localities =
            Localities::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let result = localities.get(&LocalityId::new(99999));
        assert!(result.is_none());
    }
}
