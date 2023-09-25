use serde_json::{Value as JsonValue, Number as JsonNumber, Map as JsonMap, json};
use super::value::Value;
use chrono::SecondsFormat;

impl Into<JsonValue> for Value {
    fn into(self) -> JsonValue {
        match self {
            Value::Null => {
                JsonValue::Null
            }
            Value::ObjectId(val) => {
                JsonValue::String(val.to_hex())
            }
            Value::Bool(val) => {
                JsonValue::Bool(val.clone())
            }
            Value::I32(val) => {
                JsonValue::Number(JsonNumber::from(val))
            }
            Value::I64(val) => {
                JsonValue::Number(JsonNumber::from(val))
            }
            Value::F32(val) => {
                JsonValue::Number(JsonNumber::from_f64(val as f64).unwrap())
            }
            Value::F64(val) => {
                JsonValue::Number(JsonNumber::from_f64(val).unwrap())
            }
            Value::Decimal(val) => {
                json!({"$decimal": val.normalized().to_string() })
            }
            Value::String(val) => {
                JsonValue::String(val.clone())
            }
            Value::Date(val) => {
                JsonValue::String(val.format("%Y-%m-%d").to_string())
            }
            Value::DateTime(val) => {
                json!({"$date": val.to_rfc3339_opts(SecondsFormat::Millis, true)})
            }
            Value::Vec(val) => {
                JsonValue::Array(val.iter().map(|v| v.into()).collect())
            }
            Value::HashMap(val) => {
                let mut map = JsonMap::new();
                for (k, v) in val {
                    map.insert(k.to_string(), v.into());
                }
                JsonValue::Object(map)
            }
            Value::BTreeMap(val) => {
                let mut map = JsonMap::new();
                for (k, v) in val {
                    map.insert(k.to_string(), v.into());
                }
                JsonValue::Object(map)
            }
            Value::IndexMap(val) => {
                let mut map = JsonMap::new();
                for (k, v) in val {
                    map.insert(k.to_string(), v.into());
                }
                JsonValue::Object(map)
            }
            _ => {
                panic!("Cannot convert into json.")
            }
        }
    }
}

impl Into<JsonValue> for &Value {
    fn into(self) -> JsonValue {
        self.clone().into()
    }
}
