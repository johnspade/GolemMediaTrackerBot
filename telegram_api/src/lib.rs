use frankenstein::api_params::AddStickerToSetParams;
use frankenstein::api_params::AnswerCallbackQueryParams;
use frankenstein::api_params::AnswerInlineQueryParams;
use frankenstein::api_params::AnswerPreCheckoutQueryParams;
use frankenstein::api_params::AnswerShippingQueryParams;
use frankenstein::api_params::AnswerWebAppQueryParams;
use frankenstein::api_params::ApproveChatJoinRequestParams;
use frankenstein::api_params::BanChatMemberParams;
use frankenstein::api_params::BanChatSenderChatParams;
use frankenstein::api_params::CloseForumTopicParams;
use frankenstein::api_params::CloseGeneralForumTopicParams;
use frankenstein::api_params::CopyMessageParams;
use frankenstein::api_params::CreateChatInviteLinkParams;
use frankenstein::api_params::CreateForumTopicParams;
use frankenstein::api_params::CreateInvoiceLinkParams;
use frankenstein::api_params::CreateNewStickerSetParams;
use frankenstein::api_params::DeclineChatJoinRequestParams;
use frankenstein::api_params::DeleteChatPhotoParams;
use frankenstein::api_params::DeleteChatStickerSetParams;
use frankenstein::api_params::DeleteForumTopicParams;
use frankenstein::api_params::DeleteMessageParams;
use frankenstein::api_params::DeleteMyCommandsParams;
use frankenstein::api_params::DeleteStickerFromSetParams;
use frankenstein::api_params::DeleteStickerSetParams;
use frankenstein::api_params::DeleteWebhookParams;
use frankenstein::api_params::EditChatInviteLinkParams;
use frankenstein::api_params::EditForumTopicParams;
use frankenstein::api_params::EditGeneralForumTopicParams;
use frankenstein::api_params::EditMessageCaptionParams;
use frankenstein::api_params::EditMessageLiveLocationParams;
use frankenstein::api_params::EditMessageMediaParams;
use frankenstein::api_params::EditMessageReplyMarkupParams;
use frankenstein::api_params::EditMessageTextParams;
use frankenstein::api_params::ExportChatInviteLinkParams;
use frankenstein::api_params::FileUpload;
use frankenstein::api_params::ForwardMessageParams;
use frankenstein::api_params::GetChatAdministratorsParams;
use frankenstein::api_params::GetChatMemberCountParams;
use frankenstein::api_params::GetChatMemberParams;
use frankenstein::api_params::GetChatMenuButtonParams;
use frankenstein::api_params::GetChatParams;
use frankenstein::api_params::GetFileParams;
use frankenstein::api_params::GetGameHighScoresParams;
use frankenstein::api_params::GetMyCommandsParams;
use frankenstein::api_params::GetMyDefaultAdministratorRightsParams;
use frankenstein::api_params::GetMyDescriptionParams;
use frankenstein::api_params::GetMyNameParams;
use frankenstein::api_params::GetMyShortDescriptionParams;
use frankenstein::api_params::GetStickerSetParams;
use frankenstein::api_params::GetUpdatesParams;
use frankenstein::api_params::GetUserProfilePhotosParams;
use frankenstein::api_params::HideGeneralForumTopicParams;
use frankenstein::api_params::InputMedia;
use frankenstein::api_params::LeaveChatParams;
use frankenstein::api_params::Media;
use frankenstein::api_params::PinChatMessageParams;
use frankenstein::api_params::PromoteChatMemberParams;
use frankenstein::api_params::ReopenForumTopicParams;
use frankenstein::api_params::ReopenGeneralForumTopicParams;
use frankenstein::api_params::RestrictChatMemberParams;
use frankenstein::api_params::RevokeChatInviteLinkParams;
use frankenstein::api_params::SendAnimationParams;
use frankenstein::api_params::SendAudioParams;
use frankenstein::api_params::SendChatActionParams;
use frankenstein::api_params::SendContactParams;
use frankenstein::api_params::SendDiceParams;
use frankenstein::api_params::SendDocumentParams;
use frankenstein::api_params::SendGameParams;
use frankenstein::api_params::SendInvoiceParams;
use frankenstein::api_params::SendLocationParams;
use frankenstein::api_params::SendMediaGroupParams;
use frankenstein::api_params::SendMessageParams;
use frankenstein::api_params::SendPhotoParams;
use frankenstein::api_params::SendPollParams;
use frankenstein::api_params::SendStickerParams;
use frankenstein::api_params::SendVenueParams;
use frankenstein::api_params::SendVideoNoteParams;
use frankenstein::api_params::SendVideoParams;
use frankenstein::api_params::SendVoiceParams;
use frankenstein::api_params::SetChatAdministratorCustomTitleParams;
use frankenstein::api_params::SetChatDescriptionParams;
use frankenstein::api_params::SetChatMenuButtonParams;
use frankenstein::api_params::SetChatPermissionsParams;
use frankenstein::api_params::SetChatPhotoParams;
use frankenstein::api_params::SetChatStickerSetParams;
use frankenstein::api_params::SetChatTitleParams;
use frankenstein::api_params::SetCustomEmojiStickerSetThumbnailParams;
use frankenstein::api_params::SetGameScoreParams;
use frankenstein::api_params::SetMyCommandsParams;
use frankenstein::api_params::SetMyDefaultAdministratorRightsParams;
use frankenstein::api_params::SetMyDescriptionParams;
use frankenstein::api_params::SetMyNameParams;
use frankenstein::api_params::SetMyShortDescriptionParams;
use frankenstein::api_params::SetStickerEmojiListParams;
use frankenstein::api_params::SetStickerKeywordsParams;
use frankenstein::api_params::SetStickerMaskPositionParams;
use frankenstein::api_params::SetStickerPositionInSetParams;
use frankenstein::api_params::SetStickerSetThumbnailParams;
use frankenstein::api_params::SetStickerSetTitleParams;
use frankenstein::api_params::SetWebhookParams;
use frankenstein::api_params::StopMessageLiveLocationParams;
use frankenstein::api_params::StopPollParams;
use frankenstein::api_params::UnbanChatMemberParams;
use frankenstein::api_params::UnbanChatSenderChatParams;
use frankenstein::api_params::UnhideGeneralForumTopicParams;
use frankenstein::api_params::UnpinAllForumTopicMessagesParams;
use frankenstein::api_params::UnpinChatMessageParams;
use frankenstein::api_params::UploadStickerFileParams;
use frankenstein::objects::BotCommand;
use frankenstein::objects::BotDescription;
use frankenstein::objects::BotName;
use frankenstein::objects::BotShortDescription;
use frankenstein::objects::Chat;
use frankenstein::objects::ChatAdministratorRights;
use frankenstein::objects::ChatInviteLink;
use frankenstein::objects::ChatMember;
use frankenstein::objects::File as FileObject;
use frankenstein::objects::ForumTopic;
use frankenstein::objects::GameHighScore;
use frankenstein::objects::InputSticker;
use frankenstein::objects::MenuButton;
use frankenstein::objects::MessageId;
use frankenstein::objects::Poll;
use frankenstein::objects::SentWebAppMessage;
use frankenstein::objects::StickerSet;
use frankenstein::objects::Update;
use frankenstein::objects::User;
use frankenstein::objects::UserProfilePhotos;
use frankenstein::objects::WebhookInfo;
use frankenstein::GetCustomEmojiStickersParams;
use frankenstein::Sticker;
use frankenstein::UnpinAllGeneralForumTopicMessagesParams;
use std::path::PathBuf;

use frankenstein::objects::{Message, ResponseParameters};
use serde::{Deserialize, Serialize};

use std::time::Duration;

use typed_builder::TypedBuilder;

pub fn send_message(api: &Api, chat_id: i64, text: &str) { // todo move to a separate module
    let message_params = SendMessageParams::builder()
        .chat_id(chat_id)
        .text(text)
        .build();
    let result = api.send_message(&message_params);
    if let Err(err) = result {
        println!("Error sending message: {:?}", err);
    };
}

const BASE_API_URL: &str = "https://api.telegram.org/bot";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    #[error("{0}")]
    Http(HttpError),
    #[error("Api Error {0:?}")]
    Api(ErrorResponse),
    #[error("Decode Error {0}")]
    Decode(String),
    #[error("Encode Error {0}")]
    Encode(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("Http Error {code}: {message}")]
pub struct HttpError {
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Api {
    #[builder(setter(into))]
    pub api_url: String,
    #[builder(
        default_code = "reqwest::ClientBuilder::new().connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(500)).build().unwrap()"
    )]
    pub client: reqwest::Client,
}

impl Api {

    /// Create a new `Api`. You can use `Api::builder()` for more options.
    pub fn new(api_key: &str) -> Self {
        let api_url = format!("{}{api_key}", BASE_API_URL);

        Self::builder().api_url(api_url).build()
    }

    /// Create a new `Api`. You can use `Api::builder()` for more options.
    // pub fn new_url<T: Into<String>>(api_url: T) -> Self {
    //     Api::builder().api_url(api_url).build()
    // }

    pub fn encode_params<T: serde::ser::Serialize + std::fmt::Debug>(
        params: &T,
    ) -> Result<String, Error> {
        serde_json::to_string(params).map_err(|e| Error::Encode(format!("{e:?} : {params:?}")))
    }

    pub fn decode_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, Error> {
        let status_code = response.status().as_u16();
        match response.text() {
            Ok(message) => {
                if status_code == 200 {
                    let success_response: T = Self::parse_json(&message)?;
                    return Ok(success_response);
                }

                let error_response: ErrorResponse = Self::parse_json(&message)?;
                Err(Error::Api(error_response))
            }
            Err(e) => {
                let err = Error::Decode(format!("Failed to decode response: {e:?}"));
                Err(err)
            }
        }
    }

    fn parse_json<T: serde::de::DeserializeOwned>(body: &str) -> Result<T, Error> {
        let json_result: Result<T, serde_json::Error> = serde_json::from_str(body);

        match json_result {
            Ok(result) => Ok(result),

            Err(e) => {
                let err = Error::Decode(format!("{e:?} : {body:?}"));
                Err(err)
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        let message = error.to_string();
        let code = error
            .status()
            .map_or(500, |status_code| status_code.as_u16());

        let error = HttpError { code, message };
        Self::Http(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        let message = error.to_string();

        Self::Encode(message)
    }
}

impl TelegramApi for Api {
    type Error = Error;

    fn request<
        T1: serde::ser::Serialize + std::fmt::Debug,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Self::Error> {
        let url = format!("{}/{method}", self.api_url);

        let mut prepared_request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");

        prepared_request = if let Some(data) = params {
            let json_string = Self::encode_params(&data)?;

            prepared_request.body(json_string)
        } else {
            prepared_request
        };

        let response = prepared_request.send()?;
        let parsed_response: T2 = Self::decode_response(response)?;

        Ok(parsed_response)
    }

    fn request_with_form_data<
        T1: serde::ser::Serialize + std::fmt::Debug,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        _method: &str,
        _params: T1,
        _files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Error> {
        unimplemented!("no multipart");
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MethodResponse<T> {
    /// Always true
    pub ok: bool,
    pub result: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
/// \[…\] an unsuccessful request, `ok` equals false and the error is explained in the `description`.
/// An Integer `error_code` field is also returned, but its contents are subject to change in the future.
/// Some errors may also have an optional field `parameters` of the type `ResponseParameters`, which can help to automatically handle the error.
///
/// See <https://core.telegram.org/bots/api#making-requests>
pub struct ErrorResponse {
    /// Always false
    pub ok: bool,
    pub description: String,
    /// Contents are subject to change in the future
    pub error_code: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EditMessageResponse {
    Message(MethodResponse<Message>),
    Bool(MethodResponse<bool>),
}

pub trait TelegramApi {
    type Error;

    fn get_updates(
        &self,
        params: &GetUpdatesParams,
    ) -> Result<MethodResponse<Vec<Update>>, Self::Error> {
        self.request("getUpdates", Some(params))
    }

    fn send_message(
        &self,
        params: &SendMessageParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendMessage", Some(params))
    }

    fn set_webhook(&self, params: &SetWebhookParams) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setWebhook", Some(params))
    }

    fn delete_webhook(
        &self,
        params: &DeleteWebhookParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteWebhook", Some(params))
    }

    fn get_webhook_info(&self) -> Result<MethodResponse<WebhookInfo>, Self::Error> {
        self.request_without_body("getWebhookInfo")
    }

    fn get_me(&self) -> Result<MethodResponse<User>, Self::Error> {
        self.request_without_body("getMe")
    }

    fn log_out(&self) -> Result<MethodResponse<bool>, Self::Error> {
        self.request_without_body("logOut")
    }

    fn close(&self) -> Result<MethodResponse<bool>, Self::Error> {
        self.request_without_body("close")
    }

    fn forward_message(
        &self,
        params: &ForwardMessageParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("forwardMessage", Some(params))
    }

    fn copy_message(
        &self,
        params: &CopyMessageParams,
    ) -> Result<MethodResponse<MessageId>, Self::Error> {
        self.request("copyMessage", Some(params))
    }

    fn send_photo(&self, params: &SendPhotoParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendPhoto";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.photo {
            files.push(("photo", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_audio(&self, params: &SendAudioParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendAudio";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.audio {
            files.push(("audio", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_media_group(
        &self,
        params: &SendMediaGroupParams,
    ) -> Result<MethodResponse<Vec<Message>>, Self::Error> {
        let method_name = "sendMediaGroup";
        let mut files: Vec<(String, PathBuf)> = vec![];
        let mut new_medias: Vec<Media> = vec![];
        let mut file_idx = 0;

        for media in &params.media {
            match media {
                Media::Audio(audio) => {
                    let mut new_audio = audio.clone();

                    if let FileUpload::InputFile(input_file) = &audio.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_audio.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    if let Some(FileUpload::InputFile(input_file)) = &audio.thumbnail {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_audio.thumbnail = Some(FileUpload::String(attach_name));

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Audio(new_audio));
                }

                Media::Document(document) => {
                    let mut new_document = document.clone();

                    if let FileUpload::InputFile(input_file) = &document.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_document.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Document(new_document));
                }
                Media::Photo(photo) => {
                    let mut new_photo = photo.clone();

                    if let FileUpload::InputFile(input_file) = &photo.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_photo.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Photo(new_photo));
                }

                Media::Video(video) => {
                    let mut new_video = video.clone();

                    if let FileUpload::InputFile(input_file) = &video.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_video.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    if let Some(FileUpload::InputFile(input_file)) = &video.thumbnail {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_video.thumbnail = Some(FileUpload::String(attach_name));

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Video(new_video));
                }
            };
        }

        let mut new_params = params.clone();
        new_params.media = new_medias;

        let files_with_str_names: Vec<(&str, PathBuf)> = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data(method_name, &new_params, files_with_str_names)
    }

    fn send_document(
        &self,
        params: &SendDocumentParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendDocument";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.document {
            files.push(("document", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_video(&self, params: &SendVideoParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVideo";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.video {
            files.push(("video", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_animation(
        &self,
        params: &SendAnimationParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendAnimation";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.animation {
            files.push(("animation", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_voice(&self, params: &SendVoiceParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVoice";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.voice {
            files.push(("voice", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_video_note(
        &self,
        params: &SendVideoNoteParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVideoNote";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.video_note {
            files.push(("video_note", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_location(
        &self,
        params: &SendLocationParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendLocation", Some(params))
    }

    fn edit_message_live_location(
        &self,
        params: &EditMessageLiveLocationParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageLiveLocation", Some(params))
    }

    fn stop_message_live_location(
        &self,
        params: &StopMessageLiveLocationParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("stopMessageLiveLocation", Some(params))
    }

    fn send_venue(&self, params: &SendVenueParams) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendVenue", Some(params))
    }

    fn send_contact(
        &self,
        params: &SendContactParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendContact", Some(params))
    }

    fn send_poll(&self, params: &SendPollParams) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendPoll", Some(params))
    }

    fn send_dice(&self, params: &SendDiceParams) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendDice", Some(params))
    }

    fn send_chat_action(
        &self,
        params: &SendChatActionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("sendChatAction", Some(params))
    }

    fn get_user_profile_photos(
        &self,
        params: &GetUserProfilePhotosParams,
    ) -> Result<MethodResponse<UserProfilePhotos>, Self::Error> {
        self.request("getUserProfilePhotos", Some(params))
    }

    fn get_file(&self, params: &GetFileParams) -> Result<MethodResponse<FileObject>, Self::Error> {
        self.request("getFile", Some(params))
    }

    fn ban_chat_member(
        &self,
        params: &BanChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("banChatMember", Some(params))
    }

    fn unban_chat_member(
        &self,
        params: &UnbanChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unbanChatMember", Some(params))
    }

    fn restrict_chat_member(
        &self,
        params: &RestrictChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("restrictChatMember", Some(params))
    }

    fn promote_chat_member(
        &self,
        params: &PromoteChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("promoteChatMember", Some(params))
    }

    fn set_chat_administrator_custom_title(
        &self,
        params: &SetChatAdministratorCustomTitleParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatAdministratorCustomTitle", Some(params))
    }

    fn ban_chat_sender_chat(
        &self,
        params: &BanChatSenderChatParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("banChatSenderChat", Some(params))
    }

    fn unban_chat_sender_chat(
        &self,
        params: &UnbanChatSenderChatParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unbanChatSenderChat", Some(params))
    }

    fn set_chat_permissions(
        &self,
        params: &SetChatPermissionsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatPermissions", Some(params))
    }

    fn export_chat_invite_link(
        &self,
        params: &ExportChatInviteLinkParams,
    ) -> Result<MethodResponse<String>, Self::Error> {
        self.request("exportChatInviteLink", Some(params))
    }

    fn create_chat_invite_link(
        &self,
        params: &CreateChatInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("createChatInviteLink", Some(params))
    }

    fn edit_chat_invite_link(
        &self,
        params: &EditChatInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("editChatInviteLink", Some(params))
    }

    fn revoke_chat_invite_link(
        &self,
        params: &RevokeChatInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("revokeChatInviteLink", Some(params))
    }

    fn approve_chat_join_request(
        &self,
        params: &ApproveChatJoinRequestParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("approveChatJoinRequest", Some(params))
    }

    fn decline_chat_join_request(
        &self,
        params: &DeclineChatJoinRequestParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("declineChatJoinRequest", Some(params))
    }

    fn set_chat_photo(
        &self,
        params: &SetChatPhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let photo = &params.photo;

        self.request_with_form_data("setChatPhoto", params, vec![("photo", photo.path.clone())])
    }

    fn delete_chat_photo(
        &self,
        params: &DeleteChatPhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteChatPhoto", Some(params))
    }

    fn set_chat_title(
        &self,
        params: &SetChatTitleParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatTitle", Some(params))
    }

    fn set_chat_description(
        &self,
        params: &SetChatDescriptionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatDescription", Some(params))
    }

    fn pin_chat_message(
        &self,
        params: &PinChatMessageParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("pinChatMessage", Some(params))
    }

    fn unpin_chat_message(
        &self,
        params: &UnpinChatMessageParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unpinChatMessage", Some(params))
    }

    fn leave_chat(&self, params: &LeaveChatParams) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("leaveChat", Some(params))
    }

    fn get_chat(&self, params: &GetChatParams) -> Result<MethodResponse<Chat>, Self::Error> {
        self.request("getChat", Some(params))
    }

    fn get_chat_administrators(
        &self,
        params: &GetChatAdministratorsParams,
    ) -> Result<MethodResponse<Vec<ChatMember>>, Self::Error> {
        self.request("getChatAdministrators", Some(params))
    }

    fn get_chat_member_count(
        &self,
        params: &GetChatMemberCountParams,
    ) -> Result<MethodResponse<u32>, Self::Error> {
        self.request("getChatMemberCount", Some(params))
    }

    fn get_chat_member(
        &self,
        params: &GetChatMemberParams,
    ) -> Result<MethodResponse<ChatMember>, Self::Error> {
        self.request("getChatMember", Some(params))
    }

    fn set_chat_sticker_set(
        &self,
        params: &SetChatStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatStickerSet", Some(params))
    }

    fn delete_chat_sticker_set(
        &self,
        params: &DeleteChatStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteChatStickerSet", Some(params))
    }

    fn get_forum_topic_icon_stickers(&self) -> Result<MethodResponse<Vec<Sticker>>, Self::Error> {
        self.request_without_body("getForumTopicIconStickers")
    }

    fn create_forum_topic(
        &self,
        params: &CreateForumTopicParams,
    ) -> Result<MethodResponse<ForumTopic>, Self::Error> {
        self.request("createForumTopic", Some(params))
    }

    fn edit_forum_topic(
        &self,
        params: &EditForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("editForumTopic", Some(params))
    }

    fn close_forum_topic(
        &self,
        params: &CloseForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("closeForumTopic", Some(params))
    }

    fn reopen_forum_topic(
        &self,
        params: &ReopenForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("reopenForumTopic", Some(params))
    }

    fn delete_forum_topic(
        &self,
        params: &DeleteForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteForumTopic", Some(params))
    }

    fn unpin_all_forum_topic_messages(
        &self,
        params: &UnpinAllForumTopicMessagesParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unpinAllForumTopicMessages", Some(params))
    }

    fn edit_general_forum_topic(
        &self,
        params: &EditGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("editGeneralForumTopic", Some(params))
    }

    fn close_general_forum_topic(
        &self,
        params: &CloseGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("closeGeneralForumTopic", Some(params))
    }

    fn reopen_general_forum_topic(
        &self,
        params: &ReopenGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("reopenGeneralForumTopic", Some(params))
    }

    fn hide_general_forum_topic(
        &self,
        params: &HideGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("hideGeneralForumTopic", Some(params))
    }

    fn unhide_general_forum_topic(
        &self,
        params: &UnhideGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unhideGeneralForumTopic", Some(params))
    }

    fn answer_callback_query(
        &self,
        params: &AnswerCallbackQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerCallbackQuery", Some(params))
    }

    fn get_my_commands(
        &self,
        params: &GetMyCommandsParams,
    ) -> Result<MethodResponse<Vec<BotCommand>>, Self::Error> {
        self.request("getMyCommands", Some(params))
    }

    fn set_my_commands(
        &self,
        params: &SetMyCommandsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyCommands", Some(params))
    }

    fn delete_my_commands(
        &self,
        params: &DeleteMyCommandsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteMyCommands", Some(params))
    }

    fn set_my_name(&self, params: &SetMyNameParams) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyName", Some(params))
    }

    fn get_my_name(
        &self,
        params: &GetMyNameParams,
    ) -> Result<MethodResponse<BotName>, Self::Error> {
        self.request("getMyName", Some(params))
    }

    fn set_my_description(
        &self,
        params: &SetMyDescriptionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyDescription", Some(params))
    }

    fn get_my_description(
        &self,
        params: &GetMyDescriptionParams,
    ) -> Result<MethodResponse<BotDescription>, Self::Error> {
        self.request("getMyDescription", Some(params))
    }

    fn set_my_short_description(
        &self,
        params: &SetMyShortDescriptionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyShortDescription", Some(params))
    }

    fn get_my_short_description(
        &self,
        params: &GetMyShortDescriptionParams,
    ) -> Result<MethodResponse<BotShortDescription>, Self::Error> {
        self.request("getMyShortDescription", Some(params))
    }

    fn answer_inline_query(
        &self,
        params: &AnswerInlineQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerInlineQuery", Some(params))
    }

    fn edit_message_text(
        &self,
        params: &EditMessageTextParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageText", Some(params))
    }

    fn edit_message_caption(
        &self,
        params: &EditMessageCaptionParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageCaption", Some(params))
    }

    fn edit_message_media(
        &self,
        params: &EditMessageMediaParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        let method_name = "editMessageMedia";
        let mut files: Vec<(String, PathBuf)> = vec![];

        let new_media = match &params.media {
            InputMedia::Animation(animation) => {
                let mut new_animation = animation.clone();

                if let FileUpload::InputFile(input_file) = &animation.media {
                    let name = "animation".to_string();
                    let attach_name = format!("attach://{name}");

                    new_animation.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &animation.thumbnail {
                    let name = "animation_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_animation.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Animation(new_animation)
            }
            InputMedia::Document(document) => {
                let mut new_document = document.clone();

                if let FileUpload::InputFile(input_file) = &document.media {
                    let name = "document".to_string();
                    let attach_name = format!("attach://{name}");

                    new_document.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &document.thumbnail {
                    let name = "document_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_document.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Document(new_document)
            }
            InputMedia::Audio(audio) => {
                let mut new_audio = audio.clone();

                if let FileUpload::InputFile(input_file) = &audio.media {
                    let name = "audio".to_string();
                    let attach_name = format!("attach://{name}");

                    new_audio.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &audio.thumbnail {
                    let name = "audio_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_audio.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Audio(new_audio)
            }
            InputMedia::Photo(photo) => {
                let mut new_photo = photo.clone();

                if let FileUpload::InputFile(input_file) = &photo.media {
                    let name = "photo".to_string();
                    let attach_name = format!("attach://{name}");

                    new_photo.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Photo(new_photo)
            }
            InputMedia::Video(video) => {
                let mut new_video = video.clone();

                if let FileUpload::InputFile(input_file) = &video.media {
                    let name = "video".to_string();
                    let attach_name = format!("attach://{name}");

                    new_video.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &video.thumbnail {
                    let name = "video_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_video.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Video(new_video)
            }
        };

        let mut new_params = params.clone();
        new_params.media = new_media;

        let files_with_str_names: Vec<(&str, PathBuf)> = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data(method_name, &new_params, files_with_str_names)
    }

    fn edit_message_reply_markup(
        &self,
        params: &EditMessageReplyMarkupParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageReplyMarkup", Some(params))
    }

    fn stop_poll(&self, params: &StopPollParams) -> Result<MethodResponse<Poll>, Self::Error> {
        self.request("stopPoll", Some(params))
    }

    fn delete_message(
        &self,
        params: &DeleteMessageParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteMessage", Some(params))
    }

    fn send_sticker(
        &self,
        params: &SendStickerParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendSticker";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.sticker {
            files.push(("sticker", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn get_sticker_set(
        &self,
        params: &GetStickerSetParams,
    ) -> Result<MethodResponse<StickerSet>, Self::Error> {
        self.request("getStickerSet", Some(params))
    }

    fn upload_sticker_file(
        &self,
        params: &UploadStickerFileParams,
    ) -> Result<MethodResponse<FileObject>, Self::Error> {
        let sticker = &params.sticker;

        self.request_with_form_data(
            "uploadStickerFile",
            params,
            vec![("sticker", sticker.path.clone())],
        )
    }

    fn create_new_sticker_set(
        &self,
        params: &CreateNewStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "createNewStickerSet";
        let mut new_stickers: Vec<InputSticker> = vec![];
        let mut files: Vec<(String, PathBuf)> = vec![];
        let mut file_idx = 0;

        for sticker in &params.stickers {
            let mut new_sticker = sticker.clone();

            if let FileUpload::InputFile(input_file) = &sticker.sticker {
                let name = format!("file{file_idx}");
                let attach_name = format!("attach://{name}");
                file_idx += 1;

                new_sticker.sticker = FileUpload::String(attach_name);

                files.push((name, input_file.path.clone()));
            };

            new_stickers.push(new_sticker);
        }

        let mut new_params = params.clone();
        new_params.stickers = new_stickers;

        let files_with_str_names: Vec<(&str, PathBuf)> = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data(method_name, &new_params, files_with_str_names)
    }

    fn get_custom_emoji_stickers(
        &self,
        params: &GetCustomEmojiStickersParams,
    ) -> Result<MethodResponse<Vec<Sticker>>, Self::Error> {
        self.request("getCustomEmojiStickers", Some(params))
    }

    fn add_sticker_to_set(
        &self,
        params: &AddStickerToSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "addStickerToSet";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.sticker.sticker {
            files.push(("sticker", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn set_sticker_position_in_set(
        &self,
        params: &SetStickerPositionInSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerPositionInSet", Some(params))
    }

    fn delete_sticker_from_set(
        &self,
        params: &DeleteStickerFromSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteStickerFromSet", Some(params))
    }

    fn set_sticker_emoji_list(
        &self,
        params: &SetStickerEmojiListParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerEmojiList", Some(params))
    }

    fn set_sticker_keywords(
        &self,
        params: &SetStickerKeywordsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerKeywords", Some(params))
    }

    fn set_sticker_mask_position(
        &self,
        params: &SetStickerMaskPositionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerMaskPosition", Some(params))
    }

    fn set_sticker_set_title(
        &self,
        params: &SetStickerSetTitleParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerSetTitle", Some(params))
    }

    fn set_sticker_set_thumbnail(
        &self,
        params: &SetStickerSetThumbnailParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "setStickerSetThumbnail";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        params: &SetCustomEmojiStickerSetThumbnailParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setCustomEmojiStickerSetThumbnail", Some(params))
    }

    fn delete_sticker_set(
        &self,
        params: &DeleteStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteStickerSet", Some(params))
    }

    fn send_invoice(
        &self,
        params: &SendInvoiceParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendInvoice", Some(params))
    }

    fn create_invoice_link(
        &self,
        params: &CreateInvoiceLinkParams,
    ) -> Result<MethodResponse<String>, Self::Error> {
        self.request("createInvoiceLink", Some(params))
    }

    fn answer_shipping_query(
        &self,
        params: &AnswerShippingQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerShippingQuery", Some(params))
    }

    fn answer_pre_checkout_query(
        &self,
        params: &AnswerPreCheckoutQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerPreCheckoutQuery", Some(params))
    }

    fn send_game(&self, params: &SendGameParams) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendGame", Some(params))
    }

    fn set_game_score(
        &self,
        params: &SetGameScoreParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("setGameScore", Some(params))
    }

    fn get_game_high_scores(
        &self,
        params: &GetGameHighScoresParams,
    ) -> Result<MethodResponse<Vec<GameHighScore>>, Self::Error> {
        self.request("getGameHighScores", Some(params))
    }

    fn set_my_default_administrator_rights(
        &self,
        params: &SetMyDefaultAdministratorRightsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyDefaultAdministratorRights", Some(params))
    }

    fn get_my_default_administrator_rights(
        &self,
        params: &GetMyDefaultAdministratorRightsParams,
    ) -> Result<MethodResponse<ChatAdministratorRights>, Self::Error> {
        self.request("getMyDefaultAdministratorRights", Some(params))
    }

    fn answer_web_app_query(
        &self,
        params: &AnswerWebAppQueryParams,
    ) -> Result<MethodResponse<SentWebAppMessage>, Self::Error> {
        self.request("answerWebAppQuery", Some(params))
    }

    fn set_chat_menu_button(
        &self,
        params: SetChatMenuButtonParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatMenuButton", Some(params))
    }

    fn get_chat_menu_button(
        &self,
        params: GetChatMenuButtonParams,
    ) -> Result<MethodResponse<MenuButton>, Self::Error> {
        self.request("getChatMenuButton", Some(params))
    }

    fn unpin_all_general_forum_topic_messages(
        &self,
        params: UnpinAllGeneralForumTopicMessagesParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unpinAllGeneralForumTopicMessages", Some(params))
    }

    fn request_without_body<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
    ) -> Result<T, Self::Error> {
        let params: Option<()> = None;

        self.request(method, params)
    }

    fn request_with_possible_form_data<
        T1: serde::ser::Serialize + std::fmt::Debug,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method_name: &str,
        params: &T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Self::Error> {
        if files.is_empty() {
            self.request(method_name, Some(params))
        } else {
            self.request_with_form_data(method_name, params, files)
        }
    }

    fn request_with_form_data<
        T1: serde::ser::Serialize + std::fmt::Debug,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method: &str,
        params: T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Self::Error>;

    fn request<T1: serde::ser::Serialize + std::fmt::Debug, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Self::Error>;
}
