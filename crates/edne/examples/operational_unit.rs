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

use edne::models::{OperationalUnitId, PostBoxIndicator};
use edne::parser::operational_units::OperationalUnits;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_data = "\
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

    println!("=== eDNE Operational Units Parser Example ===\n");

    let units = OperationalUnits::from_utf8(sample_data.to_string())?;
    println!("✓ Parsed {} operational units\n", units.len());

    println!("--- Example 1: Get by ID ---");
    let id = OperationalUnitId::new(1);
    if let Some(unit) = units.get(&id) {
        println!("Unit ID: {}", unit.id);
        println!("  Name: {}", unit.name);
        println!("  Address: {}", unit.address);
        println!("  CEP: {}", unit.cep);
        println!("  Post Box: {:?}", unit.post_box_indicator);
        if let Some(street_id) = unit.street_id {
            println!("  Street ID: {}", street_id);
        }
    }
    println!();

    println!("--- Example 2: Units with Post Box ---");
    let with_post_box: Vec<_> = units
        .iter()
        .filter(|(_, u)| u.post_box_indicator == PostBoxIndicator::Yes)
        .collect();
    println!("Found {} units with post box", with_post_box.len());
    for (_, unit) in with_post_box {
        println!("  • {} - {}", unit.name, unit.address);
    }
    println!();

    println!("--- Example 3: Units without Street ID ---");
    let without_street: Vec<_> =
        units.iter().filter(|(_, u)| u.street_id.is_none()).collect();
    println!("Found {} units without street ID", without_street.len());
    println!();

    println!("--- Example 4: Search by Name Pattern ---");
    let cdd_units: Vec<_> =
        units.iter().filter(|(_, u)| u.name.contains("CDD")).collect();
    println!("Found {} CDD units", cdd_units.len());
    for (_, unit) in cdd_units {
        println!("  • {}", unit.name);
    }
    println!();

    println!("--- Example 5: Statistics ---");
    let with_street =
        units.iter().filter(|(_, u)| u.street_id.is_some()).count();
    let without_street_count = units.len() - with_street;
    let post_box_yes = units
        .iter()
        .filter(|(_, u)| u.post_box_indicator == PostBoxIndicator::Yes)
        .count();
    let post_box_no = units.len() - post_box_yes;

    println!("Total units: {}", units.len());
    println!(
        "With street ID: {} ({:.1}%)",
        with_street,
        (with_street as f64 / units.len() as f64) * 100.0
    );
    println!(
        "Without street ID: {} ({:.1}%)",
        without_street_count,
        (without_street_count as f64 / units.len() as f64) * 100.0
    );
    println!(
        "With post box: {} ({:.1}%)",
        post_box_yes,
        (post_box_yes as f64 / units.len() as f64) * 100.0
    );
    println!(
        "Without post box: {} ({:.1}%)",
        post_box_no,
        (post_box_no as f64 / units.len() as f64) * 100.0
    );

    Ok(())
}
