use serde::Serialize;
use crate::reader::BinaryReader;
use anyhow::Result;

// Game event types (for 0xF7B0 messages)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GameEventType {
    Allegiance_AllegianceUpdateAborted = 0x0003,
    Communication_PopUpString = 0x0004,
    Login_PlayerDescription = 0x0013,
    Allegiance_AllegianceUpdate = 0x0020,
    Social_FriendsUpdate = 0x0021,
    Item_ServerSaysContainId = 0x0022,
    Item_WearItem = 0x0023,
    Social_CharacterTitleTable = 0x0029,
    Social_AddOrSetCharacterTitle = 0x002B,
    Item_StopViewingObjectContents = 0x0052,
    Vendor_VendorInfo = 0x0062,
    Character_StartBarber = 0x0075,
    Fellowship_Quit = 0x00A3,
    Fellowship_Dismiss = 0x00A4,
    Writing_BookOpen = 0x00B4,
    Writing_BookAddPageResponse = 0x00B6,
    Writing_BookDeletePageResponse = 0x00B7,
    Writing_BookPageDataResponse = 0x00B8,
    Item_GetInscriptionResponse = 0x00C3,
    Item_SetAppraiseInfo = 0x00C9,
    Character_CharacterOptionsEvent = 0x00F7,
    Communication_ChannelBroadcast = 0x0147,
    Communication_ChannelList = 0x0148,
    Communication_ChannelIndex = 0x0149,
    Item_OnViewContents = 0x0196,
    Item_ServerSaysMoveItem = 0x019A,
    Combat_HandleAttackDoneEvent = 0x01A7,
    Magic_RemoveSpell = 0x01A8,
    Combat_HandleVictimNotificationEventSelf = 0x01AC,
    Combat_HandleVictimNotificationEventOther = 0x01AD,
    Combat_HandleAttackerNotificationEvent = 0x01B1,
    Combat_HandleDefenderNotificationEvent = 0x01B2,
    Combat_HandleEvasionAttackerNotificationEvent = 0x01B3,
    Combat_HandleEvasionDefenderNotificationEvent = 0x01B4,
    Combat_HandleCommenceAttackEvent = 0x01B8,
    Combat_QueryHealthResponse = 0x01C0,
    Character_QueryAgeResponse = 0x01C3,
    Item_UseDone = 0x01C7,
    Allegiance_AllegianceUpdateDone = 0x01C8,
    Fellowship_FellowUpdateDone = 0x01C9,
    Fellowship_FellowStatsDone = 0x01CA,
    Item_AppraiseDone = 0x01CB,
    Character_ReturnPing = 0x01EA,
    Communication_SetSquelchDB = 0x01F4,
    Trade_RegisterTrade = 0x01FD,
    Trade_OpenTrade = 0x01FE,
    Trade_CloseTrade = 0x01FF,
    Trade_AddToTrade = 0x0200,
    Trade_RemoveFromTrade = 0x0201,
    Trade_AcceptTrade = 0x0202,
    Trade_DeclineTrade = 0x0203,
    Trade_ResetTrade = 0x0205,
    Trade_TradeFailure = 0x0207,
    Trade_ClearTradeAcceptance = 0x0208,
    House_HouseProfile = 0x021D,
    House_HouseData = 0x0225,
    House_HouseStatus = 0x0226,
    House_UpdateRentTime = 0x0227,
    House_UpdateRentPayment = 0x0228,
    House_UpdateRestrictions = 0x0248,
    House_UpdateHAR = 0x0257,
    House_HouseTransaction = 0x0259,
    Item_QueryItemManaResponse = 0x0264,
    House_AvailableHouses = 0x0271,
    Character_ConfirmationRequest = 0x0274,
    Character_ConfirmationDone = 0x0276,
    Allegiance_AllegianceLoginNotificationEvent = 0x027A,
    Allegiance_AllegianceInfoResponseEvent = 0x027C,
    Game_JoinGameResponse = 0x0281,
    Game_StartGame = 0x0282,
    Game_MoveResponse = 0x0283,
    Game_OpponentTurn = 0x0284,
    Game_OpponentStalemateState = 0x0285,
    Communication_WeenieError = 0x028A,
    Communication_WeenieErrorWithString = 0x028B,
    Game_GameOver = 0x028C,
    Communication_ChatRoomTracker = 0x0295,
    Admin_QueryPluginList = 0x02AE,
    Admin_QueryPlugin = 0x02B1,
    Admin_QueryPluginResponse2 = 0x02B3,
    Inventory_SalvageOperationsResultData = 0x02B4,
    Communication_HearDirectSpeech = 0x02BD,
    Fellowship_FullUpdate = 0x02BE,
    Fellowship_Disband = 0x02BF,
    Fellowship_UpdateFellow = 0x02C0,
    Magic_UpdateSpell = 0x02C1,
    Magic_UpdateEnchantment = 0x02C2,
    Magic_RemoveEnchantment = 0x02C3,
    Magic_UpdateMultipleEnchantments = 0x02C4,
    Magic_RemoveMultipleEnchantments = 0x02C5,
    Magic_PurgeEnchantments = 0x02C6,
    Magic_DispelEnchantment = 0x02C7,
    Magic_DispelMultipleEnchantments = 0x02C8,
    Misc_PortalStormBrewing = 0x02C9,
    Misc_PortalStormImminent = 0x02CA,
    Misc_PortalStorm = 0x02CB,
    Misc_PortalStormSubsided = 0x02CC,
    Communication_TransientString = 0x02EB,
    Magic_PurgeBadEnchantments = 0x0312,
    Social_SendClientContractTrackerTable = 0x0314,
    Social_SendClientContractTracker = 0x0315,
    Unknown = 0xFFFFFFFF,
}

impl GameEventType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0x0003 => GameEventType::Allegiance_AllegianceUpdateAborted,
            0x0004 => GameEventType::Communication_PopUpString,
            0x0013 => GameEventType::Login_PlayerDescription,
            0x0020 => GameEventType::Allegiance_AllegianceUpdate,
            0x0021 => GameEventType::Social_FriendsUpdate,
            0x0022 => GameEventType::Item_ServerSaysContainId,
            0x0023 => GameEventType::Item_WearItem,
            0x0029 => GameEventType::Social_CharacterTitleTable,
            0x002B => GameEventType::Social_AddOrSetCharacterTitle,
            0x0052 => GameEventType::Item_StopViewingObjectContents,
            0x0062 => GameEventType::Vendor_VendorInfo,
            0x0075 => GameEventType::Character_StartBarber,
            0x00A3 => GameEventType::Fellowship_Quit,
            0x00A4 => GameEventType::Fellowship_Dismiss,
            0x00B4 => GameEventType::Writing_BookOpen,
            0x00B6 => GameEventType::Writing_BookAddPageResponse,
            0x00B7 => GameEventType::Writing_BookDeletePageResponse,
            0x00B8 => GameEventType::Writing_BookPageDataResponse,
            0x00C3 => GameEventType::Item_GetInscriptionResponse,
            0x00C9 => GameEventType::Item_SetAppraiseInfo,
            0x00F7 => GameEventType::Character_CharacterOptionsEvent,
            0x0147 => GameEventType::Communication_ChannelBroadcast,
            0x0148 => GameEventType::Communication_ChannelList,
            0x0149 => GameEventType::Communication_ChannelIndex,
            0x0196 => GameEventType::Item_OnViewContents,
            0x019A => GameEventType::Item_ServerSaysMoveItem,
            0x01A7 => GameEventType::Combat_HandleAttackDoneEvent,
            0x01A8 => GameEventType::Magic_RemoveSpell,
            0x01AC => GameEventType::Combat_HandleVictimNotificationEventSelf,
            0x01AD => GameEventType::Combat_HandleVictimNotificationEventOther,
            0x01B1 => GameEventType::Combat_HandleAttackerNotificationEvent,
            0x01B2 => GameEventType::Combat_HandleDefenderNotificationEvent,
            0x01B3 => GameEventType::Combat_HandleEvasionAttackerNotificationEvent,
            0x01B4 => GameEventType::Combat_HandleEvasionDefenderNotificationEvent,
            0x01B8 => GameEventType::Combat_HandleCommenceAttackEvent,
            0x01C0 => GameEventType::Combat_QueryHealthResponse,
            0x01C3 => GameEventType::Character_QueryAgeResponse,
            0x01C7 => GameEventType::Item_UseDone,
            0x01C8 => GameEventType::Allegiance_AllegianceUpdateDone,
            0x01C9 => GameEventType::Fellowship_FellowUpdateDone,
            0x01CA => GameEventType::Fellowship_FellowStatsDone,
            0x01CB => GameEventType::Item_AppraiseDone,
            0x01EA => GameEventType::Character_ReturnPing,
            0x01F4 => GameEventType::Communication_SetSquelchDB,
            0x01FD => GameEventType::Trade_RegisterTrade,
            0x01FE => GameEventType::Trade_OpenTrade,
            0x01FF => GameEventType::Trade_CloseTrade,
            0x0200 => GameEventType::Trade_AddToTrade,
            0x0201 => GameEventType::Trade_RemoveFromTrade,
            0x0202 => GameEventType::Trade_AcceptTrade,
            0x0203 => GameEventType::Trade_DeclineTrade,
            0x0205 => GameEventType::Trade_ResetTrade,
            0x0207 => GameEventType::Trade_TradeFailure,
            0x0208 => GameEventType::Trade_ClearTradeAcceptance,
            0x021D => GameEventType::House_HouseProfile,
            0x0225 => GameEventType::House_HouseData,
            0x0226 => GameEventType::House_HouseStatus,
            0x0227 => GameEventType::House_UpdateRentTime,
            0x0228 => GameEventType::House_UpdateRentPayment,
            0x0248 => GameEventType::House_UpdateRestrictions,
            0x0257 => GameEventType::House_UpdateHAR,
            0x0259 => GameEventType::House_HouseTransaction,
            0x0264 => GameEventType::Item_QueryItemManaResponse,
            0x0271 => GameEventType::House_AvailableHouses,
            0x0274 => GameEventType::Character_ConfirmationRequest,
            0x0276 => GameEventType::Character_ConfirmationDone,
            0x027A => GameEventType::Allegiance_AllegianceLoginNotificationEvent,
            0x027C => GameEventType::Allegiance_AllegianceInfoResponseEvent,
            0x0281 => GameEventType::Game_JoinGameResponse,
            0x0282 => GameEventType::Game_StartGame,
            0x0283 => GameEventType::Game_MoveResponse,
            0x0284 => GameEventType::Game_OpponentTurn,
            0x0285 => GameEventType::Game_OpponentStalemateState,
            0x028A => GameEventType::Communication_WeenieError,
            0x028B => GameEventType::Communication_WeenieErrorWithString,
            0x028C => GameEventType::Game_GameOver,
            0x0295 => GameEventType::Communication_ChatRoomTracker,
            0x02AE => GameEventType::Admin_QueryPluginList,
            0x02B1 => GameEventType::Admin_QueryPlugin,
            0x02B3 => GameEventType::Admin_QueryPluginResponse2,
            0x02B4 => GameEventType::Inventory_SalvageOperationsResultData,
            0x02BD => GameEventType::Communication_HearDirectSpeech,
            0x02BE => GameEventType::Fellowship_FullUpdate,
            0x02BF => GameEventType::Fellowship_Disband,
            0x02C0 => GameEventType::Fellowship_UpdateFellow,
            0x02C1 => GameEventType::Magic_UpdateSpell,
            0x02C2 => GameEventType::Magic_UpdateEnchantment,
            0x02C3 => GameEventType::Magic_RemoveEnchantment,
            0x02C4 => GameEventType::Magic_UpdateMultipleEnchantments,
            0x02C5 => GameEventType::Magic_RemoveMultipleEnchantments,
            0x02C6 => GameEventType::Magic_PurgeEnchantments,
            0x02C7 => GameEventType::Magic_DispelEnchantment,
            0x02C8 => GameEventType::Magic_DispelMultipleEnchantments,
            0x02C9 => GameEventType::Misc_PortalStormBrewing,
            0x02CA => GameEventType::Misc_PortalStormImminent,
            0x02CB => GameEventType::Misc_PortalStorm,
            0x02CC => GameEventType::Misc_PortalStormSubsided,
            0x02EB => GameEventType::Communication_TransientString,
            0x0312 => GameEventType::Magic_PurgeBadEnchantments,
            0x0314 => GameEventType::Social_SendClientContractTrackerTable,
            0x0315 => GameEventType::Social_SendClientContractTracker,
            _ => GameEventType::Unknown,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            GameEventType::Allegiance_AllegianceUpdateAborted => "Allegiance_AllegianceUpdateAborted",
            GameEventType::Communication_PopUpString => "Communication_PopUpString",
            GameEventType::Login_PlayerDescription => "Login_PlayerDescription",
            GameEventType::Allegiance_AllegianceUpdate => "Allegiance_AllegianceUpdate",
            GameEventType::Social_FriendsUpdate => "Social_FriendsUpdate",
            GameEventType::Item_ServerSaysContainId => "Item_ServerSaysContainId",
            GameEventType::Item_WearItem => "Item_WearItem",
            GameEventType::Social_CharacterTitleTable => "Social_CharacterTitleTable",
            GameEventType::Social_AddOrSetCharacterTitle => "Social_AddOrSetCharacterTitle",
            GameEventType::Item_StopViewingObjectContents => "Item_StopViewingObjectContents",
            GameEventType::Vendor_VendorInfo => "Vendor_VendorInfo",
            GameEventType::Character_StartBarber => "Character_StartBarber",
            GameEventType::Fellowship_Quit => "Fellowship_Quit",
            GameEventType::Fellowship_Dismiss => "Fellowship_Dismiss",
            GameEventType::Writing_BookOpen => "Writing_BookOpen",
            GameEventType::Writing_BookAddPageResponse => "Writing_BookAddPageResponse",
            GameEventType::Writing_BookDeletePageResponse => "Writing_BookDeletePageResponse",
            GameEventType::Writing_BookPageDataResponse => "Writing_BookPageDataResponse",
            GameEventType::Item_GetInscriptionResponse => "Item_GetInscriptionResponse",
            GameEventType::Item_SetAppraiseInfo => "Item_SetAppraiseInfo",
            GameEventType::Character_CharacterOptionsEvent => "Character_CharacterOptionsEvent",
            GameEventType::Communication_ChannelBroadcast => "Communication_ChannelBroadcast",
            GameEventType::Communication_ChannelList => "Communication_ChannelList",
            GameEventType::Communication_ChannelIndex => "Communication_ChannelIndex",
            GameEventType::Item_OnViewContents => "Item_OnViewContents",
            GameEventType::Item_ServerSaysMoveItem => "Item_ServerSaysMoveItem",
            GameEventType::Combat_HandleAttackDoneEvent => "Combat_HandleAttackDoneEvent",
            GameEventType::Magic_RemoveSpell => "Magic_RemoveSpell",
            GameEventType::Combat_HandleVictimNotificationEventSelf => "Combat_HandleVictimNotificationEventSelf",
            GameEventType::Combat_HandleVictimNotificationEventOther => "Combat_HandleVictimNotificationEventOther",
            GameEventType::Combat_HandleAttackerNotificationEvent => "Combat_HandleAttackerNotificationEvent",
            GameEventType::Combat_HandleDefenderNotificationEvent => "Combat_HandleDefenderNotificationEvent",
            GameEventType::Combat_HandleEvasionAttackerNotificationEvent => "Combat_HandleEvasionAttackerNotificationEvent",
            GameEventType::Combat_HandleEvasionDefenderNotificationEvent => "Combat_HandleEvasionDefenderNotificationEvent",
            GameEventType::Combat_HandleCommenceAttackEvent => "Combat_HandleCommenceAttackEvent",
            GameEventType::Combat_QueryHealthResponse => "Combat_QueryHealthResponse",
            GameEventType::Character_QueryAgeResponse => "Character_QueryAgeResponse",
            GameEventType::Item_UseDone => "Item_UseDone",
            GameEventType::Allegiance_AllegianceUpdateDone => "Allegiance_AllegianceUpdateDone",
            GameEventType::Fellowship_FellowUpdateDone => "Fellowship_FellowUpdateDone",
            GameEventType::Fellowship_FellowStatsDone => "Fellowship_FellowStatsDone",
            GameEventType::Item_AppraiseDone => "Item_AppraiseDone",
            GameEventType::Character_ReturnPing => "Character_ReturnPing",
            GameEventType::Communication_SetSquelchDB => "Communication_SetSquelchDB",
            GameEventType::Trade_RegisterTrade => "Trade_RegisterTrade",
            GameEventType::Trade_OpenTrade => "Trade_OpenTrade",
            GameEventType::Trade_CloseTrade => "Trade_CloseTrade",
            GameEventType::Trade_AddToTrade => "Trade_AddToTrade",
            GameEventType::Trade_RemoveFromTrade => "Trade_RemoveFromTrade",
            GameEventType::Trade_AcceptTrade => "Trade_AcceptTrade",
            GameEventType::Trade_DeclineTrade => "Trade_DeclineTrade",
            GameEventType::Trade_ResetTrade => "Trade_ResetTrade",
            GameEventType::Trade_TradeFailure => "Trade_TradeFailure",
            GameEventType::Trade_ClearTradeAcceptance => "Trade_ClearTradeAcceptance",
            GameEventType::House_HouseProfile => "House_HouseProfile",
            GameEventType::House_HouseData => "House_HouseData",
            GameEventType::House_HouseStatus => "House_HouseStatus",
            GameEventType::House_UpdateRentTime => "House_UpdateRentTime",
            GameEventType::House_UpdateRentPayment => "House_UpdateRentPayment",
            GameEventType::House_UpdateRestrictions => "House_UpdateRestrictions",
            GameEventType::House_UpdateHAR => "House_UpdateHAR",
            GameEventType::House_HouseTransaction => "House_HouseTransaction",
            GameEventType::Item_QueryItemManaResponse => "Item_QueryItemManaResponse",
            GameEventType::House_AvailableHouses => "House_AvailableHouses",
            GameEventType::Character_ConfirmationRequest => "Character_ConfirmationRequest",
            GameEventType::Character_ConfirmationDone => "Character_ConfirmationDone",
            GameEventType::Allegiance_AllegianceLoginNotificationEvent => "Allegiance_AllegianceLoginNotificationEvent",
            GameEventType::Allegiance_AllegianceInfoResponseEvent => "Allegiance_AllegianceInfoResponseEvent",
            GameEventType::Game_JoinGameResponse => "Game_JoinGameResponse",
            GameEventType::Game_StartGame => "Game_StartGame",
            GameEventType::Game_MoveResponse => "Game_MoveResponse",
            GameEventType::Game_OpponentTurn => "Game_OpponentTurn",
            GameEventType::Game_OpponentStalemateState => "Game_OpponentStalemateState",
            GameEventType::Communication_WeenieError => "Communication_WeenieError",
            GameEventType::Communication_WeenieErrorWithString => "Communication_WeenieErrorWithString",
            GameEventType::Game_GameOver => "Game_GameOver",
            GameEventType::Communication_ChatRoomTracker => "Communication_ChatRoomTracker",
            GameEventType::Admin_QueryPluginList => "Admin_QueryPluginList",
            GameEventType::Admin_QueryPlugin => "Admin_QueryPlugin",
            GameEventType::Admin_QueryPluginResponse2 => "Admin_QueryPluginResponse2",
            GameEventType::Inventory_SalvageOperationsResultData => "Inventory_SalvageOperationsResultData",
            GameEventType::Communication_HearDirectSpeech => "Communication_HearDirectSpeech",
            GameEventType::Fellowship_FullUpdate => "Fellowship_FullUpdate",
            GameEventType::Fellowship_Disband => "Fellowship_Disband",
            GameEventType::Fellowship_UpdateFellow => "Fellowship_UpdateFellow",
            GameEventType::Magic_UpdateSpell => "Magic_UpdateSpell",
            GameEventType::Magic_UpdateEnchantment => "Magic_UpdateEnchantment",
            GameEventType::Magic_RemoveEnchantment => "Magic_RemoveEnchantment",
            GameEventType::Magic_UpdateMultipleEnchantments => "Magic_UpdateMultipleEnchantments",
            GameEventType::Magic_RemoveMultipleEnchantments => "Magic_RemoveMultipleEnchantments",
            GameEventType::Magic_PurgeEnchantments => "Magic_PurgeEnchantments",
            GameEventType::Magic_DispelEnchantment => "Magic_DispelEnchantment",
            GameEventType::Magic_DispelMultipleEnchantments => "Magic_DispelMultipleEnchantments",
            GameEventType::Misc_PortalStormBrewing => "Misc_PortalStormBrewing",
            GameEventType::Misc_PortalStormImminent => "Misc_PortalStormImminent",
            GameEventType::Misc_PortalStorm => "Misc_PortalStorm",
            GameEventType::Misc_PortalStormSubsided => "Misc_PortalStormSubsided",
            GameEventType::Communication_TransientString => "Communication_TransientString",
            GameEventType::Magic_PurgeBadEnchantments => "Magic_PurgeBadEnchantments",
            GameEventType::Social_SendClientContractTrackerTable => "Social_SendClientContractTrackerTable",
            GameEventType::Social_SendClientContractTracker => "Social_SendClientContractTracker",
            GameEventType::Unknown => "Unknown",
        }
    }
}

pub fn parse_game_event(
    reader: &mut BinaryReader,
    object_id: u32,
    sequence: u32,
    event_type: u32,
) -> Result<(String, serde_json::Value)> {
    let evt_type = GameEventType::from_u32(event_type);

    match evt_type {
        GameEventType::Character_CharacterOptionsEvent => {
            let msg = CharacterCharacterOptionsEvent::read(reader, object_id, sequence)?;
            Ok(("Character_CharacterOptionsEvent".to_string(), serde_json::to_value(&msg)?))
        }
        GameEventType::Item_SetAppraiseInfo => {
            let msg = ItemSetAppraiseInfo::read(reader, object_id, sequence)?;
            Ok(("Item_SetAppraiseInfo".to_string(), serde_json::to_value(&msg)?))
        }
        GameEventType::Item_ServerSaysContainId => {
            let msg = ItemServerSaysContainId::read(reader, object_id, sequence)?;
            Ok(("Item_ServerSaysContainId".to_string(), serde_json::to_value(&msg)?))
        }
        GameEventType::Item_WearItem => {
            let msg = ItemWearItem::read(reader, object_id, sequence)?;
            Ok(("Item_WearItem".to_string(), serde_json::to_value(&msg)?))
        }
        GameEventType::Magic_UpdateEnchantment => {
            let msg = MagicUpdateEnchantment::read(reader, object_id, sequence)?;
            Ok(("Magic_UpdateEnchantment".to_string(), serde_json::to_value(&msg)?))
        }
        GameEventType::Magic_DispelEnchantment => {
            let msg = MagicDispelEnchantment::read(reader, object_id, sequence)?;
            Ok(("Magic_DispelEnchantment".to_string(), serde_json::to_value(&msg)?))
        }
        _ => {
            // Use the enum name if known, otherwise format as hex
            let type_name = if evt_type != GameEventType::Unknown {
                evt_type.name().to_string()
            } else {
                format!("GameEvent_{:04X}", event_type)
            };
            let remaining = reader.remaining();
            let raw_data = if remaining > 0 {
                reader.read_bytes(remaining)?
            } else {
                vec![]
            };
            Ok((
                type_name,
                serde_json::json!({
                    "OrderedObjectId": object_id,
                    "OrderedSequence": sequence,
                    "EventType": evt_type.name(),
                    "OpCode": 0xF7B0u32,
                    "MessageType": "Ordered_GameEvent",
                    "MessageDirection": "ServerToClient",
                    "RawData": hex::encode(&raw_data),
                })
            ))
        }
    }
}

// Simple S2C messages

#[derive(Debug, Clone, Serialize)]
pub struct QualitiesPrivateUpdateInt {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: i32,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl QualitiesPrivateUpdateInt {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let sequence = reader.read_u8()?;
        let key_raw = reader.read_u32()?;
        let value = reader.read_i32()?;

        Ok(Self {
            sequence,
            key: property_int_name(key_raw),
            value,
            opcode: 0x02CD,
            message_type: "Qualities_PrivateUpdateInt".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QualitiesPrivateUpdateAttribute2ndLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: u32,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl QualitiesPrivateUpdateAttribute2ndLevel {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let sequence = reader.read_u8()?;
        let key_raw = reader.read_u32()?;
        let value = reader.read_u32()?;

        Ok(Self {
            sequence,
            key: vital_name(key_raw),
            value,
            opcode: 0x02E9,
            message_type: "Qualities_PrivateUpdateAttribute2ndLevel".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QualitiesUpdateInt {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: i32,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl QualitiesUpdateInt {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let sequence = reader.read_u8()?;
        let object_id = reader.read_u32()?;
        let key_raw = reader.read_u32()?;
        let value = reader.read_i32()?;

        Ok(Self {
            sequence,
            object_id,
            key: property_int_name(key_raw),
            value,
            opcode: 0x02CE,
            message_type: "Qualities_UpdateInt".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QualitiesUpdateInstanceId {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: u32,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl QualitiesUpdateInstanceId {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let sequence = reader.read_u8()?;
        let object_id = reader.read_u32()?;
        let key_raw = reader.read_u32()?;
        let value = reader.read_u32()?;

        Ok(Self {
            sequence,
            object_id,
            key: property_instance_id_name(key_raw),
            value,
            opcode: 0x02DA,
            message_type: "Qualities_UpdateInstanceId".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MovementSetObjectMovement {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "MovementData")]
    pub movement_data: MovementData,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl MovementSetObjectMovement {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let object_id = reader.read_u32()?;
        let object_instance_sequence = reader.read_u16()?;
        let movement_data = MovementData::read(reader)?;

        Ok(Self {
            object_id,
            object_instance_sequence,
            movement_data,
            opcode: 0xF74C,
            message_type: "Movement_SetObjectMovement".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MovementData {
    #[serde(rename = "ObjectMovementSequence")]
    pub object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    pub autonomous: u8,
    #[serde(rename = "MovementType")]
    pub movement_type: String,
}

impl MovementData {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let object_movement_sequence = reader.read_u16()?;
        let object_server_control_sequence = reader.read_u16()?;
        let autonomous = reader.read_u8()?;
        let movement_type_raw = reader.read_u8()?;

        let movement_type = match movement_type_raw {
            0 => "Invalid",
            1 => "General",
            2 => "RawCommand",
            3 => "InterpertedMotionState",
            4 => "StopCompletely",
            5 => "MoveToObject",
            6 => "MoveToPosition",
            7 => "TurnToObject",
            8 => "TurnToHeading",
            9 => "Jump",
            _ => "Unknown",
        }.to_string();

        // Skip remaining movement data for now (complex parsing)
        // This is a simplified version

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            movement_type,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InventoryPickupEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectPositionSequence")]
    pub object_position_sequence: u16,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl InventoryPickupEvent {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let object_id = reader.read_u32()?;
        let object_instance_sequence = reader.read_u16()?;
        let object_position_sequence = reader.read_u16()?;

        Ok(Self {
            object_id,
            object_instance_sequence,
            object_position_sequence,
            opcode: 0xF74A,
            message_type: "Inventory_PickupEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EffectsSoundEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "SoundType")]
    pub sound_type: u32,
    #[serde(rename = "Volume")]
    pub volume: f32,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl EffectsSoundEvent {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let object_id = reader.read_u32()?;
        let sound_type = reader.read_u32()?;
        let volume = reader.read_f32()?;

        Ok(Self {
            object_id,
            sound_type,
            volume,
            opcode: 0xF750,
            message_type: "Effects_SoundEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EffectsPlayScriptType {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "ScriptType")]
    pub script_type: u32,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl EffectsPlayScriptType {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let object_id = reader.read_u32()?;
        let script_type = reader.read_u32()?;
        let speed = reader.read_f32()?;

        Ok(Self {
            object_id,
            script_type,
            speed,
            opcode: 0xF755,
            message_type: "Effects_PlayScriptType".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CommunicationTextboxString {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Type")]
    pub chat_type: String,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl CommunicationTextboxString {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let text = reader.read_string16l()?;
        let chat_type_raw = reader.read_u32()?;

        let chat_type = match chat_type_raw {
            0 => "Default",
            1 => "System",
            5 => "Magic",
            _ => "Unknown",
        }.to_string();

        Ok(Self {
            text,
            chat_type,
            opcode: 0xF7E0,
            message_type: "Communication_TextboxString".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemObjDescEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "InstanceSequence")]
    pub instance_sequence: u16,
    #[serde(rename = "VisualDescSequence")]
    pub visual_desc_sequence: u16,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl ItemObjDescEvent {
    pub fn read(reader: &mut BinaryReader) -> Result<Self> {
        let object_id = reader.read_u32()?;
        // Skip object description for now (complex)
        let remaining = reader.remaining();
        if remaining >= 4 {
            reader.read_bytes(remaining - 4)?;
        }
        let instance_sequence = reader.read_u16().unwrap_or(0);
        let visual_desc_sequence = reader.read_u16().unwrap_or(0);

        Ok(Self {
            object_id,
            instance_sequence,
            visual_desc_sequence,
            opcode: 0xF625,
            message_type: "Item_ObjDescEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

// Game Events

#[derive(Debug, Clone, Serialize)]
pub struct CharacterCharacterOptionsEvent {
    #[serde(rename = "OrderedObjectId")]
    pub ordered_object_id: u32,
    #[serde(rename = "OrderedSequence")]
    pub ordered_sequence: u32,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl CharacterCharacterOptionsEvent {
    pub fn read(_reader: &mut BinaryReader, ordered_object_id: u32, ordered_sequence: u32) -> Result<Self> {
        // Complex options data - skip for now
        Ok(Self {
            ordered_object_id,
            ordered_sequence,
            event_type: "Character_CharacterOptionsEvent".to_string(),
            opcode: 0xF7B0,
            message_type: "Ordered_GameEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemWearItem {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "Location")]
    pub location: u32,
    #[serde(rename = "OrderedObjectId")]
    pub ordered_object_id: u32,
    #[serde(rename = "OrderedSequence")]
    pub ordered_sequence: u32,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl ItemWearItem {
    pub fn read(reader: &mut BinaryReader, ordered_object_id: u32, ordered_sequence: u32) -> Result<Self> {
        let object_id = reader.read_u32()?;
        let location = reader.read_u32()?;

        Ok(Self {
            object_id,
            location,
            ordered_object_id,
            ordered_sequence,
            event_type: "Item_WearItem".to_string(),
            opcode: 0xF7B0,
            message_type: "Ordered_GameEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemSetAppraiseInfo {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "OrderedObjectId")]
    pub ordered_object_id: u32,
    #[serde(rename = "OrderedSequence")]
    pub ordered_sequence: u32,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl ItemSetAppraiseInfo {
    pub fn read(reader: &mut BinaryReader, ordered_object_id: u32, ordered_sequence: u32) -> Result<Self> {
        let object_id = reader.read_u32()?;
        let flags = reader.read_u32()?;
        let success = reader.read_bool()?;

        // Skip parsing complex property dictionaries for now
        // This would require full implementation of packable hash tables

        Ok(Self {
            object_id,
            flags,
            success,
            ordered_object_id,
            ordered_sequence,
            event_type: "Item_SetAppraiseInfo".to_string(),
            opcode: 0xF7B0,
            message_type: "Ordered_GameEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LayeredSpellId {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Layer")]
    pub layer: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct MagicDispelEnchantment {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
    #[serde(rename = "OrderedObjectId")]
    pub ordered_object_id: u32,
    #[serde(rename = "OrderedSequence")]
    pub ordered_sequence: u32,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl MagicDispelEnchantment {
    pub fn read(reader: &mut BinaryReader, ordered_object_id: u32, ordered_sequence: u32) -> Result<Self> {
        let spell_id = reader.read_u16()?;
        let layer = reader.read_u16()?;

        Ok(Self {
            spell_id: LayeredSpellId {
                id: spell_id as u32,
                layer,
            },
            ordered_object_id,
            ordered_sequence,
            event_type: "Magic_DispelEnchantment".to_string(),
            opcode: 0xF7B0,
            message_type: "Ordered_GameEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MagicUpdateEnchantment {
    #[serde(rename = "OrderedObjectId")]
    pub ordered_object_id: u32,
    #[serde(rename = "OrderedSequence")]
    pub ordered_sequence: u32,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl MagicUpdateEnchantment {
    pub fn read(_reader: &mut BinaryReader, ordered_object_id: u32, ordered_sequence: u32) -> Result<Self> {
        // Complex enchantment data - skip for now
        Ok(Self {
            ordered_object_id,
            ordered_sequence,
            event_type: "Magic_UpdateEnchantment".to_string(),
            opcode: 0xF7B0,
            message_type: "Ordered_GameEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemServerSaysContainId {
    #[serde(rename = "ObjectId")]
    pub object_id: u32,
    #[serde(rename = "ContainerId")]
    pub container_id: u32,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
    #[serde(rename = "ContainerType")]
    pub container_type: u32,
    #[serde(rename = "OrderedObjectId")]
    pub ordered_object_id: u32,
    #[serde(rename = "OrderedSequence")]
    pub ordered_sequence: u32,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "OpCode")]
    pub opcode: u32,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageDirection")]
    pub message_direction: String,
}

impl ItemServerSaysContainId {
    pub fn read(reader: &mut BinaryReader, ordered_object_id: u32, ordered_sequence: u32) -> Result<Self> {
        let object_id = reader.read_u32()?;
        let container_id = reader.read_u32()?;
        let slot_index = reader.read_u32()?;
        let container_type = reader.read_u32()?;

        Ok(Self {
            object_id,
            container_id,
            slot_index,
            container_type,
            ordered_object_id,
            ordered_sequence,
            event_type: "Item_ServerSaysContainId".to_string(),
            opcode: 0xF7B0,
            message_type: "Ordered_GameEvent".to_string(),
            message_direction: "ServerToClient".to_string(),
        })
    }
}

// Helper functions for property names
fn property_int_name(key: u32) -> String {
    match key {
        0x05 => "Age",
        0x10 => "ArmorLevel",
        0x11 => "Level",
        0x6F => "Value",
        _ => return format!("PropertyInt_{}", key),
    }.to_string()
}

fn vital_name(key: u32) -> String {
    match key {
        1 => "Health",
        2 => "Stamina",
        3 => "Mana",
        _ => return format!("Vital_{}", key),
    }.to_string()
}

fn property_instance_id_name(key: u32) -> String {
    match key {
        1 => "Owner",
        2 => "Container",
        3 => "Wielder",
        _ => return format!("PropertyInstanceId_{}", key),
    }.to_string()
}
