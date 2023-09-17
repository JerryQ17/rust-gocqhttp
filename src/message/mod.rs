mod cq_code;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub messages: Vec<String>,
}

impl ToString for Message {
    fn to_string(&self) -> String {
        self.messages.join("")
    }
}

impl FromStr for Message {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
