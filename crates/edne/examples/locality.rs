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

use edne::models::{LocalityId, LocalitySituation, LocalityType, Uf};
use edne::parser::localities::Localities;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample data (in real usage, read from file)
    let sample_data = "\
15321@AC@Terra Indígena Mamoadate@69939810@0@P@2@Terra Ind Mamoadate@
13@AC@Plácido de Castro@69928000@0@M@@Plácido Castro@1200385
15323@AC@Terra Indígena Kampa e Isolados do Rio Envira@69969820@0@P@8@Terra Ind K I R Envira@
16@AC@Rio Branco@@1@M@@Rio Branco@1200401
12@AC@Marechal Thaumaturgo@69983000@0@M@@Mal Thaumaturgo@1200351
5@AC@Capixaba@69931000@0@M@@Capixaba@1200179
6@AC@Cruzeiro do Sul@69980000@0@M@@Cruzeiro Sul@1200203";

    println!("=== eDNE Localities Parser Example ===\n");

    // Parse localities
    let localities = Localities::from_utf8(sample_data.to_string())?;
    println!("✓ Parsed {} localities\n", localities.len());

    // Example 1: Get specific locality by ID
    println!("--- Example 1: Get by ID ---");
    let id = LocalityId::new(13);
    if let Some(locality) = localities.get(&id) {
        println!("Locality ID: {}", locality.id);
        println!("  Name: {}", locality.name);
        println!("  State: {}", locality.uf);
        println!("  Type: {:?}", locality.locality_type);
        println!("  Situation: {:?}", locality.situation);
        if let Some(cep) = &locality.cep {
            println!("  CEP: {}", cep);
        }
        if let Some(ibge) = &locality.ibge_code {
            println!("  IBGE Code: {}", ibge);
        }
    }
    println!();

    // Example 2: Filter municipalities
    println!("--- Example 2: Municipalities in AC ---");
    let municipalities: Vec<_> = localities
        .iter()
        .filter(|(_, loc)| {
            loc.uf == Uf::AC && loc.locality_type == LocalityType::Municipality
        })
        .collect();

    println!("Found {} municipalities:", municipalities.len());
    for (_, locality) in &municipalities {
        println!("  • {} (ID: {})", locality.name, locality.id);
    }
    println!();

    // Example 3: Find localities with subordinate relationships
    println!("--- Example 3: Subordinate Localities ---");
    let subordinates: Vec<_> = localities
        .iter()
        .filter(|(_, loc)| loc.subordinate_to.is_some())
        .collect();

    println!("Found {} subordinate localities:", subordinates.len());
    for (_, locality) in &subordinates {
        if let Some(parent_id) = locality.subordinate_to {
            println!(
                "  • {} → subordinate to locality ID {}",
                locality.name, parent_id
            );
        }
    }
    println!();

    // Example 4: Group by situation
    println!("--- Example 4: Group by Situation ---");
    let mut by_situation = std::collections::HashMap::new();
    for (_, locality) in localities.iter() {
        *by_situation.entry(locality.situation).or_insert(0) += 1;
    }

    for (situation, count) in &by_situation {
        println!("  {:?}: {} localities", situation, count);
    }
    println!();

    // Example 5: Localities without CEP (coded at street level)
    println!("--- Example 5: Coded Localities (no CEP) ---");
    let coded: Vec<_> = localities
        .iter()
        .filter(|(_, loc)| loc.situation == LocalitySituation::Coded)
        .collect();

    println!("Found {} coded localities:", coded.len());
    for (_, locality) in &coded {
        println!("  • {} ({})", locality.name, locality.uf);
    }
    println!();

    // Example 6: Abbreviated names
    println!("--- Example 6: Name Abbreviations ---");
    let with_abbrev: Vec<_> = localities
        .iter()
        .filter(|(_, loc)| loc.abbreviated_name.is_some())
        .take(3)
        .collect();

    println!("Sample abbreviations:");
    for (_, locality) in &with_abbrev {
        if let Some(abbrev) = &locality.abbreviated_name {
            println!("  • {} → {}", locality.name, abbrev);
        }
    }

    Ok(())
}
