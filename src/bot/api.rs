use crate::{
    bot::Bot,
    requests::{
        AnswerCallbackQuery, AnswerPreCheckoutQuery, AnswerShippingQuery,
        DeleteChatPhoto, DeleteChatStickerSet, EditMessageLiveLocation,
        ExportCharInviteLink, ForwardMessage, GetChat, GetChatAdministrators,
        GetChatMember, GetChatMembersCount, GetFile, GetMe, GetUpdates,
        KickChatMember, LeaveChat, PinChatMessage, PromoteChatMember,
        RestrictChatMember, SendAnimation, SendAudio, SendChatAction,
        SendContact, SendDocument, SendLocation, SendMediaGroup, SendMessage,
        SendPhoto, SendPoll, SendVenue, SendVideo, SendVideoNote, SendVoice,
        SetChatDescription, SetChatPermissions, SetChatPhoto,
        SetChatStickerSet, SetChatTitle, StopMessageLiveLocation,
        UnbanChatMember, UnpinChatMessage,
    },
    types::{ChatAction, ChatId, ChatPermissions, InputFile, InputMedia},
};

impl Bot {
    pub fn get_me(&self) -> GetMe {
        GetMe::new(self)
    }

    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates::new(self)
    }

    pub fn send_message<C, T>(&self, chat_id: C, text: T) -> SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessage::new(self, chat_id, text)
    }

    pub fn edit_message_live_location<Lt, Lg>(
        &self,
        latitude: Lt,
        longitude: Lg,
    ) -> EditMessageLiveLocation
    where
        Lt: Into<f64>,
        Lg: Into<f64>,
    {
        EditMessageLiveLocation::new(self, latitude, longitude)
    }

    pub fn forward_message<C, F, M>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: M,
    ) -> ForwardMessage
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
        M: Into<i32>,
    {
        ForwardMessage::new(self, chat_id, from_chat_id, message_id)
    }

    pub fn send_audio<C, A>(&self, chat_id: C, audio: A) -> SendAudio
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        SendAudio::new(self, chat_id, audio)
    }

    pub fn send_location<C, Lt, Lg>(
        &self,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
    ) -> SendLocation
    where
        C: Into<ChatId>,
        Lt: Into<f64>,
        Lg: Into<f64>,
    {
        SendLocation::new(self, chat_id, latitude, longitude)
    }

    pub fn send_media_group<C, M>(&self, chat_id: C, media: M) -> SendMediaGroup
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        SendMediaGroup::new(self, chat_id, media)
    }

    pub fn send_photo<C, P>(&self, chat_id: C, photo: P) -> SendPhoto
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        SendPhoto::new(self, chat_id, photo)
    }

    pub fn stop_message_live_location(&self) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new(self)
    }

    pub fn get_file<F>(&self, file_id: F) -> GetFile
    where
        F: Into<String>,
    {
        GetFile::new(self, file_id)
    }

    pub fn answer_pre_checkout_query<I, O>(
        &self,
        pre_checkout_query_id: I,
        ok: O,
    ) -> AnswerPreCheckoutQuery
    where
        I: Into<String>,
        O: Into<bool>,
    {
        AnswerPreCheckoutQuery::new(self, pre_checkout_query_id, ok)
    }

    pub fn get_chat<I>(&self, chat_id: I) -> GetChat
    where
        I: Into<ChatId>,
    {
        GetChat::new(self, chat_id)
    }

    pub fn answer_shipping_query<I, O>(
        &self,
        shipping_query_id: I,
        ok: O,
    ) -> AnswerShippingQuery
    where
        I: Into<String>,
        O: Into<bool>,
    {
        AnswerShippingQuery::new(self, shipping_query_id, ok)
    }

    pub fn kick_chat_member<C, U>(
        &self,
        chat_id: C,
        user_id: U,
    ) -> KickChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        KickChatMember::new(self, chat_id, user_id)
    }

    pub fn pin_chat_message<C, M>(
        &self,
        chat_id: C,
        message_id: M,
    ) -> PinChatMessage
    where
        C: Into<ChatId>,
        M: Into<i32>,
    {
        PinChatMessage::new(self, chat_id, message_id)
    }

    pub fn promote_chat_member<C, U>(
        &self,
        chat_id: C,
        user_id: U,
    ) -> PromoteChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        PromoteChatMember::new(self, chat_id, user_id)
    }

    pub fn restrict_chat_member<C, U, P>(
        &self,
        chat_id: C,
        user_id: U,
        permissions: P,
    ) -> RestrictChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
        P: Into<ChatPermissions>,
    {
        RestrictChatMember::new(self, chat_id, user_id, permissions)
    }

    pub fn send_chat_action<C, A>(
        &self,
        chat_id: C,
        action: A,
    ) -> SendChatAction
    where
        C: Into<ChatId>,
        A: Into<ChatAction>,
    {
        SendChatAction::new(self, chat_id, action)
    }

    pub fn send_contact<C, P, F>(
        &self,
        chat_id: C,
        phone_number: P,
        first_name: F,
    ) -> SendContact
    where
        C: Into<ChatId>,
        P: Into<String>,
        F: Into<String>,
    {
        SendContact::new(self, chat_id, phone_number, first_name)
    }

    pub fn send_poll<C, Q, O>(
        &self,
        chat_id: C,
        question: Q,
        options: O,
    ) -> SendPoll
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: Into<Vec<String>>,
    {
        SendPoll::new(self, chat_id, question, options)
    }

    pub fn send_venue<C, Lt, Lg, T, A>(
        &self,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
        title: T,
        address: A,
    ) -> SendVenue
    where
        C: Into<ChatId>,
        Lt: Into<f64>,
        Lg: Into<f64>,
        T: Into<String>,
        A: Into<String>,
    {
        SendVenue::new(self, chat_id, latitude, longitude, title, address)
    }

    pub fn send_video_note<C, V>(
        &self,
        chat_id: C,
        video_note: V,
    ) -> SendVideoNote
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        SendVideoNote::new(self, chat_id, video_note)
    }

    pub fn send_voice<C, V>(&self, chat_id: C, voice: V) -> SendVoice
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        SendVoice::new(self, chat_id, voice)
    }

    pub fn send_chat_description<C>(&self, chat_id: C) -> SetChatDescription
    where
        C: Into<ChatId>,
    {
        SetChatDescription::new(self, chat_id)
    }

    pub fn unban_chat_member<C, U>(
        &self,
        chat_id: C,
        user_id: U,
    ) -> UnbanChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        UnbanChatMember::new(self, chat_id, user_id)
    }

    pub fn unpin_chat_message<C>(&self, chat_id: C) -> UnpinChatMessage
    where
        C: Into<ChatId>,
    {
        UnpinChatMessage::new(self, chat_id)
    }

    pub fn answer_callback_query<S>(
        &self,
        callback_query_id: S,
    ) -> AnswerCallbackQuery
    where
        S: Into<String>,
    {
        AnswerCallbackQuery::new(self, callback_query_id)
    }

    pub fn delete_chat_sticker_set<C>(&self, chat_id: C) -> DeleteChatStickerSet
    where
        C: Into<ChatId>,
    {
        DeleteChatStickerSet::new(self, chat_id)
    }

    pub fn set_chat_sticker_set<C, S>(
        &self,
        chat_id: C,
        sticker_set_name: S,
    ) -> SetChatStickerSet
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        SetChatStickerSet::new(self, chat_id, sticker_set_name)
    }

    pub fn get_chat_member<C, I>(&self, chat_id: C, user_id: I) -> GetChatMember
    where
        C: Into<ChatId>,
        I: Into<i32>,
    {
        GetChatMember::new(self, chat_id, user_id)
    }

    pub fn get_chat_administrators<C, I>(
        &self,
        chat_id: C,
    ) -> GetChatAdministrators
    where
        C: Into<ChatId>,
    {
        GetChatAdministrators::new(self, chat_id)
    }

    pub fn get_chat_members_count<C>(&self, chat_id: C) -> GetChatMembersCount
    where
        C: Into<ChatId>,
    {
        GetChatMembersCount::new(self, chat_id)
    }

    pub fn send_video<C, V>(&self, chat_id: C, video: V) -> SendVideo
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        SendVideo::new(self, chat_id, video)
    }

    pub fn send_document<C, D>(&self, chat_id: C, document: D) -> SendDocument
    where
        C: Into<ChatId>,
        D: Into<InputFile>,
    {
        SendDocument::new(self, chat_id, document)
    }

    pub fn send_animation<C, S>(
        &self,
        chat_id: C,
        animation: S,
    ) -> SendAnimation
    where
        C: Into<ChatId>,
        S: Into<InputFile>,
    {
        SendAnimation::new(self, chat_id, animation)
    }

    pub fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> SetChatTitle
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SetChatTitle::new(self, chat_id, title)
    }

    pub fn delete_chat_photo<C>(&self, chat_id: C) -> DeleteChatPhoto
    where
        C: Into<ChatId>,
    {
        DeleteChatPhoto::new(self, chat_id)
    }

    pub fn leave_chat<C>(&self, chat_id: C) -> LeaveChat
    where
        C: Into<ChatId>,
    {
        LeaveChat::new(self, chat_id)
    }

    pub fn set_chat_photo<C, P>(&self, chat_id: C, photo: P) -> SetChatPhoto
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        SetChatPhoto::new(self, chat_id, photo)
    }

    pub fn export_chat_invite_link<C>(&self, chat_id: C) -> ExportCharInviteLink
    where
        C: Into<ChatId>,
    {
        ExportCharInviteLink::new(self, chat_id)
    }

    pub fn set_chat_permissions<C, CP>(
        &self,
        chat_id: C,
        permissions: CP,
    ) -> SetChatPermissions
    where
        C: Into<ChatId>,
        CP: Into<ChatPermissions>,
    {
        SetChatPermissions::new(self, chat_id, permissions)
    }
}
