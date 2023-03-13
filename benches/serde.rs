#[path = "./shared/data.rs"]
mod data;
#[path = "./shared/http_log.rs"]
mod http_log;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use data::{JSON_FLOAT_STR, JSON_STR};
use serde_json::Value;

fn serde_log(c: &mut Criterion) {
    let mut group = c.benchmark_group("http_log");

    group.throughput(Throughput::Bytes(JSON_STR.len() as u64));
    group.bench_with_input("parse", JSON_STR, |b, input_str| {
        b.iter(|| {
            serde_json::from_str::<Value>(input_str).unwrap();
        })
    });

    {
        let data = serde_json::from_str::<Value>(JSON_STR).unwrap();
        let stringified_len = serde_json::to_string(&data).unwrap().len() as u64;

        group.throughput(Throughput::Bytes(stringified_len));
        group.bench_with_input("stringify", &data, |b, value| {
            b.iter(|| {
                serde_json::to_string(value).unwrap();
            })
        });
    }

    group.throughput(Throughput::Bytes(JSON_STR.len() as u64));
    group.bench_with_input("parse_struct", JSON_STR, |b, input_str| {
        b.iter(|| {
            serde_json::from_str::<http_log::Log>(input_str).unwrap();
        });
    });

    {
        let log = serde_json::from_str::<http_log::Log>(JSON_STR).unwrap();
        let stringified_len = serde_json::to_string(&log).unwrap().len() as u64;

        group.throughput(Throughput::Bytes(stringified_len));
        group.bench_with_input("stringify_struct", &log, |b, log| {
            b.iter(|| {
                serde_json::to_string(log).unwrap();
            })
        });
    }
}

fn serde_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("floats");

    group.throughput(Throughput::Bytes(JSON_FLOAT_STR.len() as u64));
    group.bench_with_input("parse_floats", JSON_FLOAT_STR, |b, input_str| {
        b.iter(|| {
            serde_json::from_str::<Value>(input_str).unwrap();
        })
    });

    let float_value = serde_json::from_str::<Value>(JSON_FLOAT_STR).unwrap();
    let stringified_len = serde_json::to_string(&float_value).unwrap().len() as u64;

    group.throughput(Throughput::Bytes(stringified_len));
    group.bench_with_input("stringify_floats", &float_value, |b, value| {
        b.iter(|| {
            serde_json::to_string(value).unwrap();
        })
    });
}

criterion_group!(serde_bench, serde_log, serde_float);

criterion_main!(serde_bench);
