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
        LocalityId, NeighborhoodId, Uf,
        address::{Address, AddressId, StreetTypeIndicator},
    },
    parser::base::{EdneParser, ParseError},
};

const ADDRESS_FIELD_COUNT: usize = 11;

#[derive(Debug, Clone)]
pub struct Addresses(HashMap<AddressId, Address>);

impl Addresses {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, id: &AddressId) -> Option<&Address> {
        self.0.get(id)
    }

    pub fn insert(&mut self, address: Address) -> Option<Address> {
        self.0.insert(address.id, address)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AddressId, &Address)> {
        self.0.iter()
    }

    pub fn from_iso8859_1(bytes: &[u8]) -> Result<Self, ParseError> {
        let parser = EdneParser::from_iso8859_1(bytes)?;
        Self::parse_with_parser(&parser)
    }

    pub fn from_utf8(content: String) -> Result<Self, ParseError> {
        let parser = EdneParser::from_utf8(content);
        Self::parse_with_parser(&parser)
    }

    fn parse_with_parser(parser: &EdneParser) -> Result<Self, ParseError> {
        let lines: Vec<_> = parser.lines().collect();
        let mut addresses = Self::with_capacity(lines.len());

        for (line_number, line) in lines {
            let address = parse_address_line(parser, line, line_number)?;
            addresses.insert(address);
        }

        Ok(addresses)
    }
}

impl Default for Addresses {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_address_line(
    parser: &EdneParser,
    line: &str,
    line_number: usize,
) -> Result<Address, ParseError> {
    let fields =
        parser.parse_line_checked(line, ADDRESS_FIELD_COUNT, line_number)?;

    let id_str = EdneParser::required_field(fields[0], "LOG_NU", line_number)?;
    let id = AddressId::from_str(&id_str).map_err(|e| {
        ParseError::InvalidValue {
            field_name: "LOG_NU",
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

    let bai_ini_str =
        EdneParser::required_field(fields[3], "BAI_NU_INI", line_number)?;
    let neighborhood_id_start = NeighborhoodId::from_str(&bai_ini_str)
        .map_err(|e| ParseError::InvalidValue {
            field_name: "BAI_NU_INI",
            value: bai_ini_str,
            reason: e.to_string(),
            line_number,
        })?;

    let neighborhood_id_end =
        if let Some(bai_fim_str) = EdneParser::optional_field(fields[4]) {
            Some(NeighborhoodId::from_str(&bai_fim_str).map_err(|e| {
                ParseError::InvalidValue {
                    field_name: "BAI_NU_FIM",
                    value: bai_fim_str,
                    reason: e.to_string(),
                    line_number,
                }
            })?)
        } else {
            None
        };

    let name = EdneParser::required_field(fields[5], "LOG_NO", line_number)?;
    let complement = EdneParser::optional_field(fields[6]);
    let cep = EdneParser::required_field(fields[7], "CEP", line_number)?;
    let street_type =
        EdneParser::required_field(fields[8], "TLO_TX", line_number)?;

    let street_type_indicator =
        if let Some(indicator_str) = EdneParser::optional_field(fields[9]) {
            Some(StreetTypeIndicator::from_str(&indicator_str).map_err(
                |e| ParseError::InvalidValue {
                    field_name: "LOG_STA_TLO",
                    value: indicator_str,
                    reason: e.to_string(),
                    line_number,
                },
            )?)
        } else {
            None
        };

    let abbreviated_name = EdneParser::optional_field(fields[10]);

    Ok(Address {
        id,
        uf,
        locality_id,
        neighborhood_id_start,
        neighborhood_id_end,
        name,
        complement,
        cep,
        street_type,
        street_type_indicator,
        abbreviated_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "\
1@AC@16@47@@Nelson Mesquita@@69918703@Rua@S@R Nelson Mesquita
1001866@AC@16@55447@@24 de Dezembro@@69918142@Rua@S@R 24 de Dezembro
1004886@AC@16@32@@Manoel Cezário@@69900816@Travessa@S@Tv Manoel Cezário
1004887@AC@16@55437@@São José@@69915361@Travessa@S@Tv S José
1004888@AC@16@16@@Colombo@@69905027@Beco@S@Bc Colombo
1004889@AC@16@30@@José Pinho@@69915536@Rua@S@R José Pinho
1004890@AC@16@30@@Fátima Maia@@69915572@Rua@S@R Fátima Maia
1004891@AC@16@55480@@Hortencia da Silva@@69922227@Rua@S@R Hortencia da Silva
1004892@AC@16@55480@@Tufi@@69922250@Rua@S@R Tufi
1004893@AC@16@55480@@Flor de Jardim@@69922253@Rua@S@R Flor de Jd
1004894@AC@16@55480@@Raimundo Gomes@@69922256@Rua@S@R Raimundo Gomes
1004895@AC@16@55480@@Aquiles Peret@@69922259@Rua@S@R Aquiles Peret
1004896@AC@16@55422@@11 de Agosto@@69911335@Travessa@S@Tv 11 de Agosto
1004897@AC@16@9@@Santa Inês@@69901314@Beco@S@Bc Sta Inês
1004898@AC@16@55441@@Odim de Aguiar Queiroz@@69917651@Travessa@S@Tv Odim de A Queiroz";

    #[test]
    fn parse_sample_data() {
        let addresses = Addresses::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        assert_eq!(addresses.len(), 15);
    }

    #[test]
    fn parse_address_basic() {
        let addresses = Addresses::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = AddressId::new(1);
        let addr = addresses.get(&id).unwrap();

        assert_eq!(addr.id, id);
        assert_eq!(addr.uf, Uf::AC);
        assert_eq!(addr.locality_id, LocalityId::new(16));
        assert_eq!(addr.neighborhood_id_start, NeighborhoodId::new(47));
        assert_eq!(addr.neighborhood_id_end, None);
        assert_eq!(addr.name, "Nelson Mesquita");
        assert_eq!(addr.complement, None);
        assert_eq!(addr.cep, "69918703");
        assert_eq!(addr.street_type, "Rua");
        assert_eq!(addr.street_type_indicator, Some(StreetTypeIndicator::Yes));
        assert_eq!(
            addr.abbreviated_name,
            Some("R Nelson Mesquita".to_string())
        );
    }

    #[test]
    fn parse_street_types() {
        let addresses = Addresses::from_utf8(SAMPLE_DATA.to_string()).unwrap();

        let rua_count =
            addresses.iter().filter(|(_, a)| a.street_type == "Rua").count();
        let travessa_count = addresses
            .iter()
            .filter(|(_, a)| a.street_type == "Travessa")
            .count();
        let beco_count =
            addresses.iter().filter(|(_, a)| a.street_type == "Beco").count();

        assert_eq!(rua_count, 9);
        assert_eq!(travessa_count, 4);
        assert_eq!(beco_count, 2);
    }

    #[test]
    fn parse_invalid_field_count() {
        let invalid = "1@AC@16@47@@Nelson Mesquita@@69918703@Rua";
        let result = Addresses::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }
}
