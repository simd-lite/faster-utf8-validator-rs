use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use criterion::{criterion_group, criterion_main, measurement::Measurement, Criterion, Throughput};

#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "cpb"))]
use criterion_cycles_per_byte::CyclesPerByte;

use std::{fs, str};

fn bench_file<T: Measurement>(c: &mut Criterion<T>, name: &str, is_valid: bool) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(core_ids[0]);

    let buf = fs::read(format!("data/{}.data", name)).unwrap();

    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Bytes(buf.len() as u64));
    group.bench_function("std_utf8_validator", |b| {
        b.iter(|| assert!(str::from_utf8(&buf).is_ok() == is_valid))
    });
    group.bench_function("faster_utf8_validator", |b| {
        b.iter(|| assert!(faster_utf8_validator::validate(&buf) == is_valid))
    });

    group.finish();
}

fn bench_all<T: Measurement>(c: &mut Criterion<T>) {
    bench_file(c, "apache_builds", true);
    bench_file(c, "canada", true);
    bench_file(c, "citm_catalog", true);
    bench_file(c, "github_events", true);
    bench_file(c, "gsoc_2018", true);
    bench_file(c, "instruments", true);
    bench_file(c, "log", true);
    bench_file(c, "marine_ik", true);
    bench_file(c, "mesh", true);
    bench_file(c, "numbers", true);
    bench_file(c, "random", true);
    bench_file(c, "twitterescaped", true);
    bench_file(c, "twitter", true);
    bench_file(c, "update_center", true);
    bench_file(c, "mostly_ascii_sample_ok", true);
    bench_file(c, "random_bytes", false);
    bench_file(c, "utf8_characters_0_0x10ffff", true);
    bench_file(c, "utf8_characters_0_0x10ffff_with_garbage", false);
    bench_file(c, "utf8_sample_ok", true);
    bench_file(c, "ascii_sample_ok", true);
}

#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "cpb"))]
criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench_all
}

#[cfg(not(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "cpb")))]
criterion_group!(benches, bench_all);

criterion_main!(benches);
