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

## Bench

```Bash
Merge/Bench json_value.merge(null).
                        time:   [8.5834 ns 8.9230 ns 9.2797 ns]
                        change: [-3.5199% -1.0551% +1.6214%] (p = 0.44 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

Merge/Bench json_value.merge(string).
                        time:   [50.644 ns 51.480 ns 52.560 ns]
                        change: [-0.0573% +2.5336% +4.9260%] (p = 0.05 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

Merge/Bench json_value.merge(array).
                        time:   [535.83 us 558.26 us 577.65 us]
                        change: [-12.646% -5.3659% +1.7625%] (p = 0.16 > 0.05)
                        No change in performance detected.

Merge/Bench json_value.merge(object).
                        time:   [341.14 ns 352.63 ns 364.50 ns]
                        change: [-7.2561% -2.8396% +1.7016%] (p = 0.23 > 0.05)
                        No change in performance detected.
Found 21 outliers among 100 measurements (21.00%)
  9 (9.00%) high mild
  12 (12.00%) high severe

Merge in/Bench json_value.merge_in(pointer, null).
                        time:   [261.30 ns 265.50 ns 270.18 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

Merge in/Bench json_value.merge_in(pointer, string).
                        time:   [296.51 ns 301.07 ns 305.53 ns]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe

Merge in/Bench json_value.merge_in(pointer, object).
                        time:   [618.46 ns 634.22 ns 656.36 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

Merge in/Bench json_value.merge_in(pointer, array).
                        time:   [409.86 ns 410.32 ns 410.84 ns]
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
