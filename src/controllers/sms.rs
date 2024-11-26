use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::controllers::error::ControllerError;

#[derive(Deserialize, Serialize, Clone)]
pub struct SendSMSRequest {
    pub phone_number: String,
}

#[derive(Serialize)]
pub struct SendSMSResponse {
    pub message: String,
}

#[post("/sms/send")]
pub async fn send_sms(request: web::Json<SendSMSRequest>) -> impl Responder {
    // Basic phone number validation
    if request.phone_number.is_empty() || !is_valid_phone_number(&request.phone_number) {
        return HttpResponse::BadRequest().json(ControllerError {
            message: "Invalid phone number.".to_owned(),
            code: "invalid_phone_number".to_owned(),
        });
    }

    // Placeholder for actual SMS sending logic
    // In a real project, you'd integrate with Twilio or another SMS service
    HttpResponse::Ok().json(SendSMSResponse {
        message: "SMS sent successfully".to_owned()
    })
}

// Basic phone number validation function
fn is_valid_phone_number(phone: &str) -> bool {
    // Example validation: 
    // - Starts with '+' 
    // - Contains only digits after '+'
    // - Minimum length of 10 digits
    phone.starts_with('+') && 
    phone.chars().skip(1).all(|c| c.is_digit(10)) && 
    phone.len() >= 11
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_number_validation() {
        assert!(is_valid_phone_number("+917259681100"));
        assert!(!is_valid_phone_number("invalid_phone"));
        assert!(!is_valid_phone_number(""));
        assert!(!is_valid_phone_number("+"));
    }
}