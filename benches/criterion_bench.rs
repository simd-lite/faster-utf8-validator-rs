extern crate core_affinity;
#[macro_use]
extern crate criterion;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use criterion::{BatchSize, Criterion, ParameterizedBenchmark, Throughput};
use std::fs::File;
use std::io::Read;

macro_rules! bench_file {
    ($name:ident) => {
        fn $name(c: &mut Criterion) {
            let core_ids = core_affinity::get_core_ids().unwrap();
            core_affinity::set_for_current(core_ids[0]);

            let mut vec = Vec::new();
            File::open(concat!("data/", stringify!($name), ".data"))
                .unwrap()
                .read_to_end(&mut vec)
                .unwrap();

            let b = ParameterizedBenchmark::new(
                "faster_utf8_validator",
                |b, data| {
                    b.iter_batched(
                        || data,
                        |bytes| {
                            assert!(faster_utf8_validator::validate(&bytes));
                        },
                        BatchSize::SmallInput,
                    )
                },
                vec![vec],
            );
            c.bench(
                stringify!($name),
                b.throughput(|data| Throughput::Bytes(data.len() as u64)),
            );
        }
    };
}

macro_rules! bench_file_bad {
    ($name:ident) => {
        fn $name(c: &mut Criterion) {
            let core_ids = core_affinity::get_core_ids().unwrap();
            core_affinity::set_for_current(core_ids[0]);

            let mut vec = Vec::new();
            File::open(concat!("data/", stringify!($name), ".data"))
                .unwrap()
                .read_to_end(&mut vec)
                .unwrap();

            let b = ParameterizedBenchmark::new(
                "faster_utf8_validator",
                |b, data| {
                    b.iter_batched(
                        || data,
                        |bytes| {
                            assert!(!faster_utf8_validator::validate(&bytes));
                        },
                        BatchSize::SmallInput,
                    )
                },
                vec![vec],
            );
            c.bench(
                stringify!($name),
                b.throughput(|data| Throughput::Bytes(data.len() as u64)),
            );
        }
    };
}

bench_file!(apache_builds);
bench_file!(canada);
bench_file!(citm_catalog);
bench_file!(github_events);
bench_file!(gsoc_2018);
bench_file!(instruments);
bench_file!(log);
bench_file!(marine_ik);
bench_file!(mesh);
bench_file!(numbers);
bench_file!(random);
bench_file!(twitterescaped);
bench_file!(twitter);
bench_file!(update_center);
bench_file!(mostly_ascii_sample_ok);
bench_file_bad!(random_bytes);
bench_file!(utf8_characters_0_0x10ffff);
bench_file_bad!(utf8_characters_0_0x10ffff_with_garbage);
bench_file!(utf8_sample_ok);
bench_file!(ascii_sample_ok);

criterion_group!(
    benches,
    mostly_ascii_sample_ok,
    ascii_sample_ok,
    random_bytes,
    utf8_characters_0_0x10ffff,
    utf8_characters_0_0x10ffff_with_garbage,
    utf8_sample_ok,
    apache_builds,
    canada,
    citm_catalog,
    github_events,
    gsoc_2018,
    instruments,
    log,
    marine_ik,
    mesh,
    numbers,
    random,
    twitterescaped,
    twitter,
    update_center
);

criterion_main!(benches);
