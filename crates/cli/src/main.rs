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

mod cep_lookup;

use cep_lookup::{CepInfo, CepLookupBuilder, CepType};
use edne::parser::{
    addresses::Addresses, big_users::BigUsers, cpcs::Cpcs,
    localities::Localities, neighborhoods::Neighborhoods,
    operational_units::OperationalUnits,
};
use std::{env, fs, path::Path, process};

enum Command {
    Parse(FileType, String),
    BuildIndex(String),
    Lookup(String, String),
}

enum FileType {
    Locality,
    Neighborhood,
    Cpc,
    BigUser,
    OperationalUnit,
    Address,
}

fn print_usage(program: &str) {
    eprintln!("Usage: {} <command> [args]", program);
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  Parse single file:");
    eprintln!("    {} <type> <path-to-file>", program);
    eprintln!();
    eprintln!("  Build CEP lookup index:");
    eprintln!("    {} build-index <data-directory>", program);
    eprintln!();
    eprintln!("  Lookup CEP:");
    eprintln!("    {} lookup <data-directory> <cep>", program);
    eprintln!();
    eprintln!("Types:");
    eprintln!("  locality      Parse LOG_LOCALIDADE.TXT file");
    eprintln!("  neighborhood  Parse LOG_BAIRRO.TXT file");
    eprintln!("  cpc           Parse LOG_CPC.TXT file");
    eprintln!("  biguser       Parse LOG_GRANDE_USUARIO.TXT file");
    eprintln!("  opunit        Parse LOG_UNID_OPER.TXT file");
    eprintln!("  address       Parse LOG_LOGRADOURO_XX.TXT file");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} locality LOG_LOCALIDADE.TXT", program);
    eprintln!("  {} build-index data", program);
    eprintln!("  {} lookup data 69918703", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let command = parse_command(&args);

    match command {
        Command::Parse(file_type, file_path) => {
            parse_file(file_type, &file_path);
        }
        Command::BuildIndex(data_dir) => {
            build_index(&data_dir);
        }
        Command::Lookup(data_dir, cep) => {
            lookup_cep(&data_dir, &cep);
        }
    }
}

fn parse_command(args: &[String]) -> Command {
    match args[1].to_lowercase().as_str() {
        "build-index" => {
            if args.len() != 3 {
                eprintln!("Error: build-index requires data directory");
                eprintln!();
                print_usage(&args[0]);
                process::exit(1);
            }
            Command::BuildIndex(args[2].clone())
        }
        "lookup" => {
            if args.len() != 4 {
                eprintln!("Error: lookup requires data directory and CEP");
                eprintln!();
                print_usage(&args[0]);
                process::exit(1);
            }
            Command::Lookup(args[2].clone(), args[3].clone())
        }
        type_str => {
            if args.len() != 3 {
                print_usage(&args[0]);
                process::exit(1);
            }

            let file_type = match type_str {
                "locality" | "localidade" => FileType::Locality,
                "neighborhood" | "neighbourhood" | "bairro" => {
                    FileType::Neighborhood
                }
                "cpc" => FileType::Cpc,
                "biguser" | "big-user" | "grande-usuario"
                | "grandeusuario" => FileType::BigUser,
                "opunit"
                | "operational-unit"
                | "unidade-operacional"
                | "unidadeoperacional" => FileType::OperationalUnit,
                "address" | "logradouro" | "street" => FileType::Address,
                unknown => {
                    eprintln!("Error: Unknown command or type '{}'", unknown);
                    eprintln!();
                    print_usage(&args[0]);
                    process::exit(1);
                }
            };

            Command::Parse(file_type, args[2].clone())
        }
    }
}

fn parse_file(file_type: FileType, file_path: &str) {
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
        FileType::Cpc => parse_cpcs(&bytes),
        FileType::BigUser => parse_big_users(&bytes),
        FileType::OperationalUnit => parse_operational_units(&bytes),
        FileType::Address => parse_addresses(&bytes),
    }
}

fn build_index(data_dir: &str) {
    println!("Building CEP index from: {}", data_dir);
    println!();

    let lookup = match build_cep_lookup(data_dir) {
        Ok(lookup) => lookup,
        Err(e) => {
            eprintln!("Error building index: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Index built successfully!");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("Total CEPs indexed: {}", lookup.len());
    println!();
    println!("Statistics by UF:");
    println!("───────────────────────────────────────────────────────");

    let mut by_uf: std::collections::HashMap<_, usize> =
        std::collections::HashMap::new();
    for uf in edne::models::Uf::iter() {
        let count = lookup.by_uf(uf).len();
        if count > 0 {
            by_uf.insert(uf, count);
        }
    }

    let mut ufs: Vec<_> = by_uf.iter().collect();
    ufs.sort_by_key(|(uf, _)| *uf);

    for (uf, count) in ufs {
        println!("  {}: {:>8} CEPs", uf, count);
    }

    println!();
}

fn lookup_cep(data_dir: &str, cep: &str) {
    println!("Loading data and building index...");
    println!();

    let lookup = match build_cep_lookup(data_dir) {
        Ok(lookup) => lookup,
        Err(e) => {
            eprintln!("Error building index: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Searching for CEP: {}", cep);
    println!("═══════════════════════════════════════════════════════");
    println!();

    match lookup.lookup(cep) {
        Some(info) => {
            print_cep_info(info);
        }
        None => {
            println!("CEP not found: {}", cep);
            println!();
            println!(
                "The CEP may not exist or the data files may be incomplete."
            );
        }
    }

    println!();
}

fn print_cep_info(info: &CepInfo) {
    println!("CEP:        {}", info.cep);
    println!("UF:         {} ({})", info.uf, info.uf.full_name());
    println!("Locality:   {}", info.locality);

    if let Some(neighborhood) = &info.neighborhood {
        println!("Neighborhood: {}", neighborhood);
    }

    if !info.address.is_empty() {
        println!("Address:    {}", info.address);
    }

    if let Some(complement) = &info.complement {
        println!("Complement: {}", complement);
    }

    let type_str = match info.type_ {
        CepType::UncodedLocality => "Uncoded Locality (General CEP)",
        CepType::Street => "Street/Address",
        CepType::BigUser => "Big User",
        CepType::OperationalUnit => "Operational Unit",
        CepType::Cpc => "Community Postal Box (CPC)",
    };
    println!("Type:       {}", type_str);
}

fn build_cep_lookup(
    data_dir: &str,
) -> Result<cep_lookup::CepLookup, Box<dyn std::error::Error>> {
    let mut builder = CepLookupBuilder::new();

    println!("Loading eDNE data...");

    // Load LOG_LOCALIDADE
    let loc_path = format!("{}/log/LOG_LOCALIDADE.TXT", data_dir);
    if Path::new(&loc_path).exists() {
        let bytes = fs::read(&loc_path)?;
        let localities = Localities::from_iso8859_1(&bytes)?;
        println!("✓ {} localities", localities.len());
        builder.add_localities(localities);
    }

    // Load LOG_BAIRRO
    let neighborhood_path = format!("{}/log/LOG_BAIRRO.TXT", data_dir);
    if Path::new(&neighborhood_path).exists() {
        let bytes = fs::read(&neighborhood_path)?;
        let neighborhoods = Neighborhoods::from_iso8859_1(&bytes)?;
        println!("✓ {} neighborhoods", neighborhoods.len());
        builder.add_neighborhoods(neighborhoods);
    }

    // Load all LOG_LOGRADOURO_XX
    for uf in edne::models::Uf::iter() {
        let log_path = format!("{}/log/LOG_LOGRADOURO_{}.TXT", data_dir, uf);
        if Path::new(&log_path).exists() {
            let bytes = fs::read(&log_path)?;
            let addresses = Addresses::from_iso8859_1(&bytes)?;
            println!("✓ {} addresses ({})", addresses.len(), uf);
            builder.add_addresses(addresses);
        }
    }

    // Load LOG_GRANDE_USUARIO
    let gu_path = format!("{}/log/LOG_GRANDE_USUARIO.TXT", data_dir);
    if Path::new(&gu_path).exists() {
        let bytes = fs::read(&gu_path)?;
        let big_users = BigUsers::from_iso8859_1(&bytes)?;
        println!("✓ {} big users", big_users.len());
        builder.add_big_users(big_users);
    }

    // Load LOG_UNID_OPER
    let uo_path = format!("{}/log/LOG_UNID_OPER.TXT", data_dir);
    if Path::new(&uo_path).exists() {
        let bytes = fs::read(&uo_path)?;
        let units = OperationalUnits::from_iso8859_1(&bytes)?;
        println!("✓ {} operational units", units.len());
        builder.add_operational_units(units);
    }

    // Load LOG_CPC
    let cpc_path = format!("{}/log/LOG_CPC.TXT", data_dir);
    if Path::new(&cpc_path).exists() {
        let bytes = fs::read(&cpc_path)?;
        let cpcs = Cpcs::from_iso8859_1(&bytes)?;
        println!("✓ {} CPCs", cpcs.len());
        builder.add_cpcs(cpcs);
    }

    println!();
    println!("Building CEP index...");
    Ok(builder.build())
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

fn parse_cpcs(bytes: &[u8]) {
    println!("Parsing CPCs (Community Postal Boxes)...");

    let cpcs = match Cpcs::from_iso8859_1(bytes) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Successfully parsed {} CPCs", cpcs.len());
    println!("═══════════════════════════════════════════════════════");
    println!();

    // Print CPCs grouped by UF
    println!("CPCs by State:");
    println!("───────────────────────────────────────────────────────");

    let mut by_uf: std::collections::HashMap<_, Vec<_>> =
        std::collections::HashMap::new();
    for (id, cpc) in cpcs.iter() {
        by_uf.entry(cpc.uf).or_default().push((id, cpc));
    }

    let mut ufs: Vec<_> = by_uf.keys().collect();
    ufs.sort();

    for uf in ufs {
        let cpc_list = &by_uf[uf];
        println!();
        println!("{} ({} CPCs)", uf, cpc_list.len());
        println!("───────────────────────────────────────────────────────");

        let mut sorted_cpcs = cpc_list.clone();
        sorted_cpcs.sort_by_key(|(id, _)| *id);

        for (id, cpc) in sorted_cpcs.iter().take(10) {
            println!("  [{}] {}", id, cpc.name);
            println!("      Address: {}", cpc.address);
            println!("      CEP: {} (Locality: {})", cpc.cep, cpc.locality_id);
        }

        if cpc_list.len() > 10 {
            println!("  ... and {} more", cpc_list.len() - 10);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Summary");
    println!("═══════════════════════════════════════════════════════");

    // Count by locality
    let mut by_locality: std::collections::HashMap<_, usize> =
        std::collections::HashMap::new();
    for (_, cpc) in cpcs.iter() {
        *by_locality.entry(cpc.locality_id).or_default() += 1;
    }

    println!();
    println!("Statistics:");
    println!("  Total CPCs:              {}", cpcs.len());
    println!("  Localities with CPCs:    {}", by_locality.len());

    let avg_per_locality = cpcs.len() as f64 / by_locality.len() as f64;
    println!("  Average CPCs/locality:   {:.2}", avg_per_locality);

    // Top localities by CPC count
    println!();
    println!("Top 10 localities by CPC count:");
    let mut locality_counts: Vec<_> = by_locality.iter().collect();
    locality_counts.sort_by(|a, b| b.1.cmp(a.1));

    for (locality_id, count) in locality_counts.iter().take(10) {
        println!("  Locality {}: {} CPCs", locality_id, count);
    }

    println!();
}

fn parse_big_users(bytes: &[u8]) {
    println!("Parsing big users...");

    let big_users = match BigUsers::from_iso8859_1(bytes) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Successfully parsed {} big users", big_users.len());
    println!("═══════════════════════════════════════════════════════");
    println!();

    println!("Big Users by State:");
    println!("───────────────────────────────────────────────────────");

    let mut by_uf: std::collections::HashMap<_, Vec<_>> =
        std::collections::HashMap::new();
    for (id, user) in big_users.iter() {
        by_uf.entry(user.uf).or_default().push((id, user));
    }

    let mut ufs: Vec<_> = by_uf.keys().collect();
    ufs.sort();

    for uf in ufs {
        let users = &by_uf[uf];
        println!();
        println!("{} ({} big users)", uf, users.len());
        println!("───────────────────────────────────────────────────────");

        let mut sorted_users = users.clone();
        sorted_users.sort_by_key(|(id, _)| *id);

        for (id, user) in sorted_users.iter().take(10) {
            println!("  [{}] {}", id, user.name);
            println!("      Address: {}", user.address);
            print!(
                "      CEP: {} (Locality: {}, Neighborhood: {}",
                user.cep, user.locality_id, user.neighborhood_id
            );
            if let Some(street_id) = user.street_id {
                print!(", Street: {}", street_id);
            }
            println!(")");
            if let Some(abbrev) = &user.abbreviated_name {
                println!("      Abbreviated: {}", abbrev);
            }
        }

        if users.len() > 10 {
            println!("  ... and {} more", users.len() - 10);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Summary");
    println!("═══════════════════════════════════════════════════════");

    let with_street =
        big_users.iter().filter(|(_, u)| u.street_id.is_some()).count();
    let without_street = big_users.len() - with_street;

    println!();
    println!("Statistics:");
    println!("  Total big users:         {}", big_users.len());
    println!(
        "  With street ID:          {} ({:.1}%)",
        with_street,
        (with_street as f64 / big_users.len() as f64) * 100.0
    );
    println!(
        "  Without street ID:       {} ({:.1}%)",
        without_street,
        (without_street as f64 / big_users.len() as f64) * 100.0
    );

    let mut by_locality: std::collections::HashMap<_, usize> =
        std::collections::HashMap::new();
    for (_, user) in big_users.iter() {
        *by_locality.entry(user.locality_id).or_default() += 1;
    }

    println!("  Localities with users:   {}", by_locality.len());

    println!();
    println!("Top 10 localities by big user count:");
    let mut locality_counts: Vec<_> = by_locality.iter().collect();
    locality_counts.sort_by(|a, b| b.1.cmp(a.1));

    for (locality_id, count) in locality_counts.iter().take(10) {
        println!("  Locality {}: {} big users", locality_id, count);
    }

    println!();
}

fn parse_operational_units(bytes: &[u8]) {
    println!("Parsing operational units...");

    let units = match OperationalUnits::from_iso8859_1(bytes) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Successfully parsed {} operational units", units.len());
    println!("═══════════════════════════════════════════════════════");
    println!();

    println!("Operational Units by State:");
    println!("───────────────────────────────────────────────────────");

    let mut by_uf: std::collections::HashMap<_, Vec<_>> =
        std::collections::HashMap::new();
    for (id, unit) in units.iter() {
        by_uf.entry(unit.uf).or_default().push((id, unit));
    }

    let mut ufs: Vec<_> = by_uf.keys().collect();
    ufs.sort();

    for uf in ufs {
        let unit_list = &by_uf[uf];
        println!();
        println!("{} ({} units)", uf, unit_list.len());
        println!("───────────────────────────────────────────────────────");

        let mut sorted_units = unit_list.clone();
        sorted_units.sort_by_key(|(id, _)| *id);

        for (id, unit) in sorted_units.iter().take(10) {
            println!("  [{}] {}", id, unit.name);
            println!("      Address: {}", unit.address);
            print!(
                "      CEP: {}, Post Box: {:?}",
                unit.cep, unit.post_box_indicator
            );
            if let Some(street_id) = unit.street_id {
                print!(", Street: {}", street_id);
            }
            println!();
        }

        if unit_list.len() > 10 {
            println!("  ... and {} more", unit_list.len() - 10);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Summary");
    println!("═══════════════════════════════════════════════════════");

    let with_street =
        units.iter().filter(|(_, u)| u.street_id.is_some()).count();
    let without_street = units.len() - with_street;
    let with_post_box = units
        .iter()
        .filter(|(_, u)| {
            matches!(u.post_box_indicator, edne::PostBoxIndicator::Yes)
        })
        .count();
    let without_post_box = units.len() - with_post_box;

    println!();
    println!("Statistics:");
    println!("  Total operational units: {}", units.len());
    println!(
        "  With street ID:          {} ({:.1}%)",
        with_street,
        (with_street as f64 / units.len() as f64) * 100.0
    );
    println!(
        "  Without street ID:       {} ({:.1}%)",
        without_street,
        (without_street as f64 / units.len() as f64) * 100.0
    );
    println!(
        "  With post box:           {} ({:.1}%)",
        with_post_box,
        (with_post_box as f64 / units.len() as f64) * 100.0
    );
    println!(
        "  Without post box:        {} ({:.1}%)",
        without_post_box,
        (without_post_box as f64 / units.len() as f64) * 100.0
    );

    println!();
}

fn parse_addresses(bytes: &[u8]) {
    println!("Parsing addresses (streets)...");

    let addresses = match Addresses::from_iso8859_1(bytes) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    };

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Successfully parsed {} addresses", addresses.len());
    println!("═══════════════════════════════════════════════════════");
    println!();

    println!("Addresses by State:");
    println!("───────────────────────────────────────────────────────");

    let mut by_uf: std::collections::HashMap<_, Vec<_>> =
        std::collections::HashMap::new();
    for (id, addr) in addresses.iter() {
        by_uf.entry(addr.uf).or_default().push((id, addr));
    }

    let mut ufs: Vec<_> = by_uf.keys().collect();
    ufs.sort();

    for uf in ufs {
        let addr_list = &by_uf[uf];
        println!();
        println!("{} ({} addresses)", uf, addr_list.len());
        println!("───────────────────────────────────────────────────────");

        let mut sorted_addrs = addr_list.clone();
        sorted_addrs.sort_by_key(|(id, _)| *id);

        for (id, addr) in sorted_addrs.iter().take(10) {
            println!("  [{}] {} {}", id, addr.street_type, addr.name);
            println!(
                "      CEP: {}, Neighborhood: {}",
                addr.cep, addr.neighborhood_id_start
            );
            if let Some(abbrev) = &addr.abbreviated_name {
                println!("      Abbreviated: {}", abbrev);
            }
        }

        if addr_list.len() > 10 {
            println!("  ... and {} more", addr_list.len() - 10);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  Summary");
    println!("═══════════════════════════════════════════════════════");

    let mut by_type: std::collections::HashMap<_, usize> =
        std::collections::HashMap::new();
    for (_, addr) in addresses.iter() {
        *by_type.entry(&addr.street_type).or_default() += 1;
    }

    println!();
    println!("By Street Type:");
    let mut types: Vec<_> = by_type.iter().collect();
    types.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (street_type, count) in types.iter().take(10) {
        println!("  {}: {} addresses", street_type, count);
    }

    let with_complement =
        addresses.iter().filter(|(_, a)| a.complement.is_some()).count();
    let with_abbrev =
        addresses.iter().filter(|(_, a)| a.abbreviated_name.is_some()).count();

    println!();
    println!("Statistics:");
    println!("  Total addresses:         {}", addresses.len());
    println!(
        "  With complement:         {} ({:.1}%)",
        with_complement,
        (with_complement as f64 / addresses.len() as f64) * 100.0
    );
    println!(
        "  With abbreviation:       {} ({:.1}%)",
        with_abbrev,
        (with_abbrev as f64 / addresses.len() as f64) * 100.0
    );

    println!();
}
