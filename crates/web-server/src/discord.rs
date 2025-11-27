//! Discord API integration for fetching message attachments

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DiscordMessage {
    pub id: String,
    pub channel_id: String,
    pub attachments: Vec<DiscordAttachment>,
}

#[derive(Debug, Deserialize)]
pub struct DiscordAttachment {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub content_type: Option<String>,
}

/// Validate a Discord snowflake ID (18-digit number)
pub fn is_valid_snowflake(id: &str) -> bool {
    id.len() == 18 && id.chars().all(|c| c.is_ascii_digit())
}

/// Fetch message details from Discord API
pub async fn fetch_message(
    channel_id: &str,
    message_id: &str,
    token: &str,
) -> Result<DiscordMessage, (StatusCode, String)> {
    // Validate snowflake IDs
    if !is_valid_snowflake(channel_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid channel ID format".to_string(),
        ));
    }
    if !is_valid_snowflake(message_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid message ID format".to_string(),
        ));
    }

    // TODO: Implement actual Discord API call
    // GET https://discord.com/api/v10/channels/{channel_id}/messages/{message_id}
    // with Authorization header: Bearer {token}

    Err((
        StatusCode::NOT_IMPLEMENTED,
        "Discord API fetch not yet implemented".to_string(),
    ))
}

/// Download attachment from URL
pub async fn download_attachment(
    url: &str,
    token: &str,
) -> Result<Vec<u8>, (StatusCode, String)> {
    // TODO: Implement attachment download
    // Use reqwest to fetch from url with Discord auth headers if needed

    Err((
        StatusCode::NOT_IMPLEMENTED,
        "Attachment download not yet implemented".to_string(),
    ))
}
