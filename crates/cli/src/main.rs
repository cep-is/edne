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

use std::{env, fs, process};

use edne::parser::localities::Localities;
use edne::parser::neighborhoods::Neighborhoods;

enum FileType {
    Locality,
    Neighborhood,
}

fn print_usage(program: &str) {
    eprintln!("Usage: {} <type> <path-to-file>", program);
    eprintln!();
    eprintln!("Types:");
    eprintln!("  locality      Parse LOG_LOCALIDADE.TXT file");
    eprintln!("  neighborhood  Parse LOG_BAIRRO.TXT file");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} locality LOG_LOCALIDADE.TXT", program);
    eprintln!("  {} neighborhood LOG_BAIRRO.TXT", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let file_type = match args[1].to_lowercase().as_str() {
        "locality" | "localidade" => FileType::Locality,
        "neighborhood" | "neighbourhood" | "bairro" => FileType::Neighborhood,
        unknown => {
            eprintln!("Error: Unknown type '{}'", unknown);
            eprintln!();
            print_usage(&args[0]);
            process::exit(1);
        }
    };

    let file_path = &args[2];

    println!("Reading file: {}", file_path);

    let bytes = match fs::read(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    match file_type {
        FileType::Locality => parse_localities(&bytes),
        FileType::Neighborhood => parse_neighborhoods(&bytes),
    }
}

fn parse_localities(bytes: &[u8]) {
    println!("Parsing localities...");

    let localities = match Localities::from_iso8859_1(bytes) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Successfully parsed {} localities", localities.len());
    println!("═══════════════════════════════════════════════════════");
    println!();

    // Print localities grouped by UF
    println!("Localities by State:");
    println!("───────────────────────────────────────────────────────");

    let mut by_uf: std::collections::HashMap<_, Vec<_>> =
        std::collections::HashMap::new();
    for (id, locality) in localities.iter() {
        by_uf.entry(locality.uf).or_default().push((id, locality));
    }

    let mut ufs: Vec<_> = by_uf.keys().collect();
    ufs.sort();

    for uf in ufs {
        let locs = &by_uf[uf];
        println!();
        println!("{} ({} localities)", uf, locs.len());
        println!("───────────────────────────────────────────────────────");

        let mut sorted_locs = locs.clone();
        sorted_locs.sort_by_key(|(id, _)| *id);

        for (id, locality) in sorted_locs.iter().take(10) {
            print!("  [{}] {}", id, locality.name);

            if let Some(cep) = &locality.cep {
                print!(" (CEP: {})", cep);
            }

            print!(" - {:?}", locality.locality_type);

            if let Some(abbrev) = &locality.abbreviated_name {
                print!(" [{}]", abbrev);
            }

            println!();
        }

        if locs.len() > 10 {
            println!("  ... and {} more", locs.len() - 10);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Summary");
    println!("═══════════════════════════════════════════════════════");

    // Count by type
    let mut by_type: std::collections::HashMap<_, usize> =
        std::collections::HashMap::new();
    for (_, locality) in localities.iter() {
        *by_type.entry(locality.locality_type).or_default() += 1;
    }

    println!();
    println!("By Type:");
    println!(
        "  Municipalities: {}",
        by_type.get(&edne::models::LocalityType::Municipality).unwrap_or(&0)
    );
    println!(
        "  Districts:      {}",
        by_type.get(&edne::models::LocalityType::District).unwrap_or(&0)
    );
    println!(
        "  Villages:       {}",
        by_type.get(&edne::models::LocalityType::Village).unwrap_or(&0)
    );

    // Count by situation
    let mut by_situation: std::collections::HashMap<_, usize> =
        std::collections::HashMap::new();
    for (_, locality) in localities.iter() {
        *by_situation.entry(locality.situation).or_default() += 1;
    }

    println!();
    println!("By Situation:");
    println!(
        "  Not Coded:          {}",
        by_situation
            .get(&edne::models::LocalitySituation::NotCoded)
            .unwrap_or(&0)
    );
    println!(
        "  Coded:              {}",
        by_situation
            .get(&edne::models::LocalitySituation::Coded)
            .unwrap_or(&0)
    );
    println!(
        "  District/Village:   {}",
        by_situation
            .get(&edne::models::LocalitySituation::DistrictOrVillage)
            .unwrap_or(&0)
    );
    println!(
        "  Coding in Progress: {}",
        by_situation
            .get(&edne::models::LocalitySituation::CodingInProgress)
            .unwrap_or(&0)
    );

    println!();
}

fn parse_neighborhoods(bytes: &[u8]) {
    println!("Parsing neighborhoods...");

    let neighborhoods = match Neighborhoods::from_iso8859_1(bytes) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Successfully parsed {} neighborhoods", neighborhoods.len());
    println!("═══════════════════════════════════════════════════════");
    println!();

    // Print neighborhoods grouped by UF
    println!("Neighborhoods by State:");
    println!("───────────────────────────────────────────────────────");

    let mut by_uf: std::collections::HashMap<_, Vec<_>> =
        std::collections::HashMap::new();
    for (id, neighborhood) in neighborhoods.iter() {
        by_uf.entry(neighborhood.uf).or_default().push((id, neighborhood));
    }

    let mut ufs: Vec<_> = by_uf.keys().collect();
    ufs.sort();

    for uf in ufs {
        let hoods = &by_uf[uf];
        println!();
        println!("{} ({} neighborhoods)", uf, hoods.len());
        println!("───────────────────────────────────────────────────────");

        let mut sorted_hoods = hoods.clone();
        sorted_hoods.sort_by_key(|(id, _)| *id);

        for (id, neighborhood) in sorted_hoods.iter().take(10) {
            print!("  [{}] {}", id, neighborhood.name);
            print!(" (Locality: {})", neighborhood.locality_id);

            if let Some(abbrev) = &neighborhood.abbreviated_name {
                print!(" [{}]", abbrev);
            }

            println!();
        }

        if hoods.len() > 10 {
            println!("  ... and {} more", hoods.len() - 10);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Summary");
    println!("═══════════════════════════════════════════════════════");

    // Count by locality
    let mut by_locality: std::collections::HashMap<_, usize> =
        std::collections::HashMap::new();
    for (_, neighborhood) in neighborhoods.iter() {
        *by_locality.entry(neighborhood.locality_id).or_default() += 1;
    }

    println!();
    println!("Statistics:");
    println!("  Total neighborhoods:     {}", neighborhoods.len());
    println!("  Localities with hoods:   {}", by_locality.len());

    let with_abbrev = neighborhoods
        .iter()
        .filter(|(_, n)| n.abbreviated_name.is_some())
        .count();
    let without_abbrev = neighborhoods.len() - with_abbrev;

    println!(
        "  With abbreviation:       {} ({:.1}%)",
        with_abbrev,
        (with_abbrev as f64 / neighborhoods.len() as f64) * 100.0
    );
    println!(
        "  Without abbreviation:    {} ({:.1}%)",
        without_abbrev,
        (without_abbrev as f64 / neighborhoods.len() as f64) * 100.0
    );

    // Top localities by neighborhood count
    println!();
    println!("Top 10 localities by neighborhood count:");
    let mut locality_counts: Vec<_> = by_locality.iter().collect();
    locality_counts.sort_by(|a, b| b.1.cmp(a.1));

    for (locality_id, count) in locality_counts.iter().take(10) {
        println!("  Locality {}: {} neighborhoods", locality_id, count);
    }

    println!();
}
