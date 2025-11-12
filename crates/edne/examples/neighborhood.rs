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

use edne::models::{LocalityId, NeighborhoodId, Uf};
use edne::parser::neighborhoods::Neighborhoods;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample data (in real usage, read from file)
    let sample_data = "\
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

    println!("=== eDNE Neighborhoods Parser Example ===\n");

    // Parse neighborhoods
    let neighborhoods = Neighborhoods::from_utf8(sample_data.to_string())?;
    println!("✓ Parsed {} neighborhoods\n", neighborhoods.len());

    // Example 1: Get specific neighborhood by ID
    println!("--- Example 1: Get by ID ---");
    let id = NeighborhoodId::new(55400);
    if let Some(neighborhood) = neighborhoods.get(&id) {
        println!("Neighborhood ID: {}", neighborhood.id);
        println!("  Name: {}", neighborhood.name);
        println!("  State: {}", neighborhood.uf);
        println!("  Locality ID: {}", neighborhood.locality_id);
        if let Some(abbrev) = &neighborhood.abbreviated_name {
            println!("  Abbreviated: {}", abbrev);
        }
    }
    println!();

    // Example 2: Filter neighborhoods by locality
    println!("--- Example 2: Neighborhoods in Locality 16 ---");
    let locality_id = LocalityId::new(16);
    let in_locality_16: Vec<_> = neighborhoods
        .iter()
        .filter(|(_, n)| n.locality_id == locality_id)
        .collect();

    println!("Found {} neighborhoods:", in_locality_16.len());
    for (_, neighborhood) in &in_locality_16 {
        println!("  • {} (ID: {})", neighborhood.name, neighborhood.id);
        if let Some(abbrev) = &neighborhood.abbreviated_name {
            println!("    Abbreviated: {}", abbrev);
        }
    }
    println!();

    // Example 3: Find all neighborhoods named "Centro"
    println!("--- Example 3: All 'Centro' Neighborhoods ---");
    let centro_neighborhoods: Vec<_> =
        neighborhoods.iter().filter(|(_, n)| n.name == "Centro").collect();

    println!("Found {} 'Centro' neighborhoods:", centro_neighborhoods.len());
    for (_, neighborhood) in &centro_neighborhoods {
        println!(
            "  • ID: {}, Locality: {}",
            neighborhood.id, neighborhood.locality_id
        );
    }
    println!();

    // Example 4: Group by UF
    println!("--- Example 4: Group by State ---");
    let mut by_uf = std::collections::HashMap::new();
    for (_, neighborhood) in neighborhoods.iter() {
        *by_uf.entry(neighborhood.uf).or_insert(0) += 1;
    }

    for (uf, count) in &by_uf {
        println!("  {}: {} neighborhoods", uf, count);
    }
    println!();

    // Example 5: Group by locality
    println!("--- Example 5: Group by Locality ---");
    let mut by_locality = std::collections::HashMap::new();
    for (_, neighborhood) in neighborhoods.iter() {
        by_locality
            .entry(neighborhood.locality_id)
            .or_insert_with(Vec::new)
            .push(neighborhood);
    }

    println!("Found {} localities with neighborhoods:", by_locality.len());
    let mut localities: Vec<_> = by_locality.keys().collect();
    localities.sort();

    for locality_id in localities.iter().take(5) {
        let hoods = &by_locality[locality_id];
        println!("  Locality {}: {} neighborhoods", locality_id, hoods.len());
        for hood in hoods.iter().take(3) {
            println!("    - {}", hood.name);
        }
        if hoods.len() > 3 {
            println!("    ... and {} more", hoods.len() - 3);
        }
    }
    println!();

    // Example 6: Find neighborhoods with same name/abbrev
    println!(
        "--- Example 6: Neighborhoods with Same Name and Abbreviation ---"
    );
    let same_name_abbrev: Vec<_> = neighborhoods
        .iter()
        .filter(|(_, n)| {
            if let Some(abbrev) = &n.abbreviated_name {
                &n.name == abbrev
            } else {
                false
            }
        })
        .collect();

    println!(
        "Found {} neighborhoods where name equals abbreviation:",
        same_name_abbrev.len()
    );
    for (_, neighborhood) in same_name_abbrev.iter().take(5) {
        println!("  • {} (ID: {})", neighborhood.name, neighborhood.id);
    }
    println!();

    // Example 7: Neighborhoods without abbreviation
    println!("--- Example 7: Without Abbreviation ---");
    let without_abbrev: Vec<_> = neighborhoods
        .iter()
        .filter(|(_, n)| n.abbreviated_name.is_none())
        .collect();

    println!(
        "Found {} neighborhoods without abbreviation",
        without_abbrev.len()
    );
    println!();

    // Example 8: Statistics
    println!("--- Example 8: Statistics ---");
    let total = neighborhoods.len();
    let with_abbrev = neighborhoods
        .iter()
        .filter(|(_, n)| n.abbreviated_name.is_some())
        .count();
    let without_abbrev_count = total - with_abbrev;

    println!("Total neighborhoods: {}", total);
    println!(
        "  With abbreviation:    {} ({:.1}%)",
        with_abbrev,
        (with_abbrev as f64 / total as f64) * 100.0
    );
    println!(
        "  Without abbreviation: {} ({:.1}%)",
        without_abbrev_count,
        (without_abbrev_count as f64 / total as f64) * 100.0
    );
    println!();

    // Example 9: Find longest neighborhood name
    println!("--- Example 9: Longest Neighborhood Name ---");
    if let Some((_, longest)) =
        neighborhoods.iter().max_by_key(|(_, n)| n.name.len())
    {
        println!(
            "Longest name: \"{}\" ({} characters)",
            longest.name,
            longest.name.len()
        );
        println!("  ID: {}", longest.id);
        println!("  Locality: {}", longest.locality_id);
    }
    println!();

    // Example 10: Search by name pattern
    println!("--- Example 10: Search by Name Pattern ---");
    let search_term = "Baixada";
    let matching: Vec<_> = neighborhoods
        .iter()
        .filter(|(_, n)| n.name.contains(search_term))
        .collect();

    println!("Neighborhoods containing '{}': {}", search_term, matching.len());
    for (_, neighborhood) in matching {
        println!("  • {} (ID: {})", neighborhood.name, neighborhood.id);
    }

    Ok(())
}
