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

use edne::models::BigUserId;
use edne::parser::big_users::BigUsers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_data = "\
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

    println!("=== eDNE Big Users Parser Example ===\n");

    let big_users = BigUsers::from_utf8(sample_data.to_string())?;
    println!("✓ Parsed {} big users\n", big_users.len());

    println!("--- Example 1: Get by ID ---");
    let id = BigUserId::new(41739);
    if let Some(user) = big_users.get(&id) {
        println!("Big User ID: {}", user.id);
        println!("  Name: {}", user.name);
        println!("  Address: {}", user.address);
        println!("  CEP: {}", user.cep);
        println!("  State: {}", user.uf);
        println!("  Locality ID: {}", user.locality_id);
        println!("  Neighborhood ID: {}", user.neighborhood_id);
        if let Some(street_id) = user.street_id {
            println!("  Street ID: {}", street_id);
        }
        if let Some(abbrev) = &user.abbreviated_name {
            println!("  Abbreviated: {}", abbrev);
        }
    }
    println!();

    println!("--- Example 2: Users with Street ID ---");
    let with_street: Vec<_> =
        big_users.iter().filter(|(_, u)| u.street_id.is_some()).collect();
    println!("Found {} users with street ID", with_street.len());
    for (_, user) in with_street.iter().take(3) {
        println!("  • {} - Street ID: {}", user.name, user.street_id.unwrap());
    }
    println!();

    println!("--- Example 3: Users without Street ID ---");
    let without_street: Vec<_> =
        big_users.iter().filter(|(_, u)| u.street_id.is_none()).collect();
    println!("Found {} users without street ID", without_street.len());
    for (_, user) in without_street.iter().take(3) {
        println!("  • {} - {}", user.name, user.address);
    }
    println!();

    println!("--- Example 4: Search 'Clique e Retire' ---");
    let clique_retire: Vec<_> = big_users
        .iter()
        .filter(|(_, u)| u.name.contains("Clique e Retire"))
        .collect();
    println!("Found {}", clique_retire.len());
    println!();

    println!("--- Example 5: Group by Locality ---");
    let mut by_locality = std::collections::HashMap::new();
    for (_, user) in big_users.iter() {
        *by_locality.entry(user.locality_id).or_insert(0) += 1;
    }
    println!("Localities with big users: {}", by_locality.len());
    println!();

    println!("--- Example 6: Statistics ---");
    let with_street =
        big_users.iter().filter(|(_, u)| u.street_id.is_some()).count();
    let without_street = big_users.len() - with_street;
    println!("Total: {}", big_users.len());
    println!(
        "With street ID: {} ({:.1}%)",
        with_street,
        (with_street as f64 / big_users.len() as f64) * 100.0
    );
    println!(
        "Without street ID: {} ({:.1}%)",
        without_street,
        (without_street as f64 / big_users.len() as f64) * 100.0
    );

    Ok(())
}
