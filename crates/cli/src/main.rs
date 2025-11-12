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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-locality-file>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} LOG_LOCALIDADE.TXT", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    println!("Reading file: {}", file_path);

    let bytes = match fs::read(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    println!("Parsing localities...");

    let localities = match Localities::from_iso8859_1(&bytes) {
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
