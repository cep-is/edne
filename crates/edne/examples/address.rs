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

use edne::models::AddressId;
use edne::parser::addresses::Addresses;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_data = "\
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

    println!("=== eDNE Addresses Parser Example ===\n");

    let addresses = Addresses::from_utf8(sample_data.to_string())?;
    println!("✓ Parsed {} addresses\n", addresses.len());

    println!("--- Example 1: Get by ID ---");
    let id = AddressId::new(1);
    if let Some(addr) = addresses.get(&id) {
        println!("Address ID: {}", addr.id);
        println!("  Name: {}", addr.name);
        println!("  Type: {}", addr.street_type);
        println!("  CEP: {}", addr.cep);
        println!("  Neighborhood (start): {}", addr.neighborhood_id_start);
        if let Some(abbrev) = &addr.abbreviated_name {
            println!("  Abbreviated: {}", abbrev);
        }
    }
    println!();

    println!("--- Example 2: Group by Street Type ---");
    let mut by_type = std::collections::HashMap::new();
    for (_, addr) in addresses.iter() {
        *by_type.entry(&addr.street_type).or_insert(0) += 1;
    }

    let mut types: Vec<_> = by_type.iter().collect();
    types.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (street_type, count) in types {
        println!("  {}: {} addresses", street_type, count);
    }
    println!();

    println!("--- Example 3: Search by Name Pattern ---");
    let search_term = "São";
    let matching: Vec<_> = addresses
        .iter()
        .filter(|(_, a)| a.name.contains(search_term))
        .collect();
    println!("Addresses containing '{}': {}", search_term, matching.len());
    for (_, addr) in matching {
        println!("  • {} {} - CEP: {}", addr.street_type, addr.name, addr.cep);
    }
    println!();

    println!("--- Example 4: Addresses by Neighborhood ---");
    let mut by_neighborhood = std::collections::HashMap::new();
    for (_, addr) in addresses.iter() {
        *by_neighborhood.entry(addr.neighborhood_id_start).or_insert(0) += 1;
    }

    let mut neighborhoods: Vec<_> = by_neighborhood.iter().collect();
    neighborhoods.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    println!("Top 5 neighborhoods by address count:");
    for (hood_id, count) in neighborhoods.iter().take(5) {
        println!("  Neighborhood {}: {} addresses", hood_id, count);
    }
    println!();

    println!("--- Example 5: Addresses with Range ---");
    let with_range: Vec<_> = addresses
        .iter()
        .filter(|(_, a)| a.neighborhood_id_end.is_some())
        .collect();
    println!(
        "Addresses spanning multiple neighborhoods: {}",
        with_range.len()
    );
    println!();

    println!("--- Example 6: Statistics ---");
    let with_complement =
        addresses.iter().filter(|(_, a)| a.complement.is_some()).count();
    let with_abbrev =
        addresses.iter().filter(|(_, a)| a.abbreviated_name.is_some()).count();

    println!("Total addresses: {}", addresses.len());
    println!(
        "With complement: {} ({:.1}%)",
        with_complement,
        (with_complement as f64 / addresses.len() as f64) * 100.0
    );
    println!(
        "With abbreviation: {} ({:.1}%)",
        with_abbrev,
        (with_abbrev as f64 / addresses.len() as f64) * 100.0
    );

    Ok(())
}
