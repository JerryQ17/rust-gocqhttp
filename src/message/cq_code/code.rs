#![allow(non_snake_case)] // 某个字段的命名不符合规范，但是为了兼容go-cqhttp，所以不改了

use super::CQCode;
use crate::message::Message;
use cq_code_derive::CQCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

/// 在某些CQ码中，部分字段的可能值仅有`0`, `1`，可以视为`bool`类型。
/// 但是，不论在字符串还是文本形式中，这些字段的值是`0`, `1`而不是`true`, `false`。
///
/// 这就导致在序列化和反序列化的过程中，这些字段不能被Serde正确地解析。
///
/// 为了解决这个问题，我们将这些字段的类型定义为`BoolInCQCode`，其等同于`i8`。
pub type BoolInCQCode = i8;

/// [QQ表情](https://docs.go-cqhttp.org/cqcode/#qq-%E8%A1%A8%E6%83%85)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Face {
    /// QQ表情ID, 见[QQ表情ID表](https://github.com/richardchien/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)
    pub id: Option<i32>,
}

/// [语音](https://docs.go-cqhttp.org/cqcode/#%E8%AF%AD%E9%9F%B3)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Record {
    /// 语音文件名
    pub file: Option<String>,
    /// 发送时可选, 默认0, 设置为1表示变声
    pub magic: Option<BoolInCQCode>,
    /// 语音 URL
    pub url: Option<String>,
    /// 只在通过网络URL发送时有效, 表示是否使用已缓存的文件, 默认1
    pub cache: Option<BoolInCQCode>,
    /// 只在通过网络URL发送时有效, 表示是否通过代理下载文件(需通过环境变量或配置文件配置代理), 默认1
    pub proxy: Option<BoolInCQCode>,
    /// 只在通过网络URL发送时有效, 单位秒, 表示下载网络文件的超时时间, 默认不超时
    pub timeout: Option<i32>,
}

/// [短视频](https://docs.go-cqhttp.org/cqcode/#%E7%9F%AD%E8%A7%86%E9%A2%91)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Video {
    /// 视频地址, 支持http和file发送
    pub file: Option<String>,
    /// 视频封面, 支持http, file和base64发送, 格式必须为jpg
    pub cover: Option<String>,
    /// 通过网络下载视频时的线程数, 默认单线程. (在资源不支持并发时会自动处理)
    pub c: Option<i32>,
}

/// [@某人](https://docs.go-cqhttp.org/cqcode/#%E6%9F%90%E4%BA%BA)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct At {
    /// @的QQ号, all表示全体成员
    pub qq: Option<String>,
    /// 当在群中找不到此QQ号的名称时才会生效
    pub name: Option<String>,
}

/// [猜拳魔法表情](https://docs.go-cqhttp.org/cqcode/#%E7%8C%9C%E6%8B%B3%E9%AD%94%E6%B3%95%E8%A1%A8%E6%83%85)
///
/// **注意**：暂未被go-cqhttp支持
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Rps {}

/// [掷骰子魔法表情](https://docs.go-cqhttp.org/cqcode/#%E6%8E%B7%E9%AA%B0%E5%AD%90%E9%AD%94%E6%B3%95%E8%A1%A8%E6%83%85)
///
/// **注意**：暂未被go-cqhttp支持
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Dice {}

/// [窗口抖动（戳一戳）](https://docs.go-cqhttp.org/cqcode/#%E7%AA%97%E5%8F%A3%E6%8A%96%E5%8A%A8-%E6%88%B3%E4%B8%80%E6%88%B3)
///
/// **注意**：暂未被go-cqhttp支持
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Shake {}

/// [匿名发消息](https://docs.go-cqhttp.org/cqcode/#%E5%8C%BF%E5%90%8D%E5%8F%91%E6%B6%88%E6%81%AF)
///
/// **注意**：暂未被go-cqhttp支持
///
/// 提示：当收到匿名消息时, 需要通过`消息事件的群消息`的anonymous字段判断
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Anonymous {
    /// 可选, 表示无法匿名时是否继续发送
    pub ignore: Option<BoolInCQCode>,
}

/// [链接分享](https://docs.go-cqhttp.org/cqcode/#%E9%93%BE%E6%8E%A5%E5%88%86%E4%BA%AB)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Share {
    /// URL
    pub url: Option<String>,
    /// 标题
    pub title: Option<String>,
    /// 发送时可选, 内容描述
    pub content: Option<String>,
    /// 发送时可选, 图片URL
    pub image: Option<String>,
}

/// [推荐好友/群](https://docs.go-cqhttp.org/cqcode/#%E6%8E%A8%E8%8D%90%E5%A5%BD%E5%8F%8B-%E7%BE%A4)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Contact {
    /// 类型, group或friend
    pub type_: Option<String>,
    /// QQ号或群号
    pub id: Option<String>,
}

/// [位置](https://docs.go-cqhttp.org/cqcode/#%E4%BD%8D%E7%BD%AE)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Location {
    /// 经度
    pub lon: Option<f64>,
    /// 纬度
    pub lat: Option<f64>,
    /// 发送时可选, 标题
    pub title: Option<String>,
    /// 发送时可选, 内容描述
    pub content: Option<String>,
}

/// [音乐分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E5%88%86%E4%BA%AB)
///
/// [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB)
///
/// 由于这两类的`type`相同，所以合并为一个结构体
///
/// 1. 公有字段：`type_`
/// 2. [音乐分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E5%88%86%E4%BA%AB)私有字段：`id`
/// 3. [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB)私有字段：`url`, `audio`, `title`, `content`, `image`
///
/// **注意**：这两类的字段不同，使用时请务必查看文档。本类在序列化时，会根据`type`字段自动选择序列化的字段，如果`type`字段不匹配，不再序列化其它字段，直接返回`[CQ:music,type=<your_wrong_input>]`
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Music {
    /// [音乐分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E5%88%86%E4%BA%AB): 可选值为`qq`, `163`, `xm`分别表示使用QQ音乐、网易云音乐、虾米音乐，此时需要填写`id`字段
    ///
    /// [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB): 可选值为`custom`，此时需要填写`url`、`audio`、`title`、`content`、`image`字段
    pub type_: Option<String>,
    /// [音乐分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E5%88%86%E4%BA%AB)私有字段，歌曲ID
    pub id: Option<String>,
    /// [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB)私有字段，点击后跳转目标URL
    pub url: Option<String>,
    /// [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB)私有字段，音乐URL
    pub audio: Option<String>,
    /// [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB)私有字段，标题
    pub title: Option<String>,
    /// [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB)私有字段，发送时可选，内容描述
    pub content: Option<String>,
    /// [音乐自定义分享](https://docs.go-cqhttp.org/cqcode/#%E9%9F%B3%E4%B9%90%E8%87%AA%E5%AE%9A%E4%B9%89%E5%88%86%E4%BA%AB)私有字段，发送时可选，图片URL
    pub image: Option<String>,
}

/// [图片](https://docs.go-cqhttp.org/cqcode/#%E5%9B%BE%E7%89%87)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Image {
    /// 图片文件名，支持：
    /// - 绝对路径，例如 `file:///C:\\Users\Alice\Pictures\1.png`，格式使用 [file URI](https://tools.ietf.org/html/rfc8089)
    /// - 网络 URL，例如 `https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png`
    /// - Base64 编码，例如 `base64://iVBORw0KGg==`
    pub file: Option<String>,
    /// 图片类型, flash表示闪照, show表示秀图, 默认普通图片
    pub type_: Option<String>,
    /// 图片子类型, 只出现在群聊
    ///
    /// |value|说明|
    /// |:-:|:-:|
    /// |0|正常图片|
    /// |1|表情包, 在客户端会被分类到表情包图片并缩放显示|
    /// |2|热图|
    /// |3|斗图|
    /// |4|智图?|
    /// |7|贴图|
    /// |8|自拍|
    /// |9|贴图广告?|
    /// |10|有待测试|
    /// |13|热搜图|
    pub subType: Option<String>,
    /// 发送时可选, 图片URL
    pub url: Option<String>,
    /// 只在通过网络URL发送时有效, 表示是否使用已缓存的文件, 默认1
    pub cache: Option<BoolInCQCode>,
    /// 发送秀图时的特效id, 默认为40000
    ///
    /// |id|类型|
    /// |:-:|:-:|
    /// |40000|普通|
    /// |40001|幻影|
    /// |40002|抖动|
    /// |40003|生日|
    /// |40004|爱你|
    /// |40005|征友|
    pub id: Option<i32>,
    /// 通过网络下载图片时的线程数, 默认单线程. (在资源不支持并发时会自动处理)
    pub c: Option<i32>,
}

/// [回复](https://docs.go-cqhttp.org/cqcode/#%E5%9B%9E%E5%A4%8D)
///
/// 提示：如果`id`和`text`同时存在, 将采用自定义reply并替换原有信息。如果id获取失败, 将回退到自定义reply
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Reply {
    /// 回复时所引用的消息id, 必须为本群消息.
    pub id: Option<i32>,
    /// 自定义回复的信息
    pub text: Option<String>,
    /// 自定义回复时的自定义QQ, 如果使用自定义信息必须指定.
    pub qq: Option<i64>,
    /// 自定义回复时的时间, 格式为Unix时间
    pub time: Option<i64>,
    /// 起始消息序号, 可通过`get_msg`获得
    pub seq: Option<i64>,
}

/// [红包](https://docs.go-cqhttp.org/cqcode/#%E7%BA%A2%E5%8C%85)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct RedBag {
    /// 祝福语/口令
    pub title: Option<String>,
}

/// [戳一戳](https://docs.go-cqhttp.org/cqcode/#%E6%88%B3%E4%B8%80%E6%88%B3)
///
/// **注意**：发送戳一戳消息无法撤回, 返回的`message id`恒定为0
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Poke {
    /// 需要戳的成员
    pub qq: Option<i64>,
}

/// [礼物](https://docs.go-cqhttp.org/cqcode/#%E7%A4%BC%E7%89%A9)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Gift {
    /// 接收礼物的成员
    pub qq: Option<i64>,
    /// 礼物的类型
    ///
    /// |id|类型|
    /// |:-:|:-:|
    /// |0|甜Wink|
    /// |1|快乐肥宅水|
    /// |2|幸运手链|
    /// |3|卡布奇诺|
    /// |4|猫咪手表|
    /// |5|绒绒手套|
    /// |6|彩虹糖果|
    /// |7|坚强|
    /// |8|告白话筒|
    /// |9|牵你的手|
    /// |10|可爱猫咪|
    /// |11|神秘面具|
    /// |12|我超忙的|
    /// |13|爱心口罩|
    pub id: Option<i32>,
}

/// [合并转发](https://docs.go-cqhttp.org/cqcode/#%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Forward {
    /// 合并转发ID, 需要通过`/get_forward_msg`API获取转发的具体内容
    pub id: Option<i32>,
}

/// [合并转发消息节点](https://docs.go-cqhttp.org/cqcode/#%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91%E6%B6%88%E6%81%AF%E8%8A%82%E7%82%B9)
///
/// 特殊说明: 需要使用单独的API`/send_group_forward_msg`发送, 并且由于消息段较为复杂, 仅支持Array形式入参。
/// 如果引用消息和自定义消息同时出现, 实际查看顺序将取消息段顺序.
/// 另外按`Onebot v11`文档说明, data 应全为字符串, 但由于需要接收message类型的消息, 所以仅限此Type的content字段支持Array套娃
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    /// 转发消息id，直接引用他人的消息合并转发, 实际查看顺序为原消息发送顺序 与下面的自定义消息二选一
    pub id: Option<i32>,
    /// 发送者显示名字, 用于自定义消息(自定义消息并合并转发, 实际查看顺序为自定义消息段顺序)
    pub name: Option<String>,
    /// 发送者QQ号, 用于自定义消息
    pub uin: Option<i64>,
    /// 用于自定义消息 不支持转发套娃
    pub content: Option<Message>,
    /// 具体消息, 用于自定义消息
    pub seq: Option<Message>,
}

impl CQCode for Node {
    fn to_string(&self) -> String {
        let mut s = String::from("[CQ:node");
        if let Some(id) = &self.id {
            s.push_str(format!(",id={}", id).as_str());
        }
        if let Some(name) = &self.name {
            s.push_str(format!(",name={}", name).as_str());
        }
        if let Some(uin) = &self.uin {
            s.push_str(format!(",uin={}", uin).as_str());
        }
        if let Some(content) = &self.content {
            s.push_str(format!(",content={}", content.to_string()).as_str());
        }
        if let Some(seq) = &self.seq {
            s.push_str(format!(",seq={}", seq.to_string()).as_str());
        }
        s
    }

    fn from_string(s: String) -> crate::error::Result<Self> {
        let mut id = None;
        let mut name = None;
        let mut uin = None;
        let mut content = None;
        let mut seq = None;
        let re = regex::Regex::new(r"\[CQ:node(?P<fields>(,\w+=[^,\]]+)*)]")?;
        for cap in re.captures_iter(&s) {
            let fields = cap.name("fields").unwrap().as_str();
            for field in fields.split(',') {
                let mut iter = field.split('=');
                let key = iter.next().unwrap();
                let value = iter.next().unwrap();
                match key {
                    "id" => id = Some(i32::from_str(value)?),
                    "name" => name = Some(value.to_string()),
                    "uin" => uin = Some(i64::from_str(value)?),
                    "content" => content = Some(Message::from_string(value.to_string())?),
                    "seq" => seq = Some(Message::from_string(value.to_string())?),
                    _ => {}
                }
            }
        }
        Ok(Self {
            id,
            name,
            uin,
            content,
            seq,
        })
    }

    fn to_json(&self) -> crate::error::Result<String> {
        let mut s = String::from("{\"type\":\"node\",\"data\":{");
        if let Some(id) = &self.id {
            s.push_str(format!("\"id\":{},", id).as_str());
        }
        if let Some(name) = &self.name {
            s.push_str(format!("\"name\":\"{}\",", name).as_str());
        }
        if let Some(uin) = &self.uin {
            s.push_str(format!("\"uin\":{},", uin).as_str());
        }
        if let Some(content) = &self.content {
            s.push_str(format!("\"content\":{},", content.to_string()).as_str());
        }
        if let Some(seq) = &self.seq {
            s.push_str(format!("\"seq\":{}", seq.to_string()).as_str());
        }
        if s.ends_with(',') {
            s.pop();
        }
        s.push_str("}}");
        Ok(s)
    }

    fn from_json(s: &str) -> crate::error::Result<Self> {
        let v: Value = serde_json::from_str(s)?;
        if let Some("node") = v["type"].as_str() {
            Ok(Self {
                id: v["data"]["id"].as_i64().and_then(|i| i32::try_from(i).ok()),
                name: v["data"]["name"].as_str().map(ToString::to_string),
                uin: v["data"]["uin"].as_i64(),
                content: v["data"]["content"]
                    .as_str()
                    .and_then(|c| Message::from_json(c).ok()),
                seq: v["data"]["seq"]
                    .as_str()
                    .and_then(|c| Message::from_json(c).ok()),
            })
        } else {
            Err(Error::new(ErrorKind::InvalidData, "type字段不为node").into())
        }
    }
}

/// [XML 消息](https://docs.go-cqhttp.org/cqcode/#xml-%E6%B6%88%E6%81%AF)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Xml {
    /// xml内容, xml中的value部分, 记得实体化处理
    pub data: Option<String>,
    /// 可能为空, 或空字符串
    pub resid: Option<i32>,
}

/// [JSON 消息](https://docs.go-cqhttp.org/cqcode/#json-%E6%B6%88%E6%81%AF)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Json {
    /// json内容, json的所有字符串记得实体化处理
    pub data: Option<String>,
    /// 默认不填为0, 走小程序通道, 填了走富文本通道发送
    pub resid: Option<i32>,
}

/// [cardimage](https://docs.go-cqhttp.org/cqcode/#cardimage)
///
/// **注意**：xml接口的消息都存在风控风险, 请自行兼容发送失败后的处理(可以失败后走普通图片模式)
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct CardImage {
    /// 和image的file字段对齐, 支持也是一样的
    pub file: Option<String>,
    /// 默认不填为400, 最小width
    pub minwidth: Option<i64>,
    /// 默认不填为400, 最小height
    pub minheight: Option<i64>,
    /// 默认不填为500, 最大width
    pub maxwidth: Option<i64>,
    /// 默认不填为1000, 最大height
    pub maxheight: Option<i64>,
    /// 分享来源的名称, 可以留空
    pub source: Option<String>,
    /// 分享来源的icon图标url, 可以留空
    pub icon: Option<String>,
}

/// [文本转语音](https://docs.go-cqhttp.org/cqcode/#%E6%96%87%E6%9C%AC%E8%BD%AC%E8%AF%AD%E9%9F%B3)
///
/// **注意**：通过腾讯的TTS接口, 采用的音源与登录账号的性别有关
#[derive(Debug, Serialize, Deserialize, CQCode)]
pub struct Tts {
    /// 内容
    pub text: Option<String>,
}

/// [文本](https://docs.go-cqhttp.org/reference/#%E6%95%B0%E7%BB%84%E6%A0%BC%E5%BC%8F%E6%B6%88%E6%81%AF)，不是CQ码，是为了方便构造消息
#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    /// 文本内容
    pub text: Option<String>,
}

impl CQCode for Text {
    fn to_string(&self) -> String {
        self.text.as_ref().unwrap().to_string()
    }

    fn from_string(s: String) -> crate::error::Result<Self> {
        Ok(Self { text: Some(s) })
    }

    fn to_json(&self) -> crate::error::Result<String> {
        Ok(format!(
            "{{\"type\":\"text\",\"data\":{{\"text\":\"{}\"}}}}",
            self.text.as_ref().unwrap()
        ))
    }

    fn from_json(s: &str) -> crate::error::Result<Self> {
        let v: Value = serde_json::from_str(s)?;
        match v["type"].as_str() {
            Some("text") => match v["data"]["text"].as_str() {
                Some(s) => Ok(Self {
                    text: Some(s.to_string()),
                }),
                None => Err(Error::new(ErrorKind::InvalidData, "没有找到text字段").into()),
            },
            Some(_) => Err(Error::new(ErrorKind::InvalidData, "type字段不为text").into()),
            None => Err(Error::new(ErrorKind::InvalidData, "没有找到type字段").into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::message::Message;
    use crate::message_from_jsons;

    #[test]
    fn test_face_to_string() {
        let t = Face { id: Some(123) };
        assert_eq!(t.to_string(), "[CQ:face,id=123]");
    }

    #[test]
    fn test_face_from_string() {
        let t = Face::from_string("[CQ:face,id=123]".to_string()).unwrap();
        assert_eq!(t.id, Some(123));
    }

    #[test]
    fn test_face_to_json() {
        let t = Face { id: Some(123) };
        assert_eq!(
            t.to_json().unwrap().as_str(),
            r#"{"type":"face","data":{"id":123}}"#
        );
    }

    #[test]
    fn test_face_from_json() {
        let t = Face::from_json(r#"{"type":"face","data":{"id":123}}"#).unwrap();
        assert_eq!(t.id, Some(123));
    }

    #[test]
    fn test_record_to_string0() {
        let t = Record {
            file: Some(r"file:///C:\\Users\Alice\Pictures\1.png".to_string()),
            magic: Some(1),
            url: Some(
                "https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png".to_string(),
            ),
            cache: Some(1),
            proxy: Some(1),
            timeout: Some(1),
        };
        assert_eq!(
            t.to_string(),
            r"[CQ:record,file=file:///C:\\Users\Alice\Pictures\1.png,magic=1,url=https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png,cache=1,proxy=1,timeout=1]"
        );
    }

    #[test]
    fn test_record_to_string1() {
        let t = Record {
            file: Some(r"file:///C:\\Users\Alice\Pictures\1.png".to_string()),
            magic: Some(1),
            url: None,
            cache: None,
            proxy: None,
            timeout: None,
        };
        assert_eq!(
            t.to_string(),
            r"[CQ:record,file=file:///C:\\Users\Alice\Pictures\1.png,magic=1]"
        );
    }

    #[test]
    fn test_record_to_string2() {
        let t = Record {
            file: None,
            magic: None,
            url: None,
            cache: None,
            proxy: None,
            timeout: None,
        };
        assert_eq!(t.to_string(), r"[CQ:record]");
    }

    #[test]
    fn test_record_from_string0() {
        let t = Record::from_string(r"[CQ:record,file=file:///C:\\Users\Alice\Pictures\1.png,magic=1,url=https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png,cache=1,proxy=1,timeout=1]".to_string()).unwrap();
        assert_eq!(
            t.file,
            Some(r"file:///C:\\Users\Alice\Pictures\1.png".to_string())
        );
        assert_eq!(t.magic, Some(1));
        assert_eq!(
            t.url,
            Some("https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png".to_string())
        );
        assert_eq!(t.cache, Some(1));
        assert_eq!(t.proxy, Some(1));
        assert_eq!(t.timeout, Some(1));
    }

    #[test]
    fn test_record_from_string1() {
        let t = Record::from_string(
            r"[CQ:record,file=file:///C:\\Users\Alice\Pictures\1.png,magic=1]".to_string(),
        )
        .unwrap();
        assert_eq!(
            t.file,
            Some(r"file:///C:\\Users\Alice\Pictures\1.png".to_string())
        );
        assert_eq!(t.magic, Some(1));
        assert_eq!(t.url, None);
        assert_eq!(t.cache, None);
        assert_eq!(t.proxy, None);
        assert_eq!(t.timeout, None);
    }

    #[test]
    fn test_record_from_string2() {
        let t = Record::from_string(r"[CQ:record]".to_string()).unwrap();
        assert_eq!(t.file, None);
        assert_eq!(t.magic, None);
        assert_eq!(t.url, None);
        assert_eq!(t.cache, None);
        assert_eq!(t.proxy, None);
        assert_eq!(t.timeout, None);
    }

    #[test]
    fn test_record_to_json0() {
        let t = Record {
            file: Some(r"file:///C:\Users\User\Pictures\1.png".to_string()),
            magic: Some(1),
            url: Some("https://www.baidu.com/img/1.png".to_string()),
            cache: Some(1),
            proxy: Some(1),
            timeout: Some(1),
        };
        assert_eq!(
            t.to_json().unwrap().as_str(),
            r#"{"type":"record","data":{"file":"file:///C:\\Users\\User\\Pictures\\1.png","magic":1,"url":"https://www.baidu.com/img/1.png","cache":1,"proxy":1,"timeout":1}}"#
        );
    }

    #[test]
    fn test_record_to_json1() {
        let t = Record {
            file: Some(r"file:///C:\Users\User\Pictures\1.png".to_string()),
            magic: Some(1),
            url: None,
            cache: None,
            proxy: None,
            timeout: None,
        };
        assert_eq!(
            t.to_json().unwrap().as_str(),
            r#"{"type":"record","data":{"file":"file:///C:\\Users\\User\\Pictures\\1.png","magic":1}}"#
        );
    }

    #[test]
    fn test_record_to_json2() {
        let t = Record {
            file: None,
            magic: None,
            url: None,
            cache: None,
            proxy: None,
            timeout: None,
        };
        assert_eq!(
            t.to_json().unwrap().as_str(),
            r#"{"type":"record","data":{}}"#
        );
    }

    #[test]
    fn test_record_from_json0() {
        let t = Record::from_json(r#"{"type":"record","data":{"file":"file:///C:\\Users\\User\\Pictures\\1.png","magic":1,"url":"https://www.baidu.com/img/1.png","cache":1,"proxy":1,"timeout":1}}"#).unwrap();
        assert_eq!(
            t.file,
            Some(r"file:///C:\Users\User\Pictures\1.png".to_string())
        );
        assert_eq!(t.magic, Some(1));
        assert_eq!(t.url, Some("https://www.baidu.com/img/1.png".to_string()));
        assert_eq!(t.cache, Some(1));
        assert_eq!(t.proxy, Some(1));
        assert_eq!(t.timeout, Some(1));
    }

    #[test]
    fn test_record_from_json1() {
        let t = Record::from_json(r#"{"type":"record","data":{"file":"file:///C:\\Users\\User\\Pictures\\1.png","magic":1}}"#).unwrap();
        assert_eq!(
            t.file,
            Some(r"file:///C:\Users\User\Pictures\1.png".to_string())
        );
        assert_eq!(t.magic, Some(1));
        assert_eq!(t.url, None);
        assert_eq!(t.cache, None);
        assert_eq!(t.proxy, None);
        assert_eq!(t.timeout, None);
    }

    #[test]
    fn test_record_from_json2() {
        let t = Record::from_json(r#"{"type":"record","data":{}}"#).unwrap();
        assert_eq!(t.file, None);
        assert_eq!(t.magic, None);
        assert_eq!(t.url, None);
        assert_eq!(t.cache, None);
        assert_eq!(t.proxy, None);
        assert_eq!(t.timeout, None);
    }

    #[test]
    fn test_node_to_json0() {
        let t = Node {
            id: Some(123),
            name: None,
            uin: None,
            content: None,
            seq: None,
        };
        assert_eq!(
            t.to_json().unwrap().as_str(),
            r#"{"type":"node","data":{"id":123}}"#
        );
    }

    #[test]
    fn test_node_to_json1() {
        let t = Node {
            id: None,
            name: Some("hello".to_string()),
            uin: Some(456),
            content: Some(message_from_jsons!(Face { id: Some(123) })),
            seq: Some(message_from_jsons!(Face { id: Some(456) })),
        };
        assert_eq!(
            t.to_json().unwrap().as_str(),
            r#"{"type":"node","data":{"name":"hello","uin":456,"content":[{"type":"face","data":{"id":123}}],"seq":[{"type":"face","data":{"id":456}}]}}"#
        );
    }

    #[test]
    fn test_node_to_json2() {
        let t = Node {
            id: None,
            name: Some("hello".to_string()),
            uin: Some(456),
            content: Some(message_from_jsons!(
                Face { id: Some(123) },
                Text {
                    text: Some("world".to_string())
                },
                Face { id: Some(456) }
            )),
            seq: None,
        };
        assert_eq!(
            t.to_json().unwrap().as_str(),
            r#"{"type":"node","data":{"name":"hello","uin":456,"content":[{"type":"face","data":{"id":123}},{"type":"text","data":{"text":"world"}},{"type":"face","data":{"id":456}}]}}"#
        );
    }

    #[test]
    fn test_node_from_json0() {
        let t = Node::from_json(r#"{"type":"node","data":{"id":123}}"#).unwrap();
        assert_eq!(t.id, Some(123));
        assert_eq!(t.name, None);
        assert_eq!(t.uin, None);
        assert_eq!(t.content, None);
        assert_eq!(t.seq, None);
    }
}
