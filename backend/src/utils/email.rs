use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::configs::global_config::CONFIG;
use crate::errors::app_error::ApiError;

pub fn send_mail(to: &str, subject: &str, body: &String) -> Result<bool, ApiError> {
    // Get the email and password from the environment
    let from_email = &CONFIG.email;
    let from_password = &CONFIG.email_password;

    // Create an email address from the recipient string
    let email = Message::builder()
        .from(format!("LMS <{}>", from_email).parse().unwrap())
        .to(format!("<{}>", to).parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.to_string())
        .map_err(|_| ApiError::BadRequest("Could not create email".to_string()))?;

    // Create a set of credentials to login to the email account
    let creds: Credentials = Credentials::new(from_email.to_owned(), from_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|_| ApiError::BadRequest("Could not connect to the email server".to_string()))?
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(true),
        Err(e) => Err(ApiError::BadRequest(format!(
            "Could not send email: {:?}",
            e
        ))),
    }
}
