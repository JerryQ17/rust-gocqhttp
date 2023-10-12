use crate::message::{Message, MessageType};
use serde::{Deserialize, Deserializer, Serialize};

/// `get_login_info`API的响应数据结构
#[derive(Deserialize)]
pub struct LoginInfo {
    #[serde(default)]
    /// QQ号
    pub user_id: i64,
    #[serde(default)]
    /// QQ昵称
    pub nickname: String,
}

/// `ModelShowVariants.variants`字段的元素类型
#[derive(Deserialize)]
pub struct ModelShowVariant {
    #[serde(default)]
    /// 在线机型名
    pub model_show: String,
    #[serde(default)]
    /// 是否需要会员
    pub need_pay: bool,
}

/// `get_model_show`API的响应数据结构
#[derive(Deserialize)]
pub struct ModelShowVariants {
    #[serde(default)]
    /// 在线机型列表
    pub variants: Vec<ModelShowVariant>,
}

/// `ClientDevices.clients`字段的元素类型
#[derive(Deserialize)]
pub struct ClientDevice {
    #[serde(default)]
    /// 客户端ID
    pub app_id: i64,
    #[serde(default)]
    /// 设备名称
    pub device_name: String,
    #[serde(default)]
    /// 设备类型
    pub device_kind: String,
}

/// `get_online_clients`API的响应数据结构
#[derive(Deserialize)]
pub struct ClientDevices {
    #[serde(default)]
    /// 在线客户端列表
    pub clients: Vec<ClientDevice>,
}

/// `StrangerInfo.sex`字段的类型
pub enum Sex {
    /// 男性
    Male,
    /// 女性
    Female,
    /// 未知
    Unknown,
}

impl<T: AsRef<str>> From<T> for Sex {
    fn from(value: T) -> Self {
        match value.as_ref() {
            "male" => Sex::Male,
            "female" => Sex::Female,
            _ => Sex::Unknown,
        }
    }
}

impl ToString for Sex {
    fn to_string(&self) -> String {
        match self {
            Sex::Male => "male".to_string(),
            Sex::Female => "female".to_string(),
            Sex::Unknown => "unknown".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for Sex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// `get_stranger_info`API的响应数据结构
#[derive(Deserialize)]
pub struct StrangerInfo {
    #[serde(default)]
    /// QQ号
    pub user_id: i64,
    #[serde(default)]
    /// 昵称
    pub nickname: String,
    /// 性别, male, female或unknown
    pub sex: Sex,
    #[serde(default)]
    /// 年龄
    pub age: i32,
    #[serde(default)]
    /// qid ID身份卡
    pub qid: String,
    #[serde(default)]
    /// 等级
    pub level: i32,
    #[serde(default)]
    /// 等级
    pub login_days: i32,
}

/// `get_friend_list`API的响应数据结构
#[derive(Deserialize)]
pub struct Friend {
    #[serde(default)]
    /// QQ号
    pub user_id: i64,
    #[serde(default)]
    /// 昵称
    pub nickname: String,
    #[serde(default)]
    /// 备注名
    pub remark: String,
}

/// `get_unidirectional_friend_list`API的响应数据结构
#[derive(Deserialize)]
pub struct UnidirectionalFriend {
    #[serde(default)]
    /// QQ号
    pub user_id: i64,
    #[serde(default)]
    /// 昵称
    pub nickname: String,
    #[serde(default)]
    /// 来源
    pub source: String,
}

/// `send_private_msg`, `send_group_msg`API的响应数据结构
#[derive(Deserialize)]
pub struct MessageID {
    #[serde(default)]
    /// 消息ID
    pub message_id: i32,
}

/// `Msg.sender`字段的类型
#[derive(Deserialize)]
pub struct Sender {
    #[serde(default)]
    /// 发送者昵称
    pub nickname: String,
    #[serde(default)]
    /// 发送者QQ号
    pub user_id: i64,
}

/// `get_msg`API的响应数据结构
#[derive(Deserialize)]
pub struct Msg {
    #[serde(default)]
    /// 是否是群消息
    pub group: bool,
    #[serde(default)]
    /// 发送者群号
    pub group_id: i64,
    #[serde(default)]
    /// 消息id
    pub message_id: i32,
    #[serde(default)]
    /// 消息真实id
    pub real_id: i32,
    /// 消息类型
    pub message_type: MessageType,
    /// 发送者
    pub sender: Sender,
    /// 发送时间
    pub time: i32,
    #[serde(default)]
    /// 消息内容
    pub message: String,
    #[serde(default)]
    /// 消息内容
    pub raw_message: String,
}

/// `get_forward_msg`API的响应数据结构
#[derive(Deserialize)]
pub struct ForwardMessage {
    /// 消息内容
    pub content: Message,
    /// 发送者
    pub sender: Sender,
    #[serde(default)]
    /// 发送时间
    pub time: i32,
}

/// API的响应数据结构
#[derive(Deserialize)]
pub struct ForwardMessageID {
    #[serde(default)]
    /// 消息ID
    pub message_id: i32,
    #[serde(default)]
    /// 消息真实ID
    pub real_id: i32,
}

/// API的响应数据结构
#[derive(Deserialize)]
pub struct Image {
    #[serde(default)]
    /// 图片源文件大小
    pub size: i32,
    #[serde(default)]
    /// 图片文件原名
    pub filename: String,
    #[serde(default)]
    /// 图片下载地址
    pub url: String,
}

/// API的响应数据结构
#[derive(Deserialize)]
pub struct CanSend {
    #[serde(default)]
    /// 是或否
    pub yes: bool,
}

/// API的响应数据结构
#[derive(Deserialize)]
pub struct TextDetection {
    #[serde(default)]
    /// 文本
    pub text: String,
    #[serde(default)]
    /// 置信度
    pub confidence: i32,
    #[serde(default)]
    /// 坐标
    pub coordinates: [i32; 2],
}

/// `ocr_image`API的响应数据结构
#[derive(Deserialize)]
pub struct OCRImage {
    #[serde(default)]
    /// OCR结果
    pub texts: Vec<TextDetection>,
    #[serde(default)]
    /// 语言
    pub language: String,
}

/// `get_record`API的响应数据结构
#[derive(Deserialize)]
pub struct Record {
    #[serde(default)]
    /// 转换后的语音文件路径, 如`/home/somebody/cqhttp/data/record/0B38145AA44505000B38145AA4450500.mp3`
    pub file: String,
}

/// `get_group_info`, `get_group_list`API的响应数据结构
///
/// 如果机器人尚未加入群, `group_create_time`, `group_level`, `max_member_count`和`member_count`将会为0
#[derive(Deserialize)]
pub struct GroupInfo {
    #[serde(default)]
    /// 群号
    pub group_id: i64,
    #[serde(default)]
    /// 群名称
    pub group_name: String,
    #[serde(default)]
    /// 群备注
    pub group_memo: String,
    #[serde(default)]
    /// 群创建时间
    pub group_create_time: u32,
    #[serde(default)]
    /// 群等级
    pub group_level: u32,
    #[serde(default)]
    /// 成员数
    pub member_count: i32,
    #[serde(default)]
    /// 最大成员数（群容量）
    pub max_member_count: i32,
}

/// 群角色
pub enum GroupRole {
    /// 群主
    Owner,
    /// 群管理员
    Admin,
    /// 群成员
    Member,
}

impl<T: AsRef<str>> From<T> for GroupRole {
    /// # Panic
    /// 输入非"owner", "admin", "member"的值会导致panic
    fn from(value: T) -> Self {
        match value.as_ref() {
            "owner" => GroupRole::Owner,
            "admin" => GroupRole::Admin,
            "member" => GroupRole::Member,
            err => panic!("Can't convert {} to GroupRole!", err),
        }
    }
}

impl ToString for GroupRole {
    fn to_string(&self) -> String {
        match self {
            GroupRole::Owner => "owner".to_string(),
            GroupRole::Admin => "admin".to_string(),
            GroupRole::Member => "member".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for GroupRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Deserialize)]
pub struct GroupMemberInfo {
    #[serde(default)]
    /// 群号
    pub group_id: i64,
    #[serde(default)]
    /// QQ 号
    pub user_id: i64,
    #[serde(default)]
    /// 昵称
    pub nickname: String,
    #[serde(default)]
    /// 群名片／备注
    pub card: String,
    /// 性别, male 或 female 或 unknown
    pub sex: Sex,
    #[serde(default)]
    /// 年龄
    pub age: i32,
    #[serde(default)]
    /// 地区
    pub area: String,
    #[serde(default)]
    /// 加群时间戳
    pub join_time: i32,
    #[serde(default)]
    /// 最后发言时间戳
    pub last_sent_time: i32,
    #[serde(default)]
    /// 成员等级
    pub level: String,
    /// 角色, owner 或 admin 或 member
    pub role: GroupRole,
    /// 是否不良记录成员
    pub unfriendly: bool,
    #[serde(default)]
    /// 专属头衔
    pub title: String,
    #[serde(default)]
    /// 专属头衔过期时间戳
    pub title_expire_time: i64,
    #[serde(default)]
    /// 是否允许修改群名片
    pub card_changeable: bool,
    #[serde(default)]
    /// 禁言到期时间
    pub shut_up_timestamp: i64,
}

pub enum GroupHonorType {
    Talkative,
    Performer,
    Legend,
    StrongNewbie,
    Emotion,
    All,
}

impl ToString for GroupHonorType {
    fn to_string(&self) -> String {
        match self {
            GroupHonorType::Talkative => "talkative".to_string(),
            GroupHonorType::Performer => "performer".to_string(),
            GroupHonorType::Legend => "legend".to_string(),
            GroupHonorType::StrongNewbie => "strong_newbie".to_string(),
            GroupHonorType::Emotion => "emotion".to_string(),
            GroupHonorType::All => "all".to_string(),
        }
    }
}

impl<T: AsRef<str>> From<T> for GroupHonorType {
    fn from(value: T) -> Self {
        match value.as_ref() {
            "talkative" => GroupHonorType::Talkative,
            "performer" => GroupHonorType::Performer,
            "legend" => GroupHonorType::Legend,
            "strong_newbie" => GroupHonorType::StrongNewbie,
            "emotion" => GroupHonorType::Emotion,
            _ => GroupHonorType::All,
        }
    }
}

#[derive(Deserialize)]
pub struct CurrentTalkativeWinner {
    #[serde(default)]
    /// QQ号
    pub user_id: i64,
    #[serde(default)]
    /// 昵称
    pub nickname: String,
    #[serde(default)]
    /// 头像URL
    pub avatar: String,
    #[serde(default)]
    /// 持续天数
    pub day_count: i32,
}

#[derive(Deserialize)]
pub struct GroupHonorWinner {
    #[serde(default)]
    /// QQ号
    pub user_id: i64,
    #[serde(default)]
    /// 昵称
    pub nickname: String,
    #[serde(default)]
    /// 头像URL
    pub avatar: String,
    #[serde(default)]
    /// 荣誉描述
    pub description: String,
}

#[derive(Deserialize)]
pub struct GroupHonorInfo {
    #[serde(default)]
    /// 群号
    pub group_id: i64,
    /// 当前龙王, 仅 type 为 talkative 或 all 时有数据
    pub current_talkative: CurrentTalkativeWinner,
    #[serde(default)]
    /// 历史龙王, 仅 type 为 talkative 或 all 时有数据
    pub talkative_list: Vec<GroupHonorWinner>,
    #[serde(default)]
    /// 群聊之火, 仅 type 为 performer 或 all 时有数据
    pub performer_list: Vec<GroupHonorWinner>,
    #[serde(default)]
    /// 群聊炽焰, 仅 type 为 legend 或 all 时有数据
    pub legend_list: Vec<GroupHonorWinner>,
    #[serde(default)]
    /// 冒尖小春笋, 仅 type 为 strong_newbie 或 all 时有数据
    pub strong_newbie_list: Vec<GroupHonorWinner>,
    #[serde(default)]
    /// 快乐之源, 仅 type 为 emotion 或 all 时有数据
    pub emotion_list: Vec<GroupHonorWinner>,
}

#[derive(Deserialize)]
pub struct InvitedRequest {
    #[serde(default)]
    /// 请求ID
    pub request_id: i64,
    #[serde(default)]
    /// 邀请者
    pub invitor_uin: i64,
    #[serde(default)]
    /// 邀请者昵称
    pub invitor_nick: String,
    #[serde(default)]
    /// 群号
    pub group_id: i64,
    #[serde(default)]
    /// 群名
    pub group_name: String,
    #[serde(default)]
    /// 是否已被处理
    pub checked: bool,
    #[serde(default)]
    /// 处理者, 未处理为0
    pub actor: i64,
}

#[derive(Deserialize)]
pub struct JoinRequest {
    #[serde(default)]
    /// 请求ID
    pub request_id: i64,
    #[serde(default)]
    /// 请求者ID
    pub requester_uin: i64,
    #[serde(default)]
    /// 请求者昵称
    pub requester_nick: String,
    #[serde(default)]
    /// 验证消息
    pub message: String,
    #[serde(default)]
    /// 群号
    pub group_id: i64,
    #[serde(default)]
    /// 群名
    pub group_name: String,
    #[serde(default)]
    /// 是否已被处理
    pub checked: bool,
    #[serde(default)]
    /// 处理者, 未处理为0
    pub actor: i64,
}

#[derive(Deserialize)]
pub struct GroupSystemMsg {
    #[serde(default)]
    /// 邀请消息列表
    pub invited_requests: Vec<InvitedRequest>,
    #[serde(default)]
    /// 进群消息列表
    pub join_requests: Vec<JoinRequest>,
}

/// `get_essence_msg_list`API的响应数据结构
#[derive(Deserialize)]
pub struct EssenceMsg {
    #[serde(default)]
    /// 发送者QQ号
    pub sender_id: i64,
    #[serde(default)]
    /// 发送者昵称
    pub sender_nick: String,
    #[serde(default)]
    /// 消息发送时间
    pub sender_time: i64,
    #[serde(default)]
    /// 操作者QQ号
    pub operator_id: i64,
    #[serde(default)]
    /// 操作者昵称
    pub operator_nick: String,
    #[serde(default)]
    /// 精华设置时间
    pub operator_time: i64,
    #[serde(default)]
    /// 消息ID
    pub message_id: i32,
}

/// `get_group_at_all_remain`API的响应数据结构
#[derive(Deserialize)]
pub struct GroupAtAllRemain {
    #[serde(default)]
    /// 是否可以 @全体成员
    pub can_at_all: bool,
    #[serde(default)]
    /// 群内所有管理当天剩余 @全体成员 次数
    pub remain_at_all_count_for_group: i16,
    #[serde(default)]
    /// Bot当天剩余 @全体成员 次数
    pub remain_at_all_count_for_uin: i16,
}

/// [群消息(anonymous字段)](https://docs.go-cqhttp.org/event/#%E7%BE%A4%E6%B6%88%E6%81%AF)，`set_group_anonymous_ban`API的其中一个可选参数
#[derive(Serialize)]
pub struct AnonymousGroupMsg {
    /// 匿名用户 ID
    id: i64,
    /// 匿名用户名称
    name: String,
    /// 匿名用户flag, 在调用禁言API时需要传入
    flag: String,
}

#[derive(Deserialize)]
pub struct GroupNoticeImage {
    #[serde(default)]
    /// 图片高度
    pub height: String,
    #[serde(default)]
    /// 图片宽度
    pub width: String,
    #[serde(default)]
    /// 图片ID
    pub id: String,
}

#[derive(Deserialize)]
pub struct GroupNoticeMessage {
    #[serde(default)]
    /// 公告内容
    pub text: String,
    #[serde(default)]
    /// 公告图片
    pub images: Vec<GroupNoticeImage>,
}

/// `get_group_notice`API的响应数据结构
#[derive(Deserialize)]
pub struct GroupNotice {
    #[serde(default)]
    /// 公告发表者
    pub sender_id: i64,
    #[serde(default)]
    /// 公告发表时间
    pub publish_time: i64,
    /// 公告内容
    pub message: GroupNoticeMessage,
}

/// `get_group_file_system_info`API的响应数据结构
#[derive(Deserialize)]
pub struct GroupFileSystemInfo {
    #[serde(default)]
    /// 文件总数
    pub file_count: i32,
    #[serde(default)]
    /// 文件上限
    pub limit_count: i32,
    #[serde(default)]
    /// 已使用空间
    pub used_space: i64,
    #[serde(default)]
    /// 空间上限
    pub total_space: i64,
}

#[derive(Deserialize)]
pub struct File {
    #[serde(default)]
    /// 群号
    pub group_id: i64,
    #[serde(default)]
    /// 文件ID
    pub file_id: String,
    #[serde(default)]
    /// 文件名
    pub file_name: String,
    #[serde(default)]
    /// 文件类型
    pub busid: i32,
    #[serde(default)]
    /// 文件大小
    pub file_size: i64,
    #[serde(default)]
    /// 上传时间
    pub upload_time: i64,
    #[serde(default)]
    /// 过期时间,永久文件恒为0
    pub dead_time: i64,
    #[serde(default)]
    /// 最后修改时间
    pub modify_time: i64,
    #[serde(default)]
    /// 下载次数
    pub download_times: i32,
    #[serde(default)]
    /// 上传者ID
    pub uploader: i64,
    #[serde(default)]
    /// 上传者名字
    pub uploader_name: String,
}

#[derive(Deserialize)]
pub struct Folder {
    #[serde(default)]
    /// 群号
    pub group_id: i64,
    #[serde(default)]
    /// 文件夹ID
    pub folder_id: String,
    #[serde(default)]
    /// 文件名
    pub folder_name: String,
    #[serde(default)]
    /// 创建时间
    pub create_time: i64,
    #[serde(default)]
    /// 创建者
    pub creator: i64,
    #[serde(default)]
    /// 创建者名字
    pub creator_name: String,
    #[serde(default)]
    /// 子文件数量
    pub total_file_count: i32,
}

/// `get_group_root_files`API的响应数据结构
#[derive(Deserialize)]
pub struct GroupFiles {
    #[serde(default)]
    /// 文件列表
    pub files: Vec<File>,
    #[serde(default)]
    /// 文件夹列表
    pub folders: Vec<Folder>,
}

/// `get_group_file_url`API的响应数据结构
#[derive(Deserialize)]
pub struct GroupFileUrl {
    #[serde(default)]
    /// 文件下载链接
    pub url: String,
}

/// `get_cookies`API的响应数据结构
#[derive(Deserialize)]
pub struct Cookies {
    #[serde(default)]
    /// Cookies
    pub cookies: String,
}

/// `get_csrf_token`API的响应数据结构
#[derive(Deserialize)]
pub struct CSRFToken {
    #[serde(default)]
    /// CSRF Token
    pub token: i32,
}

/// `get_credentials`API的响应数据结构
#[derive(Deserialize)]
pub struct Credentials {
    #[serde(default)]
    /// Cookies
    pub cookies: String,
    #[serde(default)]
    /// CSRF Token
    pub csrf_token: i32,
}

/// `get_version_info`API的响应数据结构
#[derive(Deserialize)]
pub struct VersionInfo {
    #[serde(default)]
    /// 应用标识, 固定值go-cqhttp
    pub app_name: String,
    #[serde(default)]
    /// 应用版本, 如 v0.9.40-fix4
    pub app_version: String,
    #[serde(default)]
    /// 应用完整名称
    pub app_full_name: String,
    #[serde(default)]
    /// 应用协议名称
    pub protocol_name: i32,
    #[serde(default)]
    /// OneBot标准版本
    pub protocol_version: String,
    #[serde(default)]
    /// 原Coolq版本 固定值pro
    pub coolq_edition: String,
    #[serde(default)]
    /// Coolq目录
    pub coolq_directory: String,
    #[serde(default)]
    ///是否为go-cqhttp 固定值true
    pub go_cqhttp: bool,
    #[serde(default)]
    /// 固定值4.15.0
    pub plugin_version: String,
    #[serde(default)]
    /// 固定值99
    pub plugin_build_number: i32,
    #[serde(default)]
    /// 固定值release
    pub plugin_build_configuration: String,
    #[serde(default)]
    /// 运行时版本
    pub runtime_version: String,
    #[serde(default)]
    /// 运行时操作系统
    pub runtime_os: String,
    #[serde(default)]
    /// 应用版本, 如 v0.9.40-fix4
    pub version: String,
}

/// `Status.stat`字段的类型
#[derive(Deserialize)]
pub struct Statistics {
    #[serde(default)]
    /// 收到的数据包总数
    pub packet_received: u64,
    #[serde(default)]
    /// 发送的数据包总数
    pub packet_sent: u64,
    #[serde(default)]
    /// 数据包丢失总数
    pub packet_lost: u32,
    #[serde(default)]
    /// 接受信息总数
    pub message_received: u64,
    #[serde(default)]
    ///发送信息总数
    pub message_sent: u64,
    #[serde(default)]
    /// TCP链接断开次数
    pub disconnect_times: u32,
    #[serde(default)]
    /// 账号掉线次数
    pub lost_times: u32,
    #[serde(default)]
    /// 最后一条消息时间
    pub last_message_time: i64,
}

/// `get_status`API的响应数据结构
#[derive(Deserialize)]
pub struct Status {
    #[serde(default)]
    /// 原CQHTTP字段, 恒定为true
    pub app_initialized: bool,
    #[serde(default)]
    /// 原CQHTTP字段, 恒定为true
    pub app_enabled: bool,
    #[serde(default)]
    /// 原CQHTTP字段, 恒定为true
    pub plugins_good: bool,
    #[serde(default)]
    /// 原CQHTTP字段, 恒定为true
    pub app_good: bool,
    #[serde(default)]
    /// 表示BOT是否在线
    pub online: bool,
    #[serde(default)]
    /// 表示BOT是否在线
    pub good: bool,
    /// 运行统计
    pub stat: Statistics,
}

/// `download_file`API的响应数据结构
#[derive(Deserialize)]
pub struct DownloadedFile {
    #[serde(default)]
    /// 下载文件的绝对路径
    pub file: String,
}

/// `UrlSafety.level`字段的类型，链接安全等级
pub enum UrlSafetyLevel {
    /// 安全
    Safe,
    /// 未知
    Unknown,
    /// 危险
    Dangerous,
}

impl From<i64> for UrlSafetyLevel {
    fn from(value: i64) -> Self {
        match value {
            1 => UrlSafetyLevel::Safe,
            2 => UrlSafetyLevel::Unknown,
            _ => UrlSafetyLevel::Dangerous,
        }
    }
}

impl<'de> Deserialize<'de> for UrlSafetyLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(i64::deserialize(deserializer)?.into())
    }
}

/// `check_url_safely`API的响应数据结构
#[derive(Deserialize)]
pub struct UrlSafety {
    /// 安全等级
    pub level: UrlSafetyLevel,
}

/// `get_word_slices`API的响应数据结构
#[derive(Deserialize)]
pub struct WordSlices {
    #[serde(default)]
    /// 分词结果
    pub slices: Vec<String>,
}
