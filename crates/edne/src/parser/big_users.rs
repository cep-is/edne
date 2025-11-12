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
        big_user::{BigUser, BigUserId, StreetId},
    },
    parser::base::{EdneParser, ParseError},
};

const BIG_USER_FIELD_COUNT: usize = 9;

#[derive(Debug, Clone)]
pub struct BigUsers(HashMap<BigUserId, BigUser>);

impl BigUsers {
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

    pub fn get(&self, id: &BigUserId) -> Option<&BigUser> {
        self.0.get(id)
    }

    pub fn insert(&mut self, big_user: BigUser) -> Option<BigUser> {
        self.0.insert(big_user.id, big_user)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BigUserId, &BigUser)> {
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
        let mut big_users = Self::with_capacity(lines.len());

        for (line_number, line) in lines {
            let big_user = parse_big_user_line(parser, line, line_number)?;
            big_users.insert(big_user);
        }

        Ok(big_users)
    }
}

impl Default for BigUsers {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_big_user_line(
    parser: &EdneParser,
    line: &str,
    line_number: usize,
) -> Result<BigUser, ParseError> {
    let fields =
        parser.parse_line_checked(line, BIG_USER_FIELD_COUNT, line_number)?;

    let id_str = EdneParser::required_field(fields[0], "GRU_NU", line_number)?;
    let id = BigUserId::from_str(&id_str).map_err(|e| {
        ParseError::InvalidValue {
            field_name: "GRU_NU",
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

    let name = EdneParser::required_field(fields[5], "GRU_NO", line_number)?;
    let address =
        EdneParser::required_field(fields[6], "GRU_ENDERECO", line_number)?;
    let cep = EdneParser::required_field(fields[7], "CEP", line_number)?;
    let abbreviated_name = EdneParser::optional_field(fields[8]);

    Ok(BigUser {
        id,
        uf,
        locality_id,
        neighborhood_id,
        street_id,
        name,
        address,
        cep,
        abbreviated_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "\
41739@AC@16@49922@949512@PCL Ponto de Coleta Mercantil Júnior Clique e Retire@Rua Valdomiro Lopes, 2398 Clique e Retire Correios@69919959@PCL P C M J C Retire
34344@AC@16@55439@948258@Residencial Ecoville@Rodovia BR-364, 2081@69915900@Res Ecoville
33084@AC@18@39333@@AC Santa Rosa do Purus Clique e Retire@Rua Coronel José Ferreira, 1498 Clique e Retire Correios@69955959@AC Sta R P C Retire
33089@AC@19@39330@@AC Sena Madureira Clique e Retire@Rua Dom Júlio Matiolli, 290 Clique e Retire Correios@69940959@AC S M C Retire
32492@AC@20@39324@@AC Senador Guiomard Clique e Retire@Avenida Castelo Branco, 1750 Clique e Retire Correios@69925959@AC Sen G C Retire
33082@AC@21@39335@@AC Tarauacá Clique e Retire@Rua Coronel Juvêncio de Menezes, 158 Clique e Retire Correios@69970959@AC T C Retire
33099@AC@22@39326@@AC Xapuri Clique e Retire@Rua 24 de Janeiro, 270 Clique e Retire Correios@69930959@AC X C Retire
33087@AC@1@39331@@AC Acrelândia Clique e Retire@Avenida Paraná, 296 Clique e Retire Correios@69945959@AC A C Retire
33092@AC@2@39329@@AC Assis Brasil Clique e Retire@Rua Dom Giocondo Maria Grotte, 230 Clique e Retire Correios@69935959@AC A B C Retire
33094@AC@3@39327@@AC Brasiléia Clique e Retire@Avenida Prefeito Rolando Moreira, 170 Clique e Retire Correios@69932959@AC B C Retire
32496@AC@4@39321@@AC Bujari Clique e Retire@Rua Expedito Pereira de Souza, 971 Clique e Retire Correios@69926959@AC B C Retire
33097@AC@5@39323@@AC Capixaba Clique e Retire@Avenida Governador Edmundo Pinto, 711 Clique e Retire Correios@69931959@AC C C Retire
33080@AC@6@39337@@AC Cruzeiro do Sul Clique e Retire@Rua Rego Barros, 73 Clique e Retire Correios@69980959@AC C S C Retire
33093@AC@7@39328@@AC Epitaciolândia Clique e Retire@Avenida Santos Dumont, 160 Clique e Retire Correios@69934959@AC E C Retire
33083@AC@8@39334@@AC Feijó Clique e Retire@Avenida Plácido de Castro, 871 Clique e Retire Correios@69960959@AC F C Retire";

    #[test]
    fn parse_sample_data() {
        let big_users = BigUsers::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        assert_eq!(big_users.len(), 15);
    }

    #[test]
    fn parse_big_user_with_street_id() {
        let big_users = BigUsers::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = BigUserId::new(41739);
        let user = big_users.get(&id).unwrap();

        assert_eq!(user.id, id);
        assert_eq!(user.uf, Uf::AC);
        assert_eq!(user.locality_id, LocalityId::new(16));
        assert_eq!(user.neighborhood_id, NeighborhoodId::new(49922));
        assert_eq!(user.street_id, Some(StreetId::new(949512)));
        assert!(user.name.contains("PCL"));
        assert!(user.address.contains("Rua Valdomiro Lopes"));
        assert_eq!(user.cep, "69919959");
    }

    #[test]
    fn parse_big_user_without_street_id() {
        let big_users = BigUsers::from_utf8(SAMPLE_DATA.to_string()).unwrap();
        let id = BigUserId::new(33084);
        let user = big_users.get(&id).unwrap();

        assert_eq!(user.street_id, None);
        assert!(user.name.contains("Santa Rosa"));
    }

    #[test]
    fn parse_invalid_field_count() {
        let invalid = "41739@AC@16@49922@949512@PCL@Rua Valdomiro@69919959";
        let result = BigUsers::from_utf8(invalid.to_string());
        assert!(result.is_err());
    }
}
