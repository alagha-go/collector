use super::*;
use serde_json::{Value};

impl From<Value> for Gender {
    fn from(value: Value) -> Self {
        match value {
            Value::String(string) => string.into(),
            Value::Number(int) => {
                let number = match int.as_f64() {
                    None => 0,
                    Some(number) => number as u8
                };
                match number {
                    1 => Self::Female,
                    2 => Self::Male,
                    3 => Self::NonBinary,
                    _ => Self::UnSpedified
                }
            },
            _ => Self::UnSpedified
        }
    }
}

impl From<String> for Gender {
    fn from(s: String) -> Self {
        let s = s.to_lowercase();
        let s = s.as_str();
        match s {
            "female" => Self::Female,
            "male" => Self::Male,
            "nonbinary" => Self::NonBinary,
            _ => Self::UnSpedified
        }
    }
}