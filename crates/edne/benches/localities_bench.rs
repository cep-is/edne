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

use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};
use std::hint::black_box;

use edne::parser::localities::Localities;

const SAMPLE_DATA: &str = "\
15321@AC@Terra Indgena Mamoadate@69939810@0@P@2@Terra Ind Mamoadate@
13@AC@Plcido de Castro@69928000@0@M@@Plcido Castro@1200385
15323@AC@Terra Indgena Kampa e Isolados do Rio Envira@69969820@0@P@8@Terra Ind K I R Envira@
15324@AC@Terra Indgena Kaxinaw do Rio Humait@69969830@0@P@8@Terra Ind K R Humait@
15325@AC@Terra Indgena Kulina do Rio Envira@69969840@0@P@8@Terra Ind K R Envira@
15326@AC@Terra Indgena Riozinho do Alto Envira@69969850@0@P@8@Terra Ind R At Envira@
15327@AC@Terra Indgena Igarap Taboca do Alto Tarauac@69979810@0@P@9@Terra Ind I T At Tarauac@
21@AC@Tarauac@69970000@0@M@@Tarauac@1200609
19@AC@Sena Madureira@69940000@0@M@@Sena Madureira@1200500
16@AC@Rio Branco@@1@M@@Rio Branco@1200401
12@AC@Marechal Thaumaturgo@69983000@0@M@@Mal Thaumaturgo@1200351
5@AC@Capixaba@69931000@0@M@@Capixaba@1200179
6@AC@Cruzeiro do Sul@69980000@0@M@@Cruzeiro Sul@1200203
7@AC@Epitaciolndia@69934000@0@M@@Epitaciolndia@1200252
8@AC@Feij@69960000@0@M@@Feij@1200302";

fn generate_large_dataset(count: usize) -> String {
    let mut result = String::with_capacity(count * 100);

    for i in 1..=count {
        let line = format!(
            "{}@SP@Localidade Teste {}@01234567@0@M@@Loc Test {}@7654321\n",
            i, i, i
        );
        result.push_str(&line);
    }

    result
}

fn bench_small_dataset(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_dataset");
    group.throughput(Throughput::Elements(15));

    group.bench_function("parse_15_localities", |b| {
        b.iter(|| {
            Localities::from_utf8(black_box(SAMPLE_DATA.to_string())).unwrap()
        });
    });

    group.finish();
}

fn bench_large_datasets(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_datasets");

    for size in [100, 1_000, 10_000].iter() {
        let data = generate_large_dataset(*size);
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &data,
            |b, data| {
                b.iter(|| {
                    Localities::from_utf8(black_box(data.clone())).unwrap()
                });
            },
        );
    }

    group.finish();
}

fn bench_iso8859_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso8859_decoding");

    // Generate sample with ISO-8859-1 encoded data
    let data = SAMPLE_DATA.as_bytes();
    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("decode_and_parse", |b| {
        b.iter(|| Localities::from_iso8859_1(black_box(data)).unwrap());
    });

    group.finish();
}

fn bench_lookup_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup");

    let localities = Localities::from_utf8(SAMPLE_DATA.to_string()).unwrap();
    let id = edne::models::LocalityId::new(13);

    group.bench_function("get_by_id", |b| {
        b.iter(|| localities.get(black_box(&id)));
    });

    group.finish();
}

fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration");

    for size in [100, 1_000, 10_000].iter() {
        let data = generate_large_dataset(*size);
        let localities = Localities::from_utf8(data).unwrap();
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &localities,
            |b, localities| {
                b.iter(|| localities.iter().count());
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_small_dataset,
    bench_large_datasets,
    bench_iso8859_decoding,
    bench_lookup_performance,
    bench_iteration
);

criterion_main!(benches);
