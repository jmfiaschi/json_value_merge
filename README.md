# json_value_merge

[![Actions Status](https://github.com/jmfiaschi/json_value_merge/workflows/CI/badge.svg)](https://github.com/jmfiaschi/json_value_merge/actions)

Give an interface to merge two json_serde::Value together.

## Installation

 ```Toml
[dependencies]
json_value_merge = "0.1"
```

## Usage

Merge two arrays:

```rust
extern crate json_value_merge;

use json_value_merge::Merge;

{
    let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    first_json_value.merge(secound_json_value);
    assert_eq!(r#"["a","b","c"]"#, first_json_value.to_string());
}
```

Merge two objects:

```rust
extern crate json_value_merge;

use json_value_merge::Merge;

{
    let mut first_json_value: Value =
        serde_json::from_str(r#"[{"value":"a"},{"value":"b"}]"#).unwrap();
    let secound_json_value: Value =
        serde_json::from_str(r#"[{"value":"b"},{"value":"c"}]"#).unwrap();
    first_json_value.merge(secound_json_value);
    assert_eq!(
        r#"[{"value":"a"},{"value":"b"},{"value":"c"}]"#,
        first_json_value.to_string()
    );
}
```

Merge two arrays in a specifique position:

```rust
extern crate json_value_merge;

use json_value_merge::Merge;

{
    let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    let value_b: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    value_a.merge_in("/my_array", value_b.clone());
    assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, value_a.to_string());
}
```

Merge two objects in a specifique position:

```rust
extern crate json_value_merge;

use json_value_merge::Merge;

{
    let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
    value_a.merge_in("/my_array/0", value_b.clone());
    assert_eq!(r#"{"my_array":[{"a":"t","b":"c"}]}"#, value_a.to_string());
}
```

Build new object:

```rust
extern crate json_value_merge;

use json_value_merge::Merge;

{
    let mut object: Value = Value::default();
    object.merge_in("/field", Value::String("value".to_string()));
    object.merge_in("/object", Value::Object(Map::default()));
    object.merge_in("/array", Value::Array(Vec::default()));
    assert_eq!(
        r#"{"array":[],"field":"value","object":{}}"#,
        object.to_string()
    );
}
```

## Bench

```Bash
Merge/Bench json_value.merge(null).
                        time:   [7.7932 ns 7.8098 ns 7.8353 ns]
                        change: [-0.2730% +0.4027% +1.0706%] (p = 0.25 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

Merge/Bench json_value.merge(string).
                        time:   [45.939 ns 45.988 ns 46.049 ns]
                        change: [-0.6558% +0.1228% +0.8434%] (p = 0.76 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

Merge/Bench json_value.merge(array).
                        time:   [500.95 us 535.75 us 567.63 us]
                        change: [-0.9322% +6.6158% +14.919%] (p = 0.10 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

Merge/Bench json_value.merge(object).
                        time:   [314.60 ns 316.69 ns 318.88 ns]
                        change: [+1.4743% +2.7929% +3.8830%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

Merge in/Bench json_value.merge_in(pointer, null).
                        time:   [295.13 ns 297.23 ns 299.74 ns]
                        change: [-8.4767% -7.5709% -6.5915%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

Merge in/Bench json_value.merge_in(pointer, string).
                        time:   [338.77 ns 342.75 ns 347.36 ns]
                        change: [-8.2556% -6.5970% -5.2257%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

Merge in/Bench json_value.merge_in(pointer, object).
                        time:   [651.13 ns 652.80 ns 654.73 ns]
                        change: [-0.5400% +1.0663% +3.3065%] (p = 0.32 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

Merge in/Bench json_value.merge_in(pointer, array).
                        time:   [435.32 ns 437.06 ns 438.97 ns]
                        change: [-2.9152% -2.3497% -1.7253%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
