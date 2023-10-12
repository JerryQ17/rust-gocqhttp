pub mod data;

use crate::message::cq_code::code::Node;
use crate::message::{Message, MessageType};
use crate::Result;
use async_trait::async_trait;
use data::*;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::io::{Error, ErrorKind};

/// [GoCqhttp API](https://docs.go-cqhttp.org/api/#api)
#[async_trait]
pub trait GoCqhttpAPI {
    /// [获取登录号信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%99%BB%E5%BD%95%E5%8F%B7%E4%BF%A1%E6%81%AF)
    async fn get_login_info(&self) -> Result<LoginInfo>;

    /// [设置登录号资料](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%99%BB%E5%BD%95%E5%8F%B7%E8%B5%84%E6%96%99)
    async fn set_qq_profile(
        &self,
        nickname: String,
        company: String,
        email: String,
        college: String,
        personal_note: String,
    ) -> Result<()>;

    /// [获取企点账号信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E4%BC%81%E7%82%B9%E8%B4%A6%E5%8F%B7%E4%BF%A1%E6%81%AF)
    async fn qidian_get_account_info(&self) -> Result<String>;

    /// [获取在线机型](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%9C%A8%E7%BA%BF%E6%9C%BA%E5%9E%8B)
    async fn get_model_show(&self) -> Result<ModelShowVariants>;

    /// [设置在线机型](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E5%9C%A8%E7%BA%BF%E6%9C%BA%E5%9E%8B)
    async fn set_model_show(&self, model: String, model_show: String) -> Result<()>;

    /// [获取当前账号在线客户端列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%BD%93%E5%89%8D%E8%B4%A6%E5%8F%B7%E5%9C%A8%E7%BA%BF%E5%AE%A2%E6%88%B7%E7%AB%AF%E5%88%97%E8%A1%A8)
    async fn get_online_clients(&self, no_cache: bool) -> Result<ClientDevices>;

    /// [获取陌生人信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E9%99%8C%E7%94%9F%E4%BA%BA%E4%BF%A1%E6%81%AF)
    async fn get_stranger_info(&self, user_id: i64, no_cache: bool) -> Result<StrangerInfo>;

    /// [获取好友列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%A5%BD%E5%8F%8B%E5%88%97%E8%A1%A8)
    async fn get_friend_list(&self) -> Result<Vec<Friend>>;

    /// [获取单向好友列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%8D%95%E5%90%91%E5%A5%BD%E5%8F%8B%E5%88%97%E8%A1%A8)
    async fn get_unidirectional_friend_list(&self) -> Result<Vec<UnidirectionalFriend>>;

    /// [删除好友](https://docs.go-cqhttp.org/api/#%E5%88%A0%E9%99%A4%E5%A5%BD%E5%8F%8B)
    async fn delete_friend(&self, user_id: i64) -> Result<()>;

    /// [删除单向好友](https://docs.go-cqhttp.org/api/#%E5%88%A0%E9%99%A4%E5%8D%95%E5%90%91%E5%A5%BD%E5%8F%8B)
    async fn delete_unidirectional_friend(&self, user_id: i64) -> Result<()>;

    /// [发送私聊消息](https://docs.go-cqhttp.org/api/#%E5%8F%91%E9%80%81%E7%A7%81%E8%81%8A%E6%B6%88%E6%81%AF)
    async fn send_private_msg(
        &self,
        user_id: i64,
        group_id: i64,
        message: Message,
        auto_escape: bool,
    ) -> Result<MessageID>;

    /// [发送群聊消息](https://docs.go-cqhttp.org/api/#%E5%8F%91%E9%80%81%E7%BE%A4%E8%81%8A%E6%B6%88%E6%81%AF)
    async fn send_group_msg(
        &self,
        group_id: i64,
        message: Message,
        auto_escape: bool,
    ) -> Result<MessageID>;

    /// [发送消息](https://docs.go-cqhttp.org/api/#%E5%8F%91%E9%80%81%E6%B6%88%E6%81%AF)
    async fn send_msg(
        &self,
        message_type: MessageType,
        user_id: i64,
        group_id: i64,
        message: Message,
        auto_escape: bool,
    ) -> Result<MessageID>;

    /// [获取消息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E6%B6%88%E6%81%AF)
    ///
    /// **注意**：该API要求go-cqhttp版本在v0.9.37及以上，不满足版本要求将导致反序列化出错
    ///
    /// 提示：运行时不会检查版本号
    async fn get_msg(&self, message_id: i32) -> Result<Msg>;

    /// [撤回消息](https://docs.go-cqhttp.org/api/#%E6%92%A4%E5%9B%9E%E6%B6%88%E6%81%AF)
    async fn delete_msg(&self, message_id: i32) -> Result<()>;

    /// [标记消息已读](https://docs.go-cqhttp.org/api/#%E6%A0%87%E8%AE%B0%E6%B6%88%E6%81%AF%E5%B7%B2%E8%AF%BB)
    async fn mark_msg_as_read(&self, message_id: i32) -> Result<()>;

    /// [获取合并转发内容](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91%E5%86%85%E5%AE%B9)
    async fn get_forward_msg(&self, message_id: String) -> Result<Vec<ForwardMessage>>;

    /// [发送合并转发(群聊)](https://docs.go-cqhttp.org/api/#%E5%8F%91%E9%80%81%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91-%E7%BE%A4%E8%81%8A)
    async fn send_group_forward_msg(
        &self,
        group_id: i64,
        messages: Node,
    ) -> Result<ForwardMessageID>;

    /// [发送合并转发(好友)](https://docs.go-cqhttp.org/api/#%E5%8F%91%E9%80%81%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91-%E5%A5%BD%E5%8F%8B)
    async fn send_private_forward_msg(&self) -> Result<ForwardMessageID>;

    /// [获取群消息历史记录](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%B6%88%E6%81%AF%E5%8E%86%E5%8F%B2%E8%AE%B0%E5%BD%95)
    async fn get_group_msg_history(&self, message_seq: i64, group_id: i64) -> Result<Vec<Message>>;

    /// [获取图片信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%9B%BE%E7%89%87%E4%BF%A1%E6%81%AF)
    async fn get_image(&self, file: String) -> Result<Image>;

    /// [检查是否可以发送图片](https://docs.go-cqhttp.org/api/#%E6%A3%80%E6%9F%A5%E6%98%AF%E5%90%A6%E5%8F%AF%E4%BB%A5%E5%8F%91%E9%80%81%E5%9B%BE%E7%89%87)
    async fn can_send_image(&self) -> Result<CanSend>;

    /// [图片OCR](https://docs.go-cqhttp.org/api/#%E5%9B%BE%E7%89%87-ocr)
    async fn ocr_image(&self, image: String) -> Result<OCRImage>;

    /// [获取语音](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E8%AF%AD%E9%9F%B3)
    async fn get_record(&self, file: String, out_format: String) -> Result<Record>;

    /// [检查是否可以发送语音](https://docs.go-cqhttp.org/api/#%E6%A3%80%E6%9F%A5%E6%98%AF%E5%90%A6%E5%8F%AF%E4%BB%A5%E5%8F%91%E9%80%81%E8%AF%AD%E9%9F%B3)
    async fn can_send_record(&self) -> Result<CanSend>;

    /// [处理加好友请求](https://docs.go-cqhttp.org/api/#%E5%A4%84%E7%90%86%E5%8A%A0%E5%A5%BD%E5%8F%8B%E8%AF%B7%E6%B1%82)
    async fn set_friend_add_request(
        &self,
        flag: String,
        approve: bool,
        remark: String,
    ) -> Result<()>;

    /// [处理加群请求／邀请](https://docs.go-cqhttp.org/api/#%E5%A4%84%E7%90%86%E5%8A%A0%E7%BE%A4%E8%AF%B7%E6%B1%82-%E9%82%80%E8%AF%B7)
    async fn set_group_add_request(
        &self,
        flag: String,
        sub_type: String,
        approve: bool,
        reason: String,
    ) -> Result<()>;

    /// [获取群信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E4%BF%A1%E6%81%AF)
    ///
    /// **注意**：在 go-cqhttp-v0.9.40之前的版本中，该API不能获取陌生群消息
    async fn get_group_info(&self, group_id: i64, no_cache: bool) -> Result<GroupInfo>;

    /// [获取群列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E5%88%97%E8%A1%A8)
    async fn get_group_list(&self, no_cache: bool) -> Result<Vec<GroupInfo>>;

    /// [获取群成员信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%88%90%E5%91%98%E4%BF%A1%E6%81%AF)
    async fn get_group_member_info(
        &self,
        group_id: i64,
        user_id: i64,
        no_cache: bool,
    ) -> Result<GroupMemberInfo>;

    /// [获取群成员列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%88%90%E5%91%98%E5%88%97%E8%A1%A8)
    async fn get_group_member_list(
        &self,
        group_id: i64,
        no_cache: bool,
    ) -> Result<Vec<GroupMemberInfo>>;

    /// [获取群荣誉信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E8%8D%A3%E8%AA%89%E4%BF%A1%E6%81%AF)
    async fn get_group_honor_info(
        &self,
        group_id: i64,
        honor_type: GroupHonorType,
    ) -> Result<GroupHonorInfo>;

    /// [获取群系统消息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E7%B3%BB%E7%BB%9F%E6%B6%88%E6%81%AF)
    ///
    /// **注意**：在 go-cqhttp-v0.9.40 之前的版本中，无法获取被过滤的群系统消息
    async fn get_group_system_msg(&self, group_id: i64) -> Result<GroupSystemMsg>;

    /// [获取精华消息列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%B2%BE%E5%8D%8E%E6%B6%88%E6%81%AF%E5%88%97%E8%A1%A8)
    async fn get_essence_msg_list(&self, group_id: i64) -> Result<Vec<EssenceMsg>>;

    /// [获取群 @全体成员 剩余次数](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4-%E5%85%A8%E4%BD%93%E6%88%90%E5%91%98-%E5%89%A9%E4%BD%99%E6%AC%A1%E6%95%B0)
    async fn get_group_at_all_remain(&self, group_id: i64) -> Result<GroupAtAllRemain>;

    /// [设置群名](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%BE%A4%E5%90%8D)
    async fn set_group_name(&self, group_id: i64, group_name: String) -> Result<()>;

    /// [设置群头像](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%BE%A4%E5%A4%B4%E5%83%8F)
    ///
    /// 提示：`file`参数支持本地文件路径、URL、Base64编码的图片
    ///
    /// 提示：目前这个API在登录一段时间后因cookie失效而失效, 请考虑后使用
    async fn set_group_portrait(&self, group_id: i64, file: String, cache: bool) -> Result<()>;

    /// [设置群管理员](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%BE%A4%E7%AE%A1%E7%90%86%E5%91%98)
    async fn set_group_admin(&self, group_id: i64, user_id: i64, enable: bool) -> Result<()>;

    /// [设置群名片(群备注)](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%BE%A4%E5%90%8D%E7%89%87-%E7%BE%A4%E5%A4%87%E6%B3%A8)
    async fn set_group_card(&self, group_id: i64, user_id: i64, card: String) -> Result<()>;

    /// [设置群组专属头衔](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%BE%A4%E7%BB%84%E4%B8%93%E5%B1%9E%E5%A4%B4%E8%A1%94)
    async fn set_group_special_title(
        &self,
        group_id: i64,
        user_id: i64,
        special_title: String,
        duration: u32,
    ) -> Result<()>;

    /// [群单人禁言](https://docs.go-cqhttp.org/api/#%E7%BE%A4%E5%8D%95%E4%BA%BA%E7%A6%81%E8%A8%80)
    async fn set_group_ban(&self, group_id: i64, user_id: i64, duration: u32) -> Result<()>;

    /// [群全员禁言](https://docs.go-cqhttp.org/api/#%E7%BE%A4%E5%85%A8%E5%91%98%E7%A6%81%E8%A8%80)
    async fn set_group_whole_ban(&self, group_id: i64, enable: bool) -> Result<()>;

    /// [群匿名用户禁言](https://docs.go-cqhttp.org/api/#%E7%BE%A4%E5%8C%BF%E5%90%8D%E7%94%A8%E6%88%B7%E7%A6%81%E8%A8%80)
    ///
    /// 提示：`anonymous`和`flag`两者任选其一传入即可, 若都传入, 则使用`anonymous`
    ///
    /// **注意**：该API从 go-cqhttp-v0.9.36 开始支持
    async fn set_group_anonymous_ban(
        &self,
        group_id: i64,
        anonymous: Option<AnonymousGroupMsg>,
        flag: Option<String>,
        duration: u32,
    ) -> Result<()>;

    /// [设置精华消息](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%B2%BE%E5%8D%8E%E6%B6%88%E6%81%AF)
    async fn set_essence_msg(&self, message_id: i32) -> Result<()>;

    /// [移出精华消息](https://docs.go-cqhttp.org/api/#%E7%A7%BB%E5%87%BA%E7%B2%BE%E5%8D%8E%E6%B6%88%E6%81%AF)
    async fn delete_essence_msg(&self, message_id: i32) -> Result<()>;

    /// [群打卡](https://docs.go-cqhttp.org/api/#%E7%BE%A4%E6%89%93%E5%8D%A1)
    async fn send_group_sign(&self, group_id: i64) -> Result<()>;

    /// [群设置匿名](https://docs.go-cqhttp.org/api/#%E7%BE%A4%E8%AE%BE%E7%BD%AE%E5%8C%BF%E5%90%8D)
    async fn set_group_anonymous(&self, group_id: i64, enable: bool) -> Result<()>;

    /// [发送群公告](https://docs.go-cqhttp.org/api/#%E5%8F%91%E9%80%81%E7%BE%A4%E5%85%AC%E5%91%8A)
    async fn _send_group_notice(
        &self,
        group_id: i64,
        content: String,
        image: Option<String>,
    ) -> Result<()>;

    /// [获取群公告](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E5%85%AC%E5%91%8A)
    async fn _get_group_notice(&self, group_id: i64) -> Result<GroupNotice>;

    /// [群组踢人](https://docs.go-cqhttp.org/api/#%E7%BE%A4%E7%BB%84%E8%B8%A2%E4%BA%BA)
    async fn set_group_kick(
        &self,
        group_id: i64,
        user_id: i64,
        reject_add_request: bool,
    ) -> Result<()>;

    /// [退出群组](https://docs.go-cqhttp.org/api/#%E9%80%80%E5%87%BA%E7%BE%A4%E7%BB%84)
    async fn set_group_leave(&self, group_id: i64, is_dismiss: bool) -> Result<()>;

    /// [上传群文件](https://docs.go-cqhttp.org/api/#%E4%B8%8A%E4%BC%A0%E7%BE%A4%E6%96%87%E4%BB%B6)
    ///
    /// **注意**：在不提供 folder 参数的情况下默认上传到根目录
    ///
    /// **注意**：只能上传本地文件, 需要上传http文件的话请先调用[`download_file`] API下载
    async fn upload_group_file(
        &self,
        group_id: i64,
        file: String,
        name: String,
        folder: Option<String>,
    ) -> Result<()>;

    /// [删除群文件](https://docs.go-cqhttp.org/api/#%E5%88%A0%E9%99%A4%E7%BE%A4%E6%96%87%E4%BB%B6)
    async fn delete_group_file(&self, group_id: i64, file_id: String, busid: i32) -> Result<()>;

    /// [创建群文件文件夹](https://docs.go-cqhttp.org/api/#%E5%88%9B%E5%BB%BA%E7%BE%A4%E6%96%87%E4%BB%B6%E6%96%87%E4%BB%B6%E5%A4%B9)
    ///
    /// **注意**：仅能在根目录创建文件夹
    async fn create_group_file_folder(
        &self,
        group_id: i64,
        name: String,
        parent_id: String,
    ) -> Result<()>;

    /// [删除群文件文件夹](https://docs.go-cqhttp.org/api/#%E5%88%A0%E9%99%A4%E7%BE%A4%E6%96%87%E4%BB%B6%E6%96%87%E4%BB%B6%E5%A4%B9)
    async fn delete_group_folder(&self, group_id: i64, folder_id: String) -> Result<()>;

    /// [获取群文件系统信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%96%87%E4%BB%B6%E7%B3%BB%E7%BB%9F%E4%BF%A1%E6%81%AF)
    async fn get_group_file_system_info(&self, group_id: i64) -> Result<GroupFileSystemInfo>;

    /// [获取群根目录文件列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%A0%B9%E7%9B%AE%E5%BD%95%E6%96%87%E4%BB%B6%E5%88%97%E8%A1%A8)
    async fn get_group_root_files(&self, group_id: i64) -> Result<GroupFiles>;

    /// [获取群子目录文件列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E5%AD%90%E7%9B%AE%E5%BD%95%E6%96%87%E4%BB%B6%E5%88%97%E8%A1%A8)
    async fn get_group_files_by_folder(
        &self,
        group_id: i64,
        folder_id: String,
    ) -> Result<GroupFiles>;

    /// [获取群文件资源链接](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%96%87%E4%BB%B6%E8%B5%84%E6%BA%90%E9%93%BE%E6%8E%A5)
    async fn get_group_file_url(
        &self,
        group_id: i64,
        file_id: String,
        busid: i32,
    ) -> Result<GroupFileUrl>;

    /// [上传私聊文件](https://docs.go-cqhttp.org/api/#%E4%B8%8A%E4%BC%A0%E7%A7%81%E8%81%8A%E6%96%87%E4%BB%B6)
    ///
    /// **注意**：只能上传本地文件, 需要上传http文件的话请先调用[`download_file`] API下载
    async fn upload_private_file(&self, user_id: i64, file: String, name: String) -> Result<()>;

    /// [获取 Cookies](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96-cookies)
    ///
    /// **注意**；该API暂未被go-cqhttp支持
    async fn _get_cookies(&self, _domain: String) -> Result<Cookies> {
        unimplemented!()
    }

    /// [获取CSRF Token](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96-csrf-token)
    ///
    /// **注意**；该API暂未被go-cqhttp支持
    async fn _get_csrf_token(&self) -> Result<CSRFToken> {
        unimplemented!()
    }

    /// [获取QQ相关接口凭证](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96-qq-%E7%9B%B8%E5%85%B3%E6%8E%A5%E5%8F%A3%E5%87%AD%E8%AF%81)
    ///
    /// **注意**；该API暂未被go-cqhttp支持
    async fn _get_credentials(&self) -> Result<Credentials> {
        unimplemented!()
    }

    /// [获取版本信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%89%88%E6%9C%AC%E4%BF%A1%E6%81%AF)
    async fn get_version_info(&self) -> Result<VersionInfo>;

    /// [获取状态](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%8A%B6%E6%80%81)
    ///
    /// **注意**：所有统计信息都将在重启后重置
    async fn get_status(&self) -> Result<Status>;

    /// [清理缓存](https://docs.go-cqhttp.org/api/#%E6%B8%85%E7%90%86%E7%BC%93%E5%AD%98)
    ///
    /// **注意**；该API暂未被go-cqhttp支持
    async fn clean_cache(&self) -> Result<()> {
        unimplemented!()
    }

    /// [重载事件过滤器](https://docs.go-cqhttp.org/api/#%E9%87%8D%E8%BD%BD%E4%BA%8B%E4%BB%B6%E8%BF%87%E6%BB%A4%E5%99%A8)
    async fn reload_event_filter(&self, file: String) -> Result<()>;

    /// [下载文件到缓存目录](https://docs.go-cqhttp.org/api/#%E4%B8%8B%E8%BD%BD%E6%96%87%E4%BB%B6%E5%88%B0%E7%BC%93%E5%AD%98%E7%9B%AE%E5%BD%95)
    ///
    /// 提示：通过这个API下载的文件能直接放入CQ码作为图片或语音发送
    ///
    /// 提示：调用后会阻塞直到下载完成后才会返回数据，请注意下载大文件时的超时
    async fn download_file(
        &self,
        url: String,
        thread_count: i32,
        headers: Vec<String>,
    ) -> Result<DownloadedFile>;

    /// [检查链接安全性](https://docs.go-cqhttp.org/api/#%E6%A3%80%E6%9F%A5%E9%93%BE%E6%8E%A5%E5%AE%89%E5%85%A8%E6%80%A7)
    async fn check_url_safely(&self, url: String) -> Result<UrlSafety>;

    /// [获取中文分词(隐藏API)](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E4%B8%AD%E6%96%87%E5%88%86%E8%AF%8D-%E9%9A%90%E8%97%8F-api)
    ///
    /// **警告**：隐藏API是不建议一般用户使用的, 它们只应该在OneBot实现内部或由SDK和框架使用, 因为不正确的使用可能造成程序运行不正常。
    async fn get_word_slices(&self, content: String) -> Result<WordSlices>;
}

/// API状态
#[derive(Debug, Eq, PartialEq)]
pub enum APIStatus {
    /// api调用成功
    Ok,
    /// api调用已经提交异步处理, 此时retcode为1, 具体api调用是否成功无法得知
    Async,
    /// api调用失败
    Failed,
}

impl ToString for APIStatus {
    fn to_string(&self) -> String {
        match self {
            APIStatus::Ok => "ok",
            APIStatus::Async => "async",
            APIStatus::Failed => "failed",
        }
        .to_string()
    }
}

impl<T: AsRef<str>> From<T> for APIStatus {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "ok" => APIStatus::Ok,
            "async" => APIStatus::Async,
            _ => APIStatus::Failed,
        }
    }
}

impl<'de> Deserialize<'de> for APIStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<APIStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct APIResponse<T> {
    pub status: APIStatus,
    pub retcode: i32,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub wording: String,
    pub data: Option<T>,
    #[serde(default)]
    pub echo: String,
}

impl<T: DeserializeOwned> APIResponse<T> {
    pub async fn from_http(resp: Response) -> Result<Self> {
        // https://docs.go-cqhttp.org/api/#%E5%93%8D%E5%BA%94%E8%AF%B4%E6%98%8E
        match resp.status() {
            StatusCode::OK => {}
            StatusCode::UNAUTHORIZED => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "access token未提供",
                )))
            }
            StatusCode::FORBIDDEN => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "access token不符合",
                )))
            }
            StatusCode::NOT_FOUND => {
                return Err(Box::new(Error::new(ErrorKind::InvalidData, "API不存在")))
            }
            StatusCode::NOT_ACCEPTABLE => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "Content-Type不支持(非`application/json`或`application/x-www-form-urlencoded`)",
                )))
            }
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "请求失败: 代码{}，内容: {}",
                        resp.status(),
                        resp.text().await?
                    ),
                )))
            }
        }
        let ret: APIResponse<T> = resp.json().await?;
        if ret.is_failed() {
            Err(Box::new(Error::new(
                ErrorKind::InvalidData,
                format!("请求失败: {}", ret.message),
            )))
        } else {
            Ok(ret)
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == APIStatus::Ok
    }

    pub fn is_async(&self) -> bool {
        self.status == APIStatus::Async
    }

    pub fn is_failed(&self) -> bool {
        self.status == APIStatus::Failed
    }
}
