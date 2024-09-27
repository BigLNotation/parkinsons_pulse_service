use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateUserPayload {
    pub first_name: String,
    pub last_name: String,
    // pub national_health_identifier: String,
    pub email_address: String,
    pub password: String,
    pub is_patient: bool,
}
