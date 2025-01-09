use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCreatePayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLoginPayload {
    pub email: String,
    pub password: String,
}
