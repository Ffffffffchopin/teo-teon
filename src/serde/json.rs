use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap};
use chrono::SecondsFormat;
use crate::Value;

impl Serialize for Value {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Value::Null => serializer.serialize_none(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Int(i) => serializer.serialize_i32(*i),
            Value::Int64(i) => serializer.serialize_i64(*i),
            Value::Float32(f) => serializer.serialize_f32(*f),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::Decimal(d) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$decimal", &d.normalized().to_string())?;
                map.end()
            }
            Value::ObjectId(o) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$objectId", &o.to_hex())?;
                map.end()
            }
            Value::String(s) => serializer.serialize_str(s),
            Value::Date(d) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$date", &d.format("%Y-%m-%d").to_string())?;
                map.end()
            }
            Value::DateTime(d) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$datetime", &d.to_rfc3339_opts(SecondsFormat::Millis, true))?;
                map.end()
            }
            Value::Array(a) => serializer.collect_seq(a),
            Value::Dictionary(d) => serializer.collect_map(d),
            Value::Range(r) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$range", &r)?;
                map.end()
            }
            Value::Tuple(t) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$tuple", &t)?;
                map.end()
            }
            Value::EnumVariant(e) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$enumVariant", &e)?;
                map.end()
            }
            Value::OptionVariant(o) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$optionVariant", &o)?;
                map.end()
            }
            Value::Regex(r) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$regex", &r.to_string())?;
                map.end()
            }
            Value::File(f) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$file", &f)?;
                map.end()
            }
        }
    }
}
