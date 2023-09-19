mod cq_code;
use crate::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub enum MessageType {
    String,
    Json,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Message {
    pub messages: Vec<String>,
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        match self.message_type() {
            MessageType::String => self.messages.join(""),
            MessageType::Json => format!("[{}]", self.messages.join(",")),
        }
    }
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        if self.messages.is_empty() {
            return MessageType::String;
        }
        if self.messages[0].starts_with('{') {
            MessageType::Json
        } else {
            MessageType::String
        }
    }

    pub fn from_string(s: String) -> Result<Self> {
        let mut messages = Vec::new();
        if s.is_empty() {
            messages.push("".to_string());
            return Ok(Self { messages });
        }
        let re = Regex::new(r"\[CQ:(?P<name>\w+)(?P<fields>(,\w+=[^,\]]+)*)]")?;
        let mut start = 0;
        for cap in re.captures_iter(&s) {
            let name = cap.name("name").unwrap().as_str();
            let fields = cap.name("fields").unwrap().as_str();
            let end = cap.get(0).unwrap().start();
            if start != end {
                messages.push(s[start..end].to_string());
            }
            start = cap.get(0).unwrap().end();
            messages.push(format!("[CQ:{}{}]", name, fields));
        }
        if start != s.len() {
            messages.push(s[start..].to_string());
        }
        Ok(Self { messages })
    }

    pub fn from_json(s: &str) -> Result<Self> {
        let mut messages = Vec::new();
        let re = Regex::new(r"\{.*?}}")?;
        for cap in re.captures_iter(s) {
            messages.push(cap.get(0).unwrap().as_str().to_string());
        }
        Ok(Self { messages })
    }
}

impl FromStr for Message {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        // 判断是否为 JSON 格式
        if s.starts_with("[{") && s.ends_with("}]") {
            Self::from_json(s)
        } else {
            Self::from_string(s.to_string())
        }
    }
}

#[macro_export]
macro_rules! message_from_strings {
    ($($x:expr),*) => {
        Message {
            messages: vec![$($x.to_string()),*],
        }
    }
}

#[macro_export]
macro_rules! message_from_jsons {
    ($($x:expr),*) => {
        Message {
            messages: vec![$($x.to_json().unwrap()),*],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::cq_code::code::Face;
    use super::cq_code::CQCode;
    use super::Message;

    #[test]
    fn test_message_from_str0() {
        let s = "";
        let m: Message = s.parse().unwrap();
        assert_eq!(m.messages.len(), 1);
        assert_eq!(m.messages[0], "");
        assert_eq!(m.to_string(), "");
    }

    #[test]
    fn test_message_from_str1() {
        let s = "你好世界";
        let m: Message = s.parse().unwrap();
        assert_eq!(m.messages.len(), 1);
        assert_eq!(m.messages[0], "你好世界");
        assert_eq!(m.to_string(), "你好世界");
    }

    #[test]
    fn test_message_from_str2() {
        let s = "[CQ:at,qq=123]";
        let m: Message = s.parse().unwrap();
        assert_eq!(m.messages.len(), 1);
        assert_eq!(m.messages[0], "[CQ:at,qq=123]");
        assert_eq!(m.to_string(), "[CQ:at,qq=123]");
    }

    #[test]
    fn test_message_from_str3() {
        let s = "你好[CQ:at,qq=123]世界[CQ:at,qq=456]你好";
        let m: Message = s.parse().unwrap();
        assert_eq!(m.messages.len(), 5);
        assert_eq!(m.messages[0], "你好");
        assert_eq!(m.messages[1], "[CQ:at,qq=123]");
        assert_eq!(m.messages[2], "世界");
        assert_eq!(m.messages[3], "[CQ:at,qq=456]");
        assert_eq!(m.messages[4], "你好");
        assert_eq!(m.to_string(), "你好[CQ:at,qq=123]世界[CQ:at,qq=456]你好");
    }

    #[test]
    fn test_message_from_str4() {
        let s = r#"[{"type":"face","data":{"id":1}}]"#;
        let m: Message = s.parse().unwrap();
        assert_eq!(m.messages.len(), 1);
        assert_eq!(m.messages[0], r#"{"type":"face","data":{"id":1}}"#);
        assert_eq!(m.to_string(), r#"[{"type":"face","data":{"id":1}}]"#);
    }

    #[test]
    fn test_message_from_str5() {
        let s = r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}}]"#;
        let m: Message = s.parse().unwrap();
        assert_eq!(m.messages.len(), 2);
        assert_eq!(m.messages[0], r#"{"type":"face","data":{"id":1}}"#);
        assert_eq!(m.messages[1], r#"{"type":"face","data":{"id":2}}"#);
        assert_eq!(
            m.to_string(),
            r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}}]"#
        );
    }

    #[test]
    fn test_message_from_str6() {
        let s = r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}},{"type":"text","data":{"text":"你好世界"}}]"#;
        let m: Message = s.parse().unwrap();
        assert_eq!(m.messages.len(), 3);
        assert_eq!(m.messages[0], r#"{"type":"face","data":{"id":1}}"#);
        assert_eq!(m.messages[1], r#"{"type":"face","data":{"id":2}}"#);
        assert_eq!(
            m.messages[2],
            r#"{"type":"text","data":{"text":"你好世界"}}"#
        );
        assert_eq!(
            m.to_string(),
            r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}},{"type":"text","data":{"text":"你好世界"}}]"#
        );
    }

    #[test]
    fn test_message_from_json0() {
        let s = "";
        let m = Message::from_json(s).unwrap();
        assert!(m.messages.is_empty());
        assert_eq!(m.to_string(), "");
    }

    #[test]
    fn test_message_from_json1() {
        let s = r#"[{"type":"face","data":{"id":1}}]"#;
        let m = Message::from_json(s).unwrap();
        assert_eq!(m.messages.len(), 1);
        assert_eq!(m.messages[0], r#"{"type":"face","data":{"id":1}}"#);
        assert_eq!(m.to_string(), r#"[{"type":"face","data":{"id":1}}]"#);
    }

    #[test]
    fn test_message_from_json2() {
        let s = r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}}]"#;
        let m = Message::from_json(s).unwrap();
        assert_eq!(m.messages.len(), 2);
        assert_eq!(m.messages[0], r#"{"type":"face","data":{"id":1}}"#);
        assert_eq!(m.messages[1], r#"{"type":"face","data":{"id":2}}"#);
        assert_eq!(
            m.to_string(),
            r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}}]"#
        );
    }

    #[test]
    fn test_message_from_json3() {
        let s = r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}},{"type":"text","data":{"text":"你好世界"}}]"#;
        let m = Message::from_json(s).unwrap();
        assert_eq!(m.messages.len(), 3);
        assert_eq!(m.messages[0], r#"{"type":"face","data":{"id":1}}"#);
        assert_eq!(m.messages[1], r#"{"type":"face","data":{"id":2}}"#);
        assert_eq!(
            m.messages[2],
            r#"{"type":"text","data":{"text":"你好世界"}}"#
        );
        assert_eq!(
            m.to_string(),
            r#"[{"type":"face","data":{"id":1}},{"type":"face","data":{"id":2}},{"type":"text","data":{"text":"你好世界"}}]"#
        );
    }

    #[test]
    fn test_macro_message_from_strings0() {
        let m = message_from_strings!();
        assert!(m.messages.is_empty());
        assert_eq!(m.to_string(), "");
    }

    #[test]
    fn test_macro_message_from_strings1() {
        let m = message_from_strings!("你好世界");
        assert_eq!(m.messages.len(), 1);
        assert_eq!(m.messages[0], "你好世界");
        assert_eq!(m.to_string(), "你好世界");
    }

    #[test]
    fn test_macro_message_from_strings2() {
        let m = message_from_strings!("你好世界", Face { id: Some(1) });
        assert_eq!(m.messages.len(), 2);
        assert_eq!(m.messages[0], "你好世界");
        assert_eq!(m.messages[1], "[CQ:face,id=1]");
        assert_eq!(m.to_string(), "你好世界[CQ:face,id=1]");
    }

    #[test]
    fn test_macro_message_from_jsons0() {
        let m = message_from_jsons!();
        assert!(m.messages.is_empty());
        assert_eq!(m.to_string(), "");
    }

    #[test]
    fn test_macro_message_from_jsons1() {
        let m = message_from_jsons!(Face { id: Some(1) });
        assert_eq!(m.messages.len(), 1);
        assert_eq!(m.messages[0], "{\"type\":\"face\",\"data\":{\"id\":1}}");
        assert_eq!(m.to_string(), "[{\"type\":\"face\",\"data\":{\"id\":1}}]");
    }

    #[test]
    fn test_macro_message_from_jsons2() {
        let m = message_from_jsons!(Face { id: Some(1) }, Face { id: Some(2) });
        assert_eq!(m.messages.len(), 2);
        assert_eq!(m.messages[0], "{\"type\":\"face\",\"data\":{\"id\":1}}");
        assert_eq!(m.messages[1], "{\"type\":\"face\",\"data\":{\"id\":2}}");
        assert_eq!(
            m.to_string(),
            "[{\"type\":\"face\",\"data\":{\"id\":1}},{\"type\":\"face\",\"data\":{\"id\":2}}]"
        );
    }
}
