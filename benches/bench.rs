use criterion::{black_box, criterion_group, criterion_main, Criterion};
use json_value_merge::Merge;
use serde_json::*;

fn merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge");

    let mut value: Value = Value::Null;
    let value_to_merge = value.clone();
    group.bench_function("null", |b| {
        b.iter(|| black_box(value.merge(&value_to_merge)))
    });

    let mut value: Value = Value::String("My String".to_string());
    let value_to_merge = value.clone();
    group.bench_function("string", |b| {
        b.iter(|| black_box(value.merge(&value_to_merge)))
    });

    let mut value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    let value_to_merge: Value = value.clone();
    group.bench_function("array", |b| {
        b.iter(|| black_box(value.merge(&value_to_merge)))
    });

    let mut value: Value = serde_json::from_str(r#"{"a":"b"}"#).unwrap();
    let value_to_merge: Value = value.clone();
    group.bench_function("object", |b| {
        b.iter(|| black_box(value.merge(&value_to_merge)))
    });
    group.finish();
}

fn merge_in(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_in");

    let mut value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
    let value_to_merge: Value = Value::Null;
    group.bench_function("null", |b| {
        b.iter(|| black_box(value.merge_in("/field", &value_to_merge).unwrap()))
    });

    let mut value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
    let value_to_merge: Value = Value::String("new value".to_string());
    group.bench_function("string.", |b| {
        b.iter(|| black_box(value.merge_in("/field", &value_to_merge).unwrap()))
    });

    let mut value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
    let value_to_merge: Value = serde_json::from_str(r#"{"new_field":"new_value"}"#).unwrap();
    group.bench_function("object", |b| {
        b.iter(|| black_box(value.merge_in("/field", &value_to_merge).unwrap()))
    });

    let mut value: Value = serde_json::from_str(r#"["value"]"#).unwrap();
    let value_to_merge: Value = serde_json::from_str(r#"["new_value"]"#).unwrap();
    group.bench_function("array", |b| {
        b.iter(|| black_box(value.merge_in("/0", &value_to_merge).unwrap()))
    });

    let mut value: Value = serde_json::from_str(r#"{"batters":{"batter":[{"id":"1001","type":"Regular"},{"id":"1002","type":"Chocolate"},{"id":"1003","type":"Blueberry"},{"id":"1004","type":"Devil's Food"}]},"id":"0001","name":"Cake","ppu":0.55,"topping":[{"id":"5001","type":"None"},{"id":"5002","type":"Glazed"},{"id":"5005","type":"Sugar"},{"id":"5007","type":"Powdered Sugar"},{"id":"5006","type":"Chocolate with Sprinkles"},{"id":"5003","type":"Chocolate"},{"id":"5004","type":"Maple"}],"type":"donut","child":{"batters":{"batter":[{"id":"1001","type":"Regular"},{"id":"1002","type":"Chocolate"},{"id":"1003","type":"Blueberry"},{"id":"1004","type":"Devil's Food"}]},"id":"0001","name":"Cake","ppu":0.55,"topping":[{"id":"5001","type":"None"},{"id":"5002","type":"Glazed"},{"id":"5005","type":"Sugar"},{"id":"5007","type":"Powdered Sugar"},{"id":"5006","type":"Chocolate with Sprinkles"},{"id":"5003","type":"Chocolate"},{"id":"5004","type":"Maple"}],"type":"donut","child":{"batters":{"batter":[{"id":"1001","type":"Regular"},{"id":"1002","type":"Chocolate"},{"id":"1003","type":"Blueberry"},{"id":"1004","type":"Devil's Food"}]},"id":"0001","name":"Cake","ppu":0.55,"topping":[{"id":"5001","type":"None"},{"id":"5002","type":"Glazed"},{"id":"5005","type":"Sugar"},{"id":"5007","type":"Powdered Sugar"},{"id":"5006","type":"Chocolate with Sprinkles"},{"id":"5003","type":"Chocolate"},{"id":"5004","type":"Maple"}],"type":"donut","child":{"batters":{"batter":[{"id":"1001","type":"Regular"},{"id":"1002","type":"Chocolate"},{"id":"1003","type":"Blueberry"},{"id":"1004","type":"Devil's Food"}]},"id":"0001","name":"Cake","ppu":0.55,"topping":[{"id":"5001","type":"None"},{"id":"5002","type":"Glazed"},{"id":"5005","type":"Sugar"},{"id":"5007","type":"Powdered Sugar"},{"id":"5006","type":"Chocolate with Sprinkles"},{"id":"5003","type":"Chocolate"},{"id":"5004","type":"Maple"}],"type":"donut","child":{"batters":{"batter":[{"id":"1001","type":"Regular"},{"id":"1002","type":"Chocolate"},{"id":"1003","type":"Blueberry"},{"id":"1004","type":"Devil's Food"}]},"id":"0001","name":"Cake","ppu":0.55,"topping":[{"id":"5001","type":"None"},{"id":"5002","type":"Glazed"},{"id":"5005","type":"Sugar"},{"id":"5007","type":"Powdered Sugar"},{"id":"5006","type":"Chocolate with Sprinkles"},{"id":"5003","type":"Chocolate"},{"id":"5004","type":"Maple"}],"type":"donut"}}}}}"#).unwrap();
    let value_to_merge: Value = serde_json::from_str(r#"{"new_field":"new_value"}"#).unwrap();
    group.bench_function("deeper_object", |b| {
        b.iter(|| black_box(value.merge_in("/child/child/child/batters/batter",&value_to_merge).unwrap()))
    });

    group.finish();
}

criterion_group!(benches, merge, merge_in);
criterion_main!(benches);
