# json_value_merge
[![Actions Status](https://github.com/jmfiaschi/json_value_merge/workflows/CI/badge.svg)](https://github.com/jmfiaschi/json_value_merge/actions)

Give an interface to merge two json_serde::Value together.

## Installation

 ```
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

# Bench

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
