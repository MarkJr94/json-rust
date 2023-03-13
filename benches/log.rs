extern crate json;

#[path = "./shared/data.rs"]
mod data;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use data::{JSON_FLOAT_STR, JSON_STR};

fn json_log(c: &mut Criterion) {
    let mut group = c.benchmark_group("http_log");

    group.throughput(Throughput::Bytes(JSON_STR.len() as u64));
    group.bench_with_input("parse", JSON_STR, |b, input_str| {
        b.iter(|| {
            json::parse(input_str).unwrap();
        })
    });

    let data = json::parse(JSON_STR).unwrap();
    let stringified_len = data.dump().len() as u64;

    group.throughput(Throughput::Bytes(stringified_len));
    group.bench_with_input("stringify", &data, |b, value| {
        b.iter(|| {
            value.dump();
        })
    });

    group.throughput(Throughput::Bytes(stringified_len));
    group.bench_with_input("stringify_io_write", &data, |b, value| {
        b.iter(|| {
            let mut target = Vec::new();
            value.write(&mut target).unwrap();
        })
    });
}

fn json_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("floats");

    group.throughput(Throughput::Bytes(JSON_FLOAT_STR.len() as u64));
    group.bench_with_input("parse_floats", JSON_FLOAT_STR, |b, input_str| {
        b.iter(|| {
            json::parse(input_str).unwrap();
        })
    });

    let float_value = json::parse(JSON_FLOAT_STR).unwrap();
    let stringified_len = float_value.dump().len() as u64;

    group.throughput(Throughput::Bytes(stringified_len));
    group.bench_with_input("stringify_floats", &float_value, |b, value| {
        b.iter(|| {
            value.dump();
        })
    });

    group.throughput(Throughput::Bytes(stringified_len));
    group.bench_with_input("stringify_floats_io_write", &float_value, |b, value| {
        b.iter(|| {
            let mut target = Vec::new();
            value.write(&mut target).unwrap();
        })
    });
}

criterion_group!(json_rust, json_log, json_float);

criterion_main!(json_rust);
