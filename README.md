# json_value_merge

[![Linter](https://github.com/jmfiaschi/json_value_merge/workflows/Lint/badge.svg)](https://github.com/jmfiaschi/json_value_merge/actions)
[![Actions Status](https://github.com/jmfiaschi/json_value_merge/workflows/CI/badge.svg)](https://github.com/jmfiaschi/json_value_merge/actions)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg)](https://github.com/semantic-release/semantic-release)

Give an interface to merge two json_serde::Value together.

## Installation

 ```Toml
[dependencies]
json_value_merge = "0.2"
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

## Useful link

* [Benchmark report](https://jmfiaschi.github.io/json_value_merge/bench/master/)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
