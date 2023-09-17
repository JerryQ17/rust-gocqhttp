use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait CQCode: Serialize + DeserializeOwned {
    fn to_string(&self) -> String;

    // fn from_string(s: String) -> Result<Self>;
    //
    // fn to_json(&self) -> Result<String>;
    //
    // fn from_json(s: &str) -> Result<Self>;
}

#[cfg(test)]
mod tests {
    use super::CQCode;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, cq_code_derive::CQCode)]
    struct T {
        a: Option<f64>,
        b: Option<String>,
    }

    #[test]
    fn test_to_string() {
        let t = T {
            a: Some(1.5),
            b: Some("你好".to_string()),
        };
        assert_eq!(t.to_string().as_str(), "[CQ:T,a=1.5,b=你好]")
    }
}
