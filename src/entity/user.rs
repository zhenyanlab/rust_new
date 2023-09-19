use serde::Deserialize;
use serde::Serialize;

#[warn(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct P {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
}

impl P {
    pub fn new() -> P {
        P {
            id: 0,
            name: String::from(""),
            age: 0,
            address: String::from(""),
        }
    }
}
