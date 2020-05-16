use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub tokens_allowed_to_admin: Option<Vec<String>>,
    pub tokens_allowed_to_publish: Option<Vec<String>>,
    pub tokens_allowed_to_subscribe: Option<Vec<String>>,
    pub tokens_disallowed_to_admin: Option<Vec<String>>,
    pub tokens_disallowed_to_publish: Option<Vec<String>>,
    pub tokens_disallowed_to_subscribe: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvisionInfo {
    pub rooms: Vec<Room>,
    pub tokens_allowed_to_enter: Option<Vec<String>>,
    pub tokens_disallowed_to_enter: Option<Vec<String>>,
}
