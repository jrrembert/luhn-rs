use criterion::{black_box, criterion_group, criterion_main, Criterion};
use luhn_rs::{generate, validate, random, GenerateOptions};

fn benchmark_generate(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");
    
    // Benchmark different input lengths
    group.bench_function("generate_short", |b| {
        b.iter(|| generate(black_box("1234"), None))
    });

    group.bench_function("generate_medium", |b| {
        b.iter(|| generate(black_box("1234567890"), None))
    });

    group.bench_function("generate_long", |b| {
        b.iter(|| generate(black_box("12345678901234567890"), None))
    });

    // Benchmark checksum only vs full number
    group.bench_function("generate_checksum_only", |b| {
        b.iter(|| {
            generate(
                black_box("1234567890"),
                Some(GenerateOptions { checksum_only: true })
            )
        })
    });

    group.finish();
}

fn benchmark_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("validate");
    
    // Benchmark different input lengths
    group.bench_function("validate_short", |b| {
        b.iter(|| validate(black_box("12344")))
    });

    group.bench_function("validate_medium", |b| {
        b.iter(|| validate(black_box("1234567890")))
    });

    group.bench_function("validate_long", |b| {
        b.iter(|| validate(black_box("12345678901234567890")))
    });

    // Benchmark valid vs invalid numbers
    group.bench_function("validate_valid", |b| {
        b.iter(|| validate(black_box("79927398713")))
    });

    group.bench_function("validate_invalid", |b| {
        b.iter(|| validate(black_box("79927398714")))
    });

    group.finish();
}

fn benchmark_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("random");
    
    // Benchmark different lengths
    group.bench_function("random_short", |b| {
        b.iter(|| random(black_box("5")))
    });

    group.bench_function("random_medium", |b| {
        b.iter(|| random(black_box("10")))
    });

    group.bench_function("random_long", |b| {
        b.iter(|| random(black_box("20")))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_generate,
    benchmark_validate,
    benchmark_random
);
criterion_main!(benches);