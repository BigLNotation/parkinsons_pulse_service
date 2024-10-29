use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaregiverTokenPath {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RemoveCaregiverPath {
    pub caregiver_id: ObjectId,
}
