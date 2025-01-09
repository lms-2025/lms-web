use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub data: T,
    pub message: String,
    pub success: bool,
}

impl<T> ApiResponse<T> {
    pub fn new(status_code: u16, data: T, message: String) -> Self {
        let success: bool = status_code < 400;
        Self {
            status: status_code,
            data,
            message,
            success,
        }
    }

    pub fn error(status_code: u16, message: String) -> Self
    where
        T: Default,
    {
        Self {
            status: status_code,
            data: T::default(),
            message,
            success: false,
        }
    }
}
