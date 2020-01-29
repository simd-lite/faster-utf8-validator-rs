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
            File::open(concat!("data/", stringify!($name), ".json"))
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

criterion_group!(
    benches,
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
