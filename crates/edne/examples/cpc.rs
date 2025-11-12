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

use edne::models::{CpcId, LocalityId};
use edne::parser::cpcs::Cpcs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample data (in real usage, read from file)
    let sample_data = "\
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

    println!("=== eDONE CPCs (Community Postal Boxes) Parser Example ===\n");

    // Parse CPCs
    let cpcs = Cpcs::from_utf8(sample_data.to_string())?;
    println!("✓ Parsed {} CPCs\n", cpcs.len());

    // Example 1: Get specific CPC by ID
    println!("--- Example 1: Get by ID ---");
    let id = CpcId::new(1285);
    if let Some(cpc) = cpcs.get(&id) {
        println!("CPC ID: {}", cpc.id);
        println!("  Name: {}", cpc.name);
        println!("  Address: {}", cpc.address);
        println!("  CEP: {}", cpc.cep);
        println!("  State: {}", cpc.uf);
        println!("  Locality ID: {}", cpc.locality_id);
    }
    println!();

    // Example 2: Filter CPCs by locality
    println!("--- Example 2: CPCs in Locality 158 ---");
    let locality_id = LocalityId::new(158);
    let in_locality_158: Vec<_> =
        cpcs.iter().filter(|(_, c)| c.locality_id == locality_id).collect();

    println!("Found {} CPCs:", in_locality_158.len());
    for (_, cpc) in &in_locality_158 {
        println!("  • {} - {} (CEP: {})", cpc.name, cpc.address, cpc.cep);
    }
    println!();

    // Example 3: Group by UF
    println!("--- Example 3: Group by State ---");
    let mut by_uf = std::collections::HashMap::new();
    for (_, cpc) in cpcs.iter() {
        *by_uf.entry(cpc.uf).or_insert(0) += 1;
    }

    for (uf, count) in &by_uf {
        println!("  {}: {} CPCs", uf, count);
    }
    println!();

    // Example 4: Group by locality
    println!("--- Example 4: CPCs per Locality ---");
    let mut by_locality = std::collections::HashMap::new();
    for (_, cpc) in cpcs.iter() {
        by_locality.entry(cpc.locality_id).or_insert_with(Vec::new).push(cpc);
    }

    println!("Found {} localities with CPCs:", by_locality.len());
    let mut localities: Vec<_> = by_locality.keys().collect();
    localities.sort();

    for locality_id in localities.iter().take(5) {
        let cpc_list = &by_locality[locality_id];
        println!("  Locality {}: {} CPCs", locality_id, cpc_list.len());
        for cpc in cpc_list.iter().take(2) {
            println!("    - {}", cpc.name);
        }
        if cpc_list.len() > 2 {
            println!("    ... and {} more", cpc_list.len() - 2);
        }
    }
    println!();

    // Example 5: Search by name pattern
    println!("--- Example 5: Search by Name Pattern ---");
    let search_term = "Povoado";
    let matching: Vec<_> =
        cpcs.iter().filter(|(_, c)| c.name.contains(search_term)).collect();

    println!("CPCs containing '{}': {}", search_term, matching.len());
    for (_, cpc) in matching.iter().take(5) {
        println!("  • {} (ID: {})", cpc.name, cpc.id);
    }
    println!();

    // Example 6: Search by address pattern
    println!("--- Example 6: Search by Address Pattern ---");
    let address_term = "Rua";
    let with_rua: Vec<_> = cpcs
        .iter()
        .filter(|(_, c)| c.address.contains(address_term))
        .collect();

    println!("CPCs with '{}' in address: {}", address_term, with_rua.len());
    for (_, cpc) in with_rua.iter().take(5) {
        println!("  • {} - {}", cpc.name, cpc.address);
    }
    println!();

    // Example 7: CEP analysis
    println!("--- Example 7: CEP Analysis ---");
    let mut cep_prefixes = std::collections::HashMap::new();
    for (_, cpc) in cpcs.iter() {
        let prefix = &cpc.cep[0..5];
        *cep_prefixes.entry(prefix).or_insert(0) += 1;
    }

    println!("Unique CEP prefixes: {}", cep_prefixes.len());
    let mut prefixes: Vec<_> = cep_prefixes.iter().collect();
    prefixes.sort_by(|a, b| b.1.cmp(a.1));

    println!("Top 5 CEP prefixes:");
    for (prefix, count) in prefixes.iter().take(5) {
        println!("  {}: {} CPCs", prefix, count);
    }
    println!();

    // Example 8: Find CPCs with longest names
    println!("--- Example 8: Longest CPC Names ---");
    let mut sorted_by_name_len: Vec<_> = cpcs.iter().collect();
    sorted_by_name_len.sort_by_key(|(_, c)| std::cmp::Reverse(c.name.len()));

    println!("Top 3 longest names:");
    for (_, cpc) in sorted_by_name_len.iter().take(3) {
        println!("  • \"{}\" ({} chars)", cpc.name, cpc.name.len());
        println!("    Address: {}", cpc.address);
    }
    println!();

    // Example 9: Find CPCs with longest addresses
    println!("--- Example 9: Longest CPC Addresses ---");
    let mut sorted_by_addr_len: Vec<_> = cpcs.iter().collect();
    sorted_by_addr_len
        .sort_by_key(|(_, c)| std::cmp::Reverse(c.address.len()));

    println!("Top 3 longest addresses:");
    for (_, cpc) in sorted_by_addr_len.iter().take(3) {
        println!("  • {} ({} chars)", cpc.name, cpc.address.len());
        println!("    Address: {}", cpc.address);
    }
    println!();

    // Example 10: Statistics
    println!("--- Example 10: Statistics ---");
    let total = cpcs.len();
    let unique_localities = by_locality.len();
    let avg_per_locality = total as f64 / unique_localities as f64;

    let avg_name_len: f64 =
        cpcs.iter().map(|(_, c)| c.name.len()).sum::<usize>() as f64
            / total as f64;
    let avg_addr_len: f64 =
        cpcs.iter().map(|(_, c)| c.address.len()).sum::<usize>() as f64
            / total as f64;

    println!("Total CPCs:              {}", total);
    println!("Unique localities:       {}", unique_localities);
    println!("Average CPCs/locality:   {:.2}", avg_per_locality);
    println!("Average name length:     {:.1} chars", avg_name_len);
    println!("Average address length:  {:.1} chars", avg_addr_len);

    Ok(())
}
