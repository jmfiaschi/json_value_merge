extern crate serde_json;

use serde_json::{Map, Value};
use std::io;

/// Trait used to merge Json Values
pub trait Merge {
    /// Method use to merge two Json Values : ValueA <- ValueB.
    fn merge(&mut self, new_value: &Value);
    /// Merge a new value in specific json pointer. If the field can't be merge in the specific path, it raise an error.
    fn merge_in(&mut self, json_pointer: &str, new_value: &Value) -> io::Result<()>;
}

impl Merge for serde_json::Value {
    /// # Examples: Merge two array together.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut array1: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    /// let array2: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// array1.merge(&array2);
    /// assert_eq!(r#"["a","b","b","c"]"#, array1.to_string());
    /// ```
    /// # Examples: Merge two objects together.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"value1":"a","value2":"b"}"#).unwrap();
    /// let object2: Value = serde_json::from_str(r#"{"value1":"a","value2":"c","value3":"d"}"#).unwrap();
    /// object1.merge(&object2);
    /// assert_eq!(r#"{"value1":"a","value2":"c","value3":"d"}"#,object1.to_string());
    /// ```
    /// # Examples: Merge an object into an array.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut array: Value = serde_json::from_str(r#"[]"#).unwrap();
    /// let object: Value = serde_json::from_str(r#"{"field1":"value1"}"#).unwrap();
    /// array.merge(&object);
    /// assert_eq!(r#"[{"field1":"value1"}]"#,array.to_string());
    /// ```
    /// # Examples: Merge an array into an object.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object: Value = serde_json::from_str(r#"{"field1":"value1"}"#).unwrap();
    /// let array: Value = serde_json::from_str(r#"["value2","value3"]"#).unwrap();
    /// object.merge(&array);
    /// assert_eq!(r#"["value2","value3"]"#,object.to_string());
    /// ```
    fn merge(&mut self, new_json_value: &Value) {
        merge(self, new_json_value);
    }
    /// # Examples: Merge an array in an object in a specific position.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let array: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// object.merge_in("/my_array", &array);
    /// assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, object.to_string());
    /// ```
    /// # Examples: Merge two objects together in a specific position.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let object2: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
    /// object1.merge_in("/my_array/0/a", &object2);
    /// assert_eq!(r#"{"my_array":[{"a":{"b":"c"}}]}"#, object1.to_string());
    /// ```
    /// # Examples: Merge an object in an array in a specific position. If the position not exist, the object is added in the array.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut json_value: Value = serde_json::from_str(r#"[{"array1":[{"field":"value1"}]}]"#).unwrap();
    /// let result = json_value.merge_in("/1", &Value::String("value".to_string()));
    ///
    /// assert_eq!(r#"[{"array1":[{"field":"value1"}]},"value"]"#,json_value.to_string());
    /// ```
    /// # Examples: Replace an Array by an Object.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut json_value: Value =
    /// serde_json::from_str(r#"{"my_array":[{"field1":"value1"}]}"#).unwrap();
    /// let json_value_to_merge: Value = serde_json::from_str(r#"{"a":"b"}"#).unwrap();
    /// json_value
    ///     .merge_in("/my_array/T/field2", &json_value_to_merge)
    ///     .unwrap();
    /// assert_eq!(
    ///     r#"{"my_array":{"T":{"field2":{"a":"b"}}}}"#,
    ///     json_value.to_string()
    /// );
    /// ```
    /// # Examples: Build a new object.
    /// ```
    /// use serde_json::{Map,Value};
    /// use json_value_merge::Merge;
    ///
    /// let mut object: Value = Value::default();
    /// object.merge_in("/field", &Value::String("value".to_string()));
    /// object.merge_in("/object", &Value::Object(Map::default()));
    /// object.merge_in("/array/1", &Value::Object(Map::default()));
    /// object.merge_in("/array/2", &Value::Array(Vec::default()));
    /// object.merge_in("/array/*", &Value::String("wildcard".to_string()));
    /// object.merge_in("/root/*/item", &Value::String("my_item".to_string()));
    /// object.merge_in("///empty", &Value::Null);
    /// assert_eq!(r#"{"":{"":{"empty":null}},"array":[{},[],"wildcard"],"field":"value","object":{},"root":[{"item":"my_item"}]}"#, object.to_string());
    /// ```
    /// # Examples: Search and replace
    /// ```
    /// use serde_json::{Map,Value};
    /// use json_value_merge::Merge;
    ///
    /// let mut object: Value = serde_json::from_str(r#"{"":{"":{"empty":null}},"array":[{},[],"wildcard"],"field":"value","1":null,"root":[{"item":"my_item"}]}"#).unwrap();
    ///
    /// object.merge_in("/field", &Value::String("my_new_value".to_string()));
    /// object.merge_in("/1", &Value::String("first field".to_string()));
    /// object.merge_in("/array/2", &Value::String("position two".to_string()));
    /// object.merge_in("///empty", &Value::String("not_item".to_string()));
    /// assert_eq!(r#"{"":{"":{"empty":"not_item"}},"1":"first field","array":[{},[],"position two"],"field":"my_new_value","root":[{"item":"my_item"}]}"#, object.to_string());
    /// ```
    fn merge_in(&mut self, json_pointer: &str, new_value: &Value) -> io::Result<()> {
        let fields: Vec<&str> = json_pointer.split('/').skip(1).collect();

        merge_in(self, fields, new_value)
    }
}

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(ref mut a), Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert_with(|| Value::Null), v);
            }
        }
        (Value::Array(ref mut a), Value::Array(ref b)) => {
            a.reserve(b.len());
            a.extend(b.iter().cloned());
        }
        (Value::Array(ref mut a), Value::Object(_)) => {
            a.push(b.clone());
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

fn merge_in(current_value: &mut Value, fields: Vec<&str>, new_value: &Value) -> io::Result<()> {
    if fields.is_empty() {
        current_value.merge(new_value);

        return Ok(());
    }

    let mut fields_iter = fields.into_iter();
    let first_field_opt = fields_iter.next();
    let remaining_fields = fields_iter.collect::<Vec<&str>>();

    let first_field: &str = match first_field_opt {
        Some(field) => field,
        None => "",
    };

    if first_field.is_empty() && remaining_fields.is_empty() {
        current_value.merge(new_value);

        return Ok(());
    }

    match current_value.pointer_mut(format!("/{}", first_field).as_str()) {
        Some(value_targeted) => merge_in(value_targeted, remaining_fields, new_value),
        // The field is not find in the object
        // Create the new field and call the merge again on this new field
        None => match (&current_value, first_field.parse::<usize>().ok()) {
            (Value::Array(vec), None) => match first_field {
                "*" => {
                    let index = vec.len();
                    let index_string = index.to_string();

                    current_value.merge(&Value::Array(vec![Value::default()]));

                    let mut new_fields = Vec::with_capacity(1 + remaining_fields.len());
                    new_fields.push(index_string.as_str());
                    new_fields.extend(remaining_fields);

                    merge_in(current_value, new_fields, new_value)
                }
                _ => {
                    let new_field = if first_field.contains('~') {
                        first_field.replace("~0", "~").replace("~1", "/")
                    } else {
                        first_field.to_string()
                    };

                    let mut map = Map::with_capacity(1);
                    map.insert(new_field, Value::default());
                    *current_value = Value::Object(map);

                    let mut new_fields = Vec::with_capacity(1 + remaining_fields.len());
                    new_fields.push(first_field);
                    new_fields.extend(remaining_fields);

                    merge_in(current_value, new_fields, new_value)
                }
            },
            (Value::Array(vec), Some(field_index)) => {
                let size = vec.len();
                let mut index = field_index;

                if index >= size {
                    current_value.merge(&Value::Array(vec![Value::default()]));
                    index = size;
                }

                let index_string = index.to_string();
                let mut new_fields = Vec::with_capacity(1 + remaining_fields.len());
                new_fields.push(index_string.as_str());
                new_fields.extend(remaining_fields);

                merge_in(current_value, new_fields, new_value)
            }
            (_, Some(_)) => {
                current_value.merge(&Value::Array(Vec::new()));

                let mut new_fields = Vec::with_capacity(1 + remaining_fields.len());
                new_fields.push("0");
                new_fields.extend(remaining_fields);

                merge_in(current_value, new_fields, new_value)
            }
            (_, None) => {
                let value = match first_field {
                    "*" => Value::Array(Vec::new()),
                    _ => {
                        let new_field = if first_field.contains('~') {
                            first_field.replace("~0", "~").replace("~1", "/")
                        } else {
                            first_field.to_string()
                        };

                        let mut map = Map::with_capacity(1);
                        map.insert(new_field, Value::default());
                        Value::Object(map)
                    }
                };

                current_value.merge(&value);

                let mut new_fields = Vec::with_capacity(1 + remaining_fields.len());
                new_fields.push(first_field);
                new_fields.extend(remaining_fields);

                merge_in(current_value, new_fields, new_value)
            }
        },
    }
}

#[cfg(test)]
mod serde_json_value_updater_test {
    use super::*;
    #[test]
    fn it_should_merge_array_string() {
        let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
        let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
        first_json_value.merge(&secound_json_value);
        assert_eq!(r#"["a","b","b","c"]"#, first_json_value.to_string());
    }
    #[test]
    fn it_should_merge_array_object() {
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
    #[test]
    fn it_should_merge_object() {
        let mut first_json_value: Value =
            serde_json::from_str(r#"{"value1":"a","value2":"b"}"#).unwrap();
        let secound_json_value: Value =
            serde_json::from_str(r#"{"value1":"a","value2":"c","value3":"d"}"#).unwrap();
        first_json_value.merge(&secound_json_value);
        assert_eq!(
            r#"{"value1":"a","value2":"c","value3":"d"}"#,
            first_json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_string() {
        let mut value_a: Value = Value::String("a".to_string());
        let value_b: Value = Value::String("b".to_string());
        value_a.merge(&value_b);
        assert_eq!(value_b.to_string(), value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_in_a_specifique_field_path() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
        value_a.merge_in("/my_array", &value_b).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_object_in_a_specifique_field_path() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array", &value_b).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_in_an_object_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/0", &value_b).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t","b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/1", &value_b).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_with_wildcard() {
        let mut value_a: Value =
            serde_json::from_str(r#"{"my_array":[{"field1":"value1"},{"field2":"value1"}]}"#)
                .unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/*/field3", &value_b).unwrap();
        assert_eq!(
            r#"{"my_array":[{"field1":"value1"},{"field2":"value1"},{"field3":{"b":"c"}}]}"#,
            value_a.to_string()
        );
    }
    #[test]
    fn it_should_merge_in_object_with_wildcard() {
        let mut json_value: Value =
            serde_json::from_str(r#"{"my_object":{"field1":null}}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        json_value
            .merge_in("/my_object/field1/*/field2", &value_b)
            .unwrap();
        assert_eq!(
            r#"{"my_object":{"field1":[{"field2":{"b":"c"}}]}}"#,
            json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_in_root_array() {
        let mut json_value: Value = serde_json::from_str(r#"["value"]"#).unwrap();
        let json_value_to_merge: Value = serde_json::from_str(r#"["new_value"]"#).unwrap();
        json_value.merge_in("/", &json_value_to_merge).unwrap();
        assert_eq!(r#"["value","new_value"]"#, json_value.to_string());
    }
    #[test]
    fn it_should_merge_in_root_object() {
        let mut json_value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
        let json_value_to_merge: Value = serde_json::from_str(r#"{"field2":"value2"}"#).unwrap();
        json_value.merge_in("/", &json_value_to_merge).unwrap();
        assert_eq!(
            r#"{"field":"value","field2":"value2"}"#,
            json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_null_in_specifique_path() {
        let mut json_value: Value = serde_json::from_str(r#"{"field":{"child":"value"}}"#).unwrap();
        let json_value_null: Value = Value::Null;
        json_value.merge_in("/field", &json_value_null).unwrap();
        assert_eq!(r#"{"field":null}"#, json_value.to_string());
    }
    #[test]
    fn it_should_replace_an_array_with_an_object() {
        let mut json_value: Value =
            serde_json::from_str(r#"{"my_array":[{"field1":"value1"}]}"#).unwrap();
        let json_value_to_merge: Value = serde_json::from_str(r#"{"a":"b"}"#).unwrap();
        json_value
            .merge_in("/my_array/T/field2", &json_value_to_merge)
            .unwrap();
        assert_eq!(
            r#"{"my_array":{"T":{"field2":{"a":"b"}}}}"#,
            json_value.to_string()
        );
    }
    #[test]
    fn it_should_build_new_object() {
        let mut object: Value = Value::default();
        object
            .merge_in("/field", &Value::String("value".to_string()))
            .unwrap();
        object
            .merge_in("/object", &Value::Object(Map::default()))
            .unwrap();
        object
            .merge_in("/array/1", &Value::Object(Map::default()))
            .unwrap();
        object
            .merge_in("/array/2", &Value::Array(Vec::default()))
            .unwrap();
        object
            .merge_in("/array/*", &Value::String("wildcard".to_string()))
            .unwrap();
        object
            .merge_in("/root/*/item", &Value::String("my_item".to_string()))
            .unwrap();
        object.merge_in("///empty", &Value::Null).unwrap();
        assert_eq!(
            r#"{"":{"":{"empty":null}},"array":[{},[],"wildcard"],"field":"value","object":{},"root":[{"item":"my_item"}]}"#,
            object.to_string()
        );
    }
}
