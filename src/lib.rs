extern crate serde_json;

use serde_json::{Map, Value};
use std::io;

/// Trait used to merge Json Values
pub trait Merge {
    /// Method use to merge two Json Values : ValueA <- ValueB.
    fn merge(&mut self, new_json_value: Value);
    /// Merge a new value in specific json pointer. If the field can't be merge in the specific path, it raise an error.
    fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) -> io::Result<()>;
}

impl Merge for serde_json::Value {
    /// # Examples: Merge two array together.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut array1: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    /// let array2: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// array1.merge(array2);
    /// assert_eq!(r#"["a","b","b","c"]"#, array1.to_string());
    /// ```
    /// # Examples: Merge two objects together.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"value1":"a","value2":"b"}"#).unwrap();
    /// let object2: Value = serde_json::from_str(r#"{"value1":"a","value2":"c","value3":"d"}"#).unwrap();
    /// object1.merge(object2);
    /// assert_eq!(r#"{"value1":"a","value2":"c","value3":"d"}"#,object1.to_string());
    /// ```
    /// # Examples: Merge an object into an array.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut array: Value = serde_json::from_str(r#"[]"#).unwrap();
    /// let object: Value = serde_json::from_str(r#"{"field1":"value1"}"#).unwrap();
    /// array.merge(object);
    /// assert_eq!(r#"[{"field1":"value1"}]"#,array.to_string());
    /// ```
    /// # Examples: Merge an array into an object.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object: Value = serde_json::from_str(r#"{"field1":"value1"}"#).unwrap();
    /// let array: Value = serde_json::from_str(r#"["value2","value3"]"#).unwrap();
    /// object.merge(array);
    /// assert_eq!(r#"["value2","value3"]"#,object.to_string());
    /// ```
    fn merge(&mut self, new_json_value: Value) {
        merge(self, &new_json_value);
    }
    /// # Examples: Merge an array in an object in a specific position.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let array: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// object.merge_in("/my_array", array.clone());
    /// assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, object.to_string());
    /// ```
    /// # Examples: Merge two objects together in a specific position.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let object2: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
    /// object1.merge_in("/my_array/0/a", object2.clone());
    /// assert_eq!(r#"{"my_array":[{"a":{"b":"c"}}]}"#, object1.to_string());
    /// ```
    /// # Examples: Merge an object in an array in a specific position. If the position not exist, the object is added in the array.
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut json_value: Value = serde_json::from_str(r#"[{"array1":[{"field":"value1"}]}]"#).unwrap();
    /// let result = json_value.merge_in("/1", Value::String("value".to_string()));
    ///
    /// assert_eq!(r#"[{"array1":[{"field":"value1"}]},"value"]"#,json_value.to_string());
    /// ```
    /// # Examples: Merge an object in an array in a wrong position and generate error
    /// ```
    /// use serde_json::Value;
    /// use json_value_merge::Merge;
    ///
    /// let mut json_value: Value = serde_json::from_str(r#"[{"array1":[{"field":"value1"}]}]"#).unwrap();
    /// let result = json_value.merge_in("/other_field", Value::String("value".to_string()));
    ///
    /// assert!(result.is_err(), "The result should be in error because it's not possible to find or add an object field in an array");
    /// assert_eq!(r#"[{"array1":[{"field":"value1"}]}]"#,json_value.to_string());
    /// ```
    /// # Examples: Build a new object.
    /// ```
    /// use serde_json::{Map,Value};
    /// use json_value_merge::Merge;
    ///
    /// let mut object: Value = Value::default();
    /// object.merge_in("/field", Value::String("value".to_string()));
    /// object.merge_in("/object", Value::Object(Map::default()));
    /// object.merge_in("/array", Value::Array(Vec::default()));
    /// object.merge_in("/1", Value::Object(Map::default()));
    /// object.merge_in("/array/1", Value::Object(Map::default()));
    /// object.merge_in("/array/2", Value::Array(Vec::default()));
    /// assert_eq!(r#"{"1":{},"array":[{},[]],"field":"value","object":{}}"#,object.to_string());
    /// ```
    fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) -> io::Result<()> {
        let fields: Vec<&str> = json_pointer.split('/').skip(1).collect();

        merge_in(self, fields, new_json_value)
    }
}

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (Value::Array(ref mut a), &Value::Array(ref b)) => {
            a.extend(b.clone());
        }
        (Value::Array(ref mut a), &Value::Object(ref b)) => {
            a.extend([Value::Object(b.clone())]);
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

fn merge_in(json_value: &mut Value, fields: Vec<&str>, new_json_value: Value) -> io::Result<()> {
    if fields.is_empty() {
        json_value.merge(new_json_value);

        return Ok(());
    }

    let mut fields = fields.clone();
    let field = fields.remove(0);

    // Controle the field
    if field.is_empty() {
        json_value.merge(new_json_value);

        return Ok(());
    }

    // It's not possible to add or find an object field into an array
    //  - current object: [{"field":"value"}]
    //  - json pointer pattern : "/field_2"
    //  - Result: raise an error
    //
    // The json pointer pattern for merging value into an array, should be:
    // - /0/field_2
    // - /
    match (&json_value, field.parse::<usize>().ok()) {
        (Value::Array(_), None) => return Err(io::Error::new(io::ErrorKind::NotFound, format!("The field '{}' can't be found or created in this array '{}'. Change the json pointer pattern.", field, &json_value.to_string()))),
        (_,_) => ()
    };

    match json_value.pointer_mut(format!("/{}", field).as_str()) {
        // Find the field and the json_value_targeted.
        Some(json_targeted) => {
            if !fields.is_empty() {
                merge_in(json_targeted, fields, new_json_value)?;
            } else {
                json_targeted.merge(new_json_value);
            }
        }
        // The field is not find in the object
        // Create the new field and call the merge again on this new field
        None => match json_value.clone() {
            Value::Array(vec) => {
                json_value.merge(Value::Array(vec![Value::default()]));

                let size = vec.len().to_string();
                let mut new_fields = vec![size.as_str()];
                new_fields.append(&mut fields);

                merge_in(json_value, new_fields, new_json_value)?;
            }
            _ => {
                let mut map = Map::default();
                map.insert(field.to_string(), Value::default());
                json_value.merge(Value::Object(map));

                let mut new_fields = vec![field];
                new_fields.append(&mut fields);

                merge_in(json_value, new_fields, new_json_value)?;
            }
        },
    };

    Ok(())
}

#[cfg(test)]
mod serde_json_value_updater_test {
    use super::*;
    #[test]
    fn it_should_merge_array_string() {
        let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
        let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
        first_json_value.merge(secound_json_value);
        assert_eq!(r#"["a","b","b","c"]"#, first_json_value.to_string());
    }
    #[test]
    fn it_should_merge_array_object() {
        let mut first_json_value: Value =
            serde_json::from_str(r#"[{"value":"a"},{"value":"b"}]"#).unwrap();
        let secound_json_value: Value =
            serde_json::from_str(r#"[{"value":"b"},{"value":"c"}]"#).unwrap();
        first_json_value.merge(secound_json_value);
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
        value_a.merge_in("/my_array", value_b.clone()).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_object_in_a_specifique_field_path() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array", value_b.clone()).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_in_an_object_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/0", value_b.clone()).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t","b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/1", value_b.clone()).unwrap();
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_in_root_array() {
        let mut json_value: Value = serde_json::from_str(r#"["value"]"#).unwrap();
        let json_value_to_merge: Value = serde_json::from_str(r#"["new_value"]"#).unwrap();
        json_value.merge_in("/", json_value_to_merge).unwrap();
        assert_eq!(r#"["value","new_value"]"#, json_value.to_string());
    }
    #[test]
    fn it_should_merge_in_root_object() {
        let mut json_value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
        let json_value_to_merge: Value = serde_json::from_str(r#"{"field2":"value2"}"#).unwrap();
        json_value.merge_in("/", json_value_to_merge).unwrap();
        assert_eq!(
            r#"{"field":"value","field2":"value2"}"#,
            json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_null_in_specifique_path() {
        let mut json_value: Value = serde_json::from_str(r#"{"field":{"child":"value"}}"#).unwrap();
        let json_value_null: Value = Value::Null;
        json_value.merge_in("/field", json_value_null).unwrap();
        assert_eq!(r#"{"field":null}"#, json_value.to_string());
    }
}
