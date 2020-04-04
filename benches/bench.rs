use criterion::{black_box, criterion_group, criterion_main, Criterion};
use json_value_merge::Merge;
use serde_json::*;

fn merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("Merge");
    let mut json_value: Value = Value::String("my_string".to_string());
    group.bench_function("Bench json_value.merge(null).", |b| {
        b.iter(|| black_box(json_value.merge(Value::Null)))
    });
    group.bench_function("Bench json_value.merge(string).", |b| {
        b.iter(|| black_box(json_value.merge(Value::String("my string B".to_string()))))
    });
    let mut json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    group.bench_function("Bench json_value.merge(array).", |b| {
        b.iter(|| {
            black_box({
                let json_value_to_merge: Value = serde_json::from_str(r#"["c","d"]"#).unwrap();
                json_value.merge(json_value_to_merge);
            })
        })
    });
    let mut json_value: Value = serde_json::from_str(r#"{"a":"b"}"#).unwrap();
    group.bench_function("Bench json_value.merge(object).", |b| {
        b.iter(|| {
            black_box({
                let json_value_to_merge: Value = serde_json::from_str(r#"{"c":"d"}"#).unwrap();
                json_value.merge(json_value_to_merge);
            })
        })
    });
    group.finish();
}

fn merge_in(c: &mut Criterion) {
    let mut group = c.benchmark_group("Merge in");

    let mut json_value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
    group.bench_function("Bench json_value.merge_in(pointer, null).", |b| {
        b.iter(|| black_box(json_value.merge_in("/", Value::Null)))
    });

    let mut json_value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
    group.bench_function("Bench json_value.merge_in(pointer, string).", |b| {
        b.iter(|| black_box(json_value.merge_in("/", Value::String("new value".to_string()))))
    });

    let mut json_value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
    group.bench_function("Bench json_value.merge_in(pointer, object).", |b| {
        b.iter(|| {
            black_box({
                let json_value_to_merge: Value =
                    serde_json::from_str(r#"{"new_field":"new_value"}"#).unwrap();
                json_value.merge_in("/", json_value_to_merge);
            })
        })
    });

    let mut json_value: Value = serde_json::from_str(r#"["value"]"#).unwrap();
    group.bench_function("Bench json_value.merge_in(pointer, array).", |b| {
        b.iter(|| {
            black_box({
                let json_value_to_merge: Value = serde_json::from_str(r#"["new_value"]"#).unwrap();
                json_value.merge_in("/", json_value_to_merge);
            })
        })
    });
    group.finish();
}

criterion_group!(benches, merge, merge_in);
criterion_main!(benches);
