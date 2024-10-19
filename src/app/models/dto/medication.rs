use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddMedicationPayload {
    pub medication_name: String,
    pub dose: String,
    pub timing: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateMedicationPayload {
    pub medication_name: String,
    pub dose: String,
    pub timing: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FindPath {
    pub medication_id: ObjectId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdatePath {
    pub medication_id: ObjectId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RemovePath {
    pub medication_id: ObjectId,
}
