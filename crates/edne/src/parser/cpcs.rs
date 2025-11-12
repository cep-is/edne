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
        cpc::{Cpc, CpcId},
    },
    parser::base::{EdneParser, ParseError},
};

/// Expected number of fields in a CPC record.
const CPC_FIELD_COUNT: usize = 6;

/// Collection of Community Postal Boxes indexed by their ID.
#[derive(Debug, Clone)]
pub struct Cpcs(HashMap<CpcId, Cpc>);

impl Cpcs {
    /// Creates a new empty collection.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates a collection with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns the number of CPCs.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets a CPC by ID.
    pub fn get(&self, id: &CpcId) -> Option<&Cpc> {
        self.0.get(id)
    }

    /// Inserts a CPC into the collection.
    pub fn insert(&mut self, cpc: Cpc) -> Option<Cpc> {
        self.0.insert(cpc.id, cpc)
    }

    /// Returns an iterator over all CPCs.
    pub fn iter(&self) -> impl Iterator<Item = (&CpcId, &Cpc)> {
        self.0.iter()
    }

    /// Parses CPCs from ISO-8859-1 encoded bytes.
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

    /// Parses CPCs from UTF-8 string (for testing).
    pub fn from_utf8(content: String) -> Result<Self, ParseError> {
        let parser = EdneParser::from_utf8(content);
        Self::parse_with_parser(&parser)
    }

    /// Internal method to parse CPCs using a configured parser.
    fn parse_with_parser(parser: &EdneParser) -> Result<Self, ParseError> {
        let lines: Vec<_> = parser.lines().collect();
        let mut cpcs = Self::with_capacity(lines.len());

        for (line_number, line) in lines {
            let cpc = parse_cpc_line(parser, line, line_number)?;
            cpcs.insert(cpc);
        }

        Ok(cpcs)
    }
}

impl Default for Cpcs {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a single CPC line into a `Cpc` struct.
///
/// # Field order (6 fields):
/// 1. CPC_NU - CPC ID
/// 2. UFE_SG - UF code
/// 3. LOC_NU - Locality ID
/// 4. CPC_NO - CPC name
/// 5. CPC_ENDERECO - CPC address
/// 6. CEP - Postal code
fn parse_cpc_line(
    parser: &EdneParser,
    line: &str,
    line_number: usize,
) -> Result<Cpc, ParseError> {
    let fields =
        parser.parse_line_checked(line, CPC_FIELD_COUNT, line_number)?;

    // Parse required fields
    let id_str = EdneParser::required_field(fields[0], "CPC_NU", line_number)?;
    let id =
        CpcId::from_str(&id_str).map_err(|e| ParseError::InvalidValue {
            field_name: "CPC_NU",
            value: id_str,
            reason: e.to_string(),
            line_number,
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

    let name = EdneParser::required_field(fields[3], "CPC_NO", line_number)?;
    let address =
        EdneParser::required_field(fields[4], "CPC_ENDERECO", line_number)?;
    let cep = EdneParser::required_field(fields[5], "CEP", line_number)?;

    Ok(Cpc { id, uf, locality_id, name, address, cep })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "\
1285@AL@158@Conjunto Mutiro@Quadra 1 n 37 - Conj.Mutiro - Rio Largo@57100990
3788@AL@158@Utinga Leo@Rua do Hospital s/n@57100993
4162@AL@184@Gulandim@Povoado Gulandim@57265990
4191@AL@144@Pontal do Peba@Povoado Pontal do Peba@57210990
4195@AL@145@Mangabeiras@Povoado Mangabeiras@57150990
4197@AL@30@Pau D'Arco@Povoado Pau D'Arco@57319990
4199@AL@31@Vila Jos Paulino@Rua Professor Genrio Cardoso s/n@57690990
4201@AL@31@Alto do Cruzeiro@Rua Joaquim Vieira 27@57690991
4203@AL@143@Tabuleiro dos Negros@Povoado Tabuleiro dos Negros@57200990
4204@AL@143@Marituba do Peixe@Povoado Marituba do Peixe@57200991
4205@AL@143@Ponta Morfina@Povoado Ponta Morfina@57200992
4381@AL@169@Povoado Quitunde@Escola Monteiro Lobato - Povoado Quitunde@57920990
5469@AL@169@Alto Cristo Redentor@Rua George Jos da Silva s/n@57920991
5470@AL@31@Povoado Genipapeiro@Rua Manoel Francisco, s/n@57690992
5471@AL@71@Usina Guaxuma@Avenida Gois n 11 - Usina Guaxuma@57230991";

    #[test]
    fn parse_sample_data() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        assert_eq!(cpcs.len(), 15);
    }

    #[test]
    fn parse_cpc_with_all_fields() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = CpcId::new(1285);
        let cpc = cpcs.get(&id).unwrap();

        assert_eq!(cpc.id, id);
        assert_eq!(cpc.uf, Uf::AL);
        assert_eq!(cpc.locality_id, LocalityId::new(158));
        assert_eq!(cpc.name, "Conjunto Mutiro");
        assert_eq!(cpc.address, "Quadra 1 n 37 - Conj.Mutiro - Rio Largo");
        assert_eq!(cpc.cep, "57100990");
    }

    #[test]
    fn parse_cpc_with_complex_address() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = CpcId::new(4381);
        let cpc = cpcs.get(&id).unwrap();

        assert_eq!(cpc.name, "Povoado Quitunde");
        assert_eq!(cpc.address, "Escola Monteiro Lobato - Povoado Quitunde");
    }

    #[test]
    fn parse_cpcs_same_locality() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let loc_158_count = cpcs
            .iter()
            .filter(|(_, c)| c.locality_id == LocalityId::new(158))
            .count();
        assert_eq!(loc_158_count, 2);
    }

    #[test]
    fn parse_cpcs_by_uf() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let al_count = cpcs.iter().filter(|(_, c)| c.uf == Uf::AL).count();
        assert_eq!(al_count, 15);
    }

    #[test]
    fn parse_cpc_cep_format() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = CpcId::new(1285);
        let cpc = cpcs.get(&id).unwrap();
        assert_eq!(cpc.cep.len(), 8);
        assert!(cpc.cep.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn parse_invalid_field_count() {
        let invalid = "1285@AL@158@Conjunto Mutiro@Quadra 1";
        let result = Cpcs::from_utf8(invalid.to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::FieldCount { expected, got, .. } => {
                assert_eq!(expected, 6);
                assert_eq!(got, 5);
            }
            _ => panic!("Expected FieldCount error"),
        }
    }

    #[test]
    fn parse_invalid_id() {
        let invalid = "abc@AL@158@Conjunto Mutiro@Quadra 1 n 37@57100990";
        let result = Cpcs::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_uf() {
        let invalid = "1285@ZZ@158@Conjunto Mutiro@Quadra 1 n 37@57100990";
        let result = Cpcs::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_locality_id() {
        let invalid = "1285@AL@abc@Conjunto Mutiro@Quadra 1 n 37@57100990";
        let result = Cpcs::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_required_field() {
        let invalid = "1285@AL@158@@Quadra 1 n 37@57100990";
        let result = Cpcs::from_utf8(invalid.to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::EmptyField { field_name, .. } => {
                assert_eq!(field_name, "CPC_NO");
            }
            _ => panic!("Expected EmptyField error"),
        }
    }

    #[test]
    fn cpcs_iterator() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let count = cpcs.iter().count();
        assert_eq!(count, 15);
    }

    #[test]
    fn cpcs_get_nonexistent() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let result = cpcs.get(&CpcId::new(99999));
        assert!(result.is_none());
    }

    #[test]
    fn parse_cpcs_multiple_in_same_locality() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let loc_31 = cpcs
            .iter()
            .filter(|(_, c)| c.locality_id == LocalityId::new(31))
            .count();
        assert_eq!(loc_31, 3);
    }

    #[test]
    fn parse_cpc_address_with_special_chars() {
        let cpcs = Cpcs::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = CpcId::new(5470);
        let cpc = cpcs.get(&id).unwrap();
        assert!(cpc.address.contains(','));
    }
}
