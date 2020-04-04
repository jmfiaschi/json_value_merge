extern crate serde_json;

use serde_json::{Map, Value};

/// Trait used to merge Json Values
pub trait Merge {
    /// Method use to merge two Json Values : ValueA <- ValueB.
    ///
    /// # Examples:
    /// ```ignore
    /// use serde_json::Value;
    ///
    /// let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    /// let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// first_json_value.merge(secound_json_value);
    /// assert_eq!(r#"["a","b","c"]"#, first_json_value.to_string());
    ///
    /// OR
    ///
    /// let mut first_json_value: Value =
    /// serde_json::from_str(r#"{"value1":"a","value2":"b"}"#).unwrap();
    /// let secound_json_value: Value =
    ///     serde_json::from_str(r#"{"value1":"a","value2":"c","value3":"d"}"#).unwrap();
    /// first_json_value.merge(secound_json_value);
    /// assert_eq!(
    ///     r#"{"value1":"a","value2":"c","value3":"d"}"#,
    ///     first_json_value.to_string()
    /// );
    /// ```
    fn merge(&mut self, new_json_value: Value);
    /// Merge a new value in specific json pointer
    ///
    /// # Examples:
    /// ```ignore
    /// use serde_json::Value;
    ///
    /// let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let value_b: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// value_a.merge_in("/my_array", value_b.clone());
    /// assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, value_a.to_string());
    ///
    /// OR
    ///
    /// let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
    /// value_a.merge_in("/my_array/0/a", value_b.clone());
    /// assert_eq!(r#"{"my_array":[{"a":{"b":"c"}}]}"#, value_a.to_string());
    /// ```
    fn merge_in(&mut self, json_pointer: &str, new_json_value: Value);
}

impl Merge for serde_json::Value {
    fn merge(&mut self, new_json_value: Value) {
        merge(self, &new_json_value);
    }
    fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) {
        merge_in(self, json_pointer, new_json_value);
    }
}

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (&mut Value::Array(ref mut a), &Value::Array(ref b)) => {
            a.extend(b.clone());
            a.dedup();
        }
        (&mut Value::Array(ref mut a), &Value::Object(ref b)) => {
            a.push(Value::Object(b.clone()));
            a.dedup();
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

fn merge_in(json_value: &mut Value, json_pointer: &str, new_json_value: Value) -> () {
    let mut fields: Vec<&str> = json_pointer.split("/").skip(1).collect();
    let first_field = fields[0].clone();
    fields.remove(0);
    let next_fields = fields;

    if let Value::Null = json_value {
        *json_value = match first_field.parse::<usize>().ok() {
            Some(_) => Value::Array(Vec::default()),
            None => Value::Object(Map::default()),
        }
        .into();
    }
    match json_value.pointer_mut(format!("/{}", first_field).as_str()) {
        Some(json_target) => {
            if 0 < next_fields.len() {
                merge_in(
                    json_target,
                    format!("/{}", next_fields.join("/")).as_ref(),
                    new_json_value,
                );
            } else {
                json_target.merge(new_json_value);
            }
        }
        None => {
            if 0 < next_fields.len() {
                merge_in(
                    &mut Value::default(),
                    format!("/{}", next_fields.join("/")).as_ref(),
                    new_json_value,
                );
            } else {
                json_value.merge(new_json_value);
            }
        }
    };
}

#[cfg(test)]
mod serde_json_value_updater_test {
    use super::*;
    #[test]
    fn it_should_merge_array_string() {
        let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
        let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
        first_json_value.merge(secound_json_value);
        assert_eq!(r#"["a","b","c"]"#, first_json_value.to_string());
    }
    #[test]
    fn it_should_merge_array_object() {
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
    #[test]
    fn it_should_merge_object() {
        let mut first_json_value: Value =
            serde_json::from_str(r#"{"value1":"a","value2":"b"}"#).unwrap();
        let secound_json_value: Value =
            serde_json::from_str(r#"{"value1":"a","value2":"c","value3":"d"}"#).unwrap();
        first_json_value.merge(secound_json_value);
        assert_eq!(
            r#"{"value1":"a","value2":"c","value3":"d"}"#,
            first_json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_string() {
        let mut value_a: Value = Value::String("a".to_string());
        let value_b: Value = Value::String("b".to_string());
        value_a.merge(value_b.clone());
        assert_eq!(value_b.to_string(), value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_in_a_specifique_field_path() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
        value_a.merge_in("/my_array", value_b.clone());
        assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_object_in_a_specifique_field_path() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array", value_b.clone());
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_in_an_object_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/0", value_b.clone());
        assert_eq!(r#"{"my_array":[{"a":"t","b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/1", value_b.clone());
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
}
