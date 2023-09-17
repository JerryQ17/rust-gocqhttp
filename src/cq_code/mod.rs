// use serde::de::DeserializeOwned;
// use serde::Serialize;

pub trait CQCode: Sized {
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

    fn from_string(s: String) -> crate::Result<Self>;
    //
    // fn to_json(&self) -> Result<String>;
    //
    // fn from_json(s: &str) -> Result<Self>;
}

mod tests {
    use super::CQCode;
    use cq_code_derive::CQCode;
    // use serde::{Deserialize, Serialize};

    #[derive(Debug, CQCode)]
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
        assert_eq!(t.to_string().as_str(), "[CQ:T,a=1.5,b=你好]")
    }

    #[test]
    fn test_from_string() {
        let t: T = T::from_string("[CQ:T,a=1.5,b=你好]".to_string()).unwrap();
        assert_eq!(t.a, Some(1.5));
        assert_eq!(t.b, Some("你好".to_string()));
    }
}
