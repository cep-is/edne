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
        LocalityId, NeighborhoodId, StreetId, Uf,
        operational_unit::{
            OperationalUnit, OperationalUnitId, PostBoxIndicator,
        },
    },
    parser::base::{EdneParser, ParseError},
};

const OPERATIONAL_UNIT_FIELD_COUNT: usize = 10;

#[derive(Debug, Clone)]
pub struct OperationalUnits(HashMap<OperationalUnitId, OperationalUnit>);

impl OperationalUnits {
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

    pub fn get(&self, id: &OperationalUnitId) -> Option<&OperationalUnit> {
        self.0.get(id)
    }

    pub fn insert(
        &mut self,
        unit: OperationalUnit,
    ) -> Option<OperationalUnit> {
        self.0.insert(unit.id, unit)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&OperationalUnitId, &OperationalUnit)> {
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
        let mut units = Self::with_capacity(lines.len());

        for (line_number, line) in lines {
            let unit = parse_operational_unit_line(parser, line, line_number)?;
            units.insert(unit);
        }

        Ok(units)
    }
}

impl Default for OperationalUnits {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_operational_unit_line(
    parser: &EdneParser,
    line: &str,
    line_number: usize,
) -> Result<OperationalUnit, ParseError> {
    let fields = parser.parse_line_checked(
        line,
        OPERATIONAL_UNIT_FIELD_COUNT,
        line_number,
    )?;

    let id_str = EdneParser::required_field(fields[0], "UOP_NU", line_number)?;
    let id = OperationalUnitId::from_str(&id_str).map_err(|e| {
        ParseError::InvalidValue {
            field_name: "UOP_NU",
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

    let bai_id_str =
        EdneParser::required_field(fields[3], "BAI_NU", line_number)?;
    let neighborhood_id =
        NeighborhoodId::from_str(&bai_id_str).map_err(|e| {
            ParseError::InvalidValue {
                field_name: "BAI_NU",
                value: bai_id_str,
                reason: e.to_string(),
                line_number,
            }
        })?;

    let street_id =
        if let Some(log_id_str) = EdneParser::optional_field(fields[4]) {
            Some(StreetId::from_str(&log_id_str).map_err(|e| {
                ParseError::InvalidValue {
                    field_name: "LOG_NU",
                    value: log_id_str,
                    reason: e.to_string(),
                    line_number,
                }
            })?)
        } else {
            None
        };

    let name = EdneParser::required_field(fields[5], "UOP_NO", line_number)?;
    let address =
        EdneParser::required_field(fields[6], "UOP_ENDERECO", line_number)?;
    let cep = EdneParser::required_field(fields[7], "CEP", line_number)?;

    let indicator_str =
        EdneParser::required_field(fields[8], "UOP_IN_CP", line_number)?;
    let post_box_indicator = PostBoxIndicator::from_str(&indicator_str)
        .map_err(|e| ParseError::InvalidValue {
            field_name: "UOP_IN_CP",
            value: indicator_str,
            reason: e.to_string(),
            line_number,
        })?;

    let abbreviated_name = EdneParser::optional_field(fields[9]);

    Ok(OperationalUnit {
        id,
        uf,
        locality_id,
        neighborhood_id,
        street_id,
        name,
        address,
        cep,
        post_box_indicator,
        abbreviated_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "\
48437@AC@11059@51784@@AGC Campinas@Rua Kaxinawás, s/n@69929970@N@AGC Campinas
11986@AC@5@39323@@AC Capixaba@Avenida Governador Edmundo Pinto, 711@69931970@N@AC Capixaba
34293@AC@6@39337@@CDD Cruzeiro do Sul@Rua Rego Barros, 73@69980972@N@CDD Cruzeiro Sul
12037@AC@7@39328@@AC Epitaciolândia@Avenida Santos Dumont, 160@69934970@N@AC Epitaciolândia
12043@AC@8@39334@@AC Feijó@Avenida Plácido de Castro, 871@69960970@N@AC Feijó
12045@AC@9@39336@@AC Jordão@Rua Romildo Magalhães, s/n@69975970@N@AC Jordão
12048@AC@12@39339@@AC Marechal Thaumaturgo@Rua 5 de Novembro, 125@69983970@N@AC Mal Thaumaturgo
11988@AC@13@39325@@AC Plácido de Castro@Avenida Diamantino Augusto de Macedo, 580@69928970@N@AC Plácido Castro
11985@AC@14@39322@@AC Porto Acre@Rua Margaridas, 131@69927970@N@AC Pto Acre
12047@AC@15@39338@@AC Porto Walter@Rua Projetada, s/n@69982970@N@AC Pto Walter
1@AC@16@17@948034@AC Rio Branco@Avenida Epaminondas Jácome, 2858@69900970@S@AC Rio Branco
25740@AC@16@17@814@AC Oca@Rua Quintino Bocaiúva, 299@69900974@N@AC Oca
24821@AC@16@55445@950232@CDD Bosque@Avenida Ceará, 3607@69900973@N@CDD Bosque
60183@AC@16@49922@949512@PCL Ponto de Coleta Mercantil Junior@Rua Valdomiro Lopes, 2398@69919970@N@PCL Ponto C M Junior
5@AC@16@10@950390@CDD Rio Branco@Rua Floriano Peixoto, 411@69900971@N@CDD Rio Branco";

    #[test]
    fn parse_sample_data() {
        let units =
            OperationalUnits::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        assert_eq!(units.len(), 15);
    }

    #[test]
    fn parse_unit_with_street_id() {
        let units =
            OperationalUnits::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = OperationalUnitId::new(1);
        let unit = units.get(&id).unwrap();

        assert_eq!(unit.id, id);
        assert_eq!(unit.street_id, Some(StreetId::new(948034)));
        assert_eq!(unit.post_box_indicator, PostBoxIndicator::Yes);
    }

    #[test]
    fn parse_unit_without_street_id() {
        let units =
            OperationalUnits::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = OperationalUnitId::new(48437);
        let unit = units.get(&id).unwrap();

        assert_eq!(unit.street_id, None);
        assert_eq!(unit.post_box_indicator, PostBoxIndicator::No);
    }

    #[test]
    fn parse_invalid_field_count() {
        let invalid = "48437@AC@11059@51784@@AGC Campinas@69929970@N";
        let result = OperationalUnits::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }
}
