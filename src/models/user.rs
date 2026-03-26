use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub username: String,
    #[expect(dead_code, reason = "stored for future auth flow, not returned in API response")]
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub fullname: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub fullname: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub fullname: Option<String>,
}
