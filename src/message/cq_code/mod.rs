pub(crate) mod code;

use crate::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait CQCode: Serialize + DeserializeOwned {
    fn escape(s: String) -> String {
        s.replace('&', "&amp;")
            .replace('[', "&#91;")
            .replace(']', "&#93;")
            .replace(',', "&#44;")
    }

    fn anti_escape(s: &str) -> String {
        s.replace("&#44;", ",")
            .replace("&#93;", "]")
            .replace("&#91;", "[")
            .replace("&amp;", "&")
    }

    fn to_string(&self) -> String;

    fn from_string(s: String) -> Result<Self>;

    fn to_json(&self) -> Result<String>;

    fn from_json(s: &str) -> Result<Self>;
}

mod tests {
    use super::CQCode;
    use cq_code_derive::CQCode;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, CQCode)]
    struct T {
        pub a: Option<f64>,
        pub b: Option<String>,
    }

    #[test]
    fn test_to_string() {
        let t = T {
            a: Some(1.5),
            b: Some("你好".to_string()),
        };
        assert_eq!(t.to_string().as_str(), "[CQ:t,a=1.5,b=你好]")
    }

    #[test]
    fn test_to_string_with_zero() {
        let t = T {
            a: Some(0f64),
            b: Some("你好".to_string()),
        };
        assert_eq!(t.to_string().as_str(), "[CQ:t,a=0,b=你好]")
    }

    #[test]
    fn test_from_string() {
        let t: T = T::from_string("[CQ:t,a=1.5,b=你好]".to_string()).unwrap();
        assert_eq!(t.a, Some(1.5));
        assert_eq!(t.b, Some("你好".to_string()));
    }

    #[test]
    fn test_from_string_with_zero() {
        let t: T = T::from_string("[CQ:t,a=0,b=你好]".to_string()).unwrap();
        assert_eq!(t.a, Some(0f64));
        assert_eq!(t.b, Some("你好".to_string()));
    }

    #[test]
    fn test_to_json() {
        let t = T {
            a: Some(1.5),
            b: Some("你好".to_string()),
        };
        assert_eq!(t.to_json().unwrap().as_str(), r#"{"a":1.5,"b":"你好"}"#)
    }

    #[test]
    fn test_to_json_with_zero() {
        let t = T {
            a: Some(0f64),
            b: Some("你好".to_string()),
        };
        assert_eq!(t.to_json().unwrap().as_str(), r#"{"a":0.0,"b":"你好"}"#)
    }

    #[test]
    fn test_from_json() {
        let t: T = T::from_json(r#"{"a":1.5,"b":"你好"}"#).unwrap();
        assert_eq!(t.a, Some(1.5));
        assert_eq!(t.b, Some("你好".to_string()));
    }

    #[test]
    fn test_from_json_with_zero() {
        let t: T = T::from_json(r#"{"a":0,"b":"你好"}"#).unwrap();
        assert_eq!(t.a, Some(0f64));
        assert_eq!(t.b, Some("你好".to_string()));
    }
}
