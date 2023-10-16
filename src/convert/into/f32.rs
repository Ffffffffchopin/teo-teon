use crate::error::Error;
use crate::value::Value;

// impl TryInto<f32> for Value {

//     type Error = Error;

//     fn try_into(self) -> Result<f32, Self::Error> {
//         match self {
//             Value::Float32(b) => Ok(b),
//             _ => Err(Error::new(format!("Cannot convert {} into f32", self.type_hint()))),
//         }
//     }
// }

// impl TryInto<f32> for &Value {

//     type Error = Error;

//     fn try_into(self) -> Result<f32, Self::Error> {
//         match self {
//             Value::Float32(b) => Ok(*b),
//             _ => Err(Error::new(format!("Cannot convert {} into f32", self.type_hint()))),
//         }
//     }
// }

impl TryFrom<Value> for f32 {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into f32", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for f32 {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into f32", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<f32> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(i) => Ok(Some(i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f32>", value.type_hint()))),
        }
    }
}

// impl TryInto<Option<f32>> for Value {

//     type Error = Error;

//     fn try_into(self) -> Result<Option<f32>, Self::Error> {
//         match self {
//             Value::Null => Ok(None),
//             Value::Float32(b) => Ok(Some(b)),
//             _ => Err(Error::new(format!("Cannot convert {} into Option<f32>", self.type_hint()))),
//         }
//     }
// }

// impl TryInto<Option<f32>> for &Value {

//     type Error = Error;

//     fn try_into(self) -> Result<Option<f32>, Self::Error> {
//         match self {
//             Value::Null => Ok(None),
//             Value::Float32(b) => Ok(Some(*b)),
//             _ => Err(Error::new(format!("Cannot convert {} into Option<f32>", self.type_hint()))),
//         }
//     }
// }

impl TryFrom<&Value> for Option<f32> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(i) => Ok(Some(*i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f32>", value.type_hint()))),
        }
    }
}

