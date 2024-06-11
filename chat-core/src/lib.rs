use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

mod util;
#[derive(Debug, Clone, Serialize, ToSchema, Deserialize, FromRow, PartialEq)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    #[sqlx(default)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema, Deserialize, FromRow, PartialEq)]
pub struct WorkSpace {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Serialize, ToSchema, Deserialize, FromRow, PartialEq)]
pub struct ChatUser {
    pub id: i64,
    pub fullname: String,
    pub email: String,
}

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Single,
    Group,
    PrivateChannel,
    PublicChannel,
}

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type)]
pub struct Chat {
    pub id: i64,
    pub ws_id: i64,
    pub name: String,
    pub r#type: ChatType,
    pub members: Vec<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub files: Vec<String>,
    pub created_at: DateTime<Utc>,
}
impl User {
    pub fn new(id: i64, fullname: String, email: String) -> Self {
        Self {
            id,
            ws_id: 0,
            fullname,
            email,
            password_hash: None,
            created_at: chrono::Utc::now(),
        }
    }
}
