use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub _id: ObjectId,
    pub uri: String,
    pub location: Coordinates,
    pub shot_on: i64,
    pub project: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    #[serde(rename = "type")]
    pub kind: String,
    pub _id: ObjectId,
    pub coordinates: Vec<f32>,
}

pub fn get_env(variable: &str) -> String {
    let error_message: String = format!("{} must be set.", variable);
    return std::env::var(variable).expect(error_message.as_str());
}
