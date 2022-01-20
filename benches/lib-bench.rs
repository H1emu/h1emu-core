use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/lib.rs"] // Here
mod lib;
use lib::*;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate_random_guid", |b| b.iter(|| generate_random_guid()));
    let data:[u8;24] = [0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205, 26, 83, 24, 212];
    c.bench_function("append_crc", |b| b.iter(|| append_crc(black_box(&data),black_box(0))));
    drop(data);
    let data:[u8;5] =  [0, 21, 0, 0, 2];
    c.bench_function("crc32", |b| b.iter(|| crc32(black_box(&data), black_box(0))));
    drop(data);
    c.bench_function("joaat", |b| b.iter(|| joaat(black_box("HAX"))));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);