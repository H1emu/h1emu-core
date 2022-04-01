use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/rc4.rs"]
mod rc4;
use rc4::RC4;

#[path = "../src/crc.rs"]
mod crc;
use crc::*;

#[path = "../src/jenkins.rs"]
mod jenkins;
use jenkins::*;

#[path = "../src/utils.rs"]
mod utils;
use utils::*;

fn crc_legacy_benchmark(c: &mut Criterion) {
    let data: [u8; 24] = [
        0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205, 26, 83,
        24, 212,
    ];
    c.bench_function("append_crc_legacy", |b| {
        b.iter(|| append_crc_legacy(black_box(&data), black_box(0)))
    });
    drop(data);
    let data: [u8; 5] = [0, 21, 0, 0, 2];
    c.bench_function("crc32_legacy", |b| {
        b.iter(|| crc32_legacy(black_box(&data), black_box(0)))
    });
    drop(data);
}

fn crc_benchmark(c: &mut Criterion) {
    let mut data: Vec<u8> = [
        0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205, 26, 83,
        24, 212,
    ]
    .to_vec();
    c.bench_function("append_crc", |b| {
        b.iter(|| append_crc(black_box(&mut data), black_box(0)))
    });
    drop(data);
    let mut data: Vec<u8> = [0, 21, 0, 0, 2].to_vec();
    c.bench_function("crc32", |b| {
        b.iter(|| crc32(black_box(&&mut data), black_box(0)))
    });
    drop(data);
}

fn utils_benchmark(c: &mut Criterion) {
    c.bench_function("generate_random_guid", |b| {
        b.iter(|| generate_random_guid())
    });
}

fn jooat_benchmark(c: &mut Criterion) {
    c.bench_function("joaat", |b| b.iter(|| joaat(black_box("HAX"))));
}

fn rc4_benchmark(c: &mut Criterion) {
    let key: [u8; 16] = [
        23, 189, 8, 107, 27, 148, 240, 47, 240, 236, 83, 215, 99, 88, 155, 95,
    ];
    c.bench_function("RC4::initialize", |b| {
        b.iter(|| RC4::initialize(black_box(key.to_vec())))
    });
    let key: [u8; 16] = [
        23, 189, 8, 107, 27, 148, 240, 47, 240, 236, 83, 215, 99, 88, 155, 95,
    ];
    let data: [u8; 34] = [
        5, 1, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 2, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0,
        116, 101, 115, 116,
    ];
    let mut rc4_obj = RC4::initialize(key.to_vec());
    c.bench_function("RC4::encrypt", |b| {
        b.iter(|| rc4_obj.encrypt(black_box(data.to_vec())))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    crc_legacy_benchmark(c);
    crc_benchmark(c);
    utils_benchmark(c);
    jooat_benchmark(c);
    rc4_benchmark(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
