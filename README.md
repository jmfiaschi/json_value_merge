# json_value_merge

[![Actions Status](https://github.com/jmfiaschi/json_value_merge/workflows/ci/badge.svg)](https://github.com/jmfiaschi/json_value_merge/actions/workflows/ci.yml)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg)](https://github.com/semantic-release/semantic-release)
![crates.io](https://img.shields.io/crates/v/json_value_merge.svg)

Give an interface to merge two json_serde::Value together.

# Getting Started

## Installation

Add the following line to your Cargo.toml:
```toml
[dependencies]
json_value_merge = "x.y.z" # Replace with the latest version
```

## Quick Start

### Merge two arrays

```rust
extern crate json_value_merge;

use json_value_merge::Merge;
use serde_json::Value;

{
    let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    first_json_value.merge(&secound_json_value);
    assert_eq!(r#"["a","b","c"]"#, first_json_value.to_string());
}
```

---

### Merge two objects

```rust
extern crate json_value_merge;

use json_value_merge::Merge;
use serde_json::Value;

{
    let mut first_json_value: Value =
        serde_json::from_str(r#"[{"value":"a"},{"value":"b"}]"#).unwrap();
    let secound_json_value: Value =
        serde_json::from_str(r#"[{"value":"b"},{"value":"c"}]"#).unwrap();
    first_json_value.merge(&secound_json_value);
    assert_eq!(
        r#"[{"value":"a"},{"value":"b"},{"value":"b"},{"value":"c"}]"#,
        first_json_value.to_string()
    );
}
```

---

### Merge two arrays in a specifique position:

```rust
extern crate json_value_merge;

use json_value_merge::Merge;
use serde_json::Value;

{
    let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    let value_b: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    value_a.merge_in("/my_array", &value_b);
    assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, value_a.to_string());
}
```

---

### Merge two objects in a specifique position

```rust
extern crate json_value_merge;

use json_value_merge::Merge;
use serde_json::Value;

{
    let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
    value_a.merge_in("/my_array/0", &value_b);
    assert_eq!(r#"{"my_array":[{"a":"t","b":"c"}]}"#, value_a.to_string());
}
```

---

### Build new object

```rust
extern crate json_value_merge;

use json_value_merge::Merge;
use serde_json::Value;

{
    let mut object: Value = Value::default();
    object.merge_in("/field", &Value::String("value".to_string()));
    object.merge_in("/object", &Value::Object(Map::default()));
    object.merge_in("/array/1", &Value::Object(Map::default()));
    object.merge_in("/array/2", &Value::Array(Vec::default()));
    object.merge_in("/array/*", &Value::String("wildcard".to_string()));
    object.merge_in("/root/*/item", &Value::String("my_item".to_string()));
    object.merge_in("///empty", &Value::Null);
    assert_eq!(r#"{"":{"":{"empty":null}},"array":[{},[],"wildcard"],"field":"value","object":{},"root":[{"item":"my_item"}]}"#, object.to_string());
}
```

---

### Replace an array by an object

```rust
extern crate json_value_merge;

use json_value_merge::Merge;
use serde_json::Value;

{
    let mut json_value: Value = serde_json::from_str(r#"[{"array1":[{"field":"value1"}]}]"#).unwrap();
    let result = json_value.merge_in("/other_field", &Value::String("value".to_string()));
    assert_eq!(r#"{"other_field":"value"}"#,json_value.to_string());
}
```

---

## Useful link

* [Benchmark report](https://jmfiaschi.github.io/json_value_merge/bench/main/)
* [Package](https://crates.io/crates/json_value_merge)

---

## Contributing

Contributions are welcome!

To contribute:

1. Fork the repository and create your branch (git checkout -b feature/my-feature).
2. Commit your changes (git commit -m 'Add some feature').
3. Push to the branch (git push origin feature/my-feature).
4. Open a pull request.

For major changes, please open an issue first to discuss your proposal.

Please ensure that tests are added or updated as appropriate.

---

## License

Licensed under either of the following, at your option:

* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT License](https://choosealicense.com/licenses/mit/)
