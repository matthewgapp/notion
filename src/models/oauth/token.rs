use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub access_token: String,
    pub bot_id: String,
    pub duplicated_template_id: Option<String>,
    pub owner: Owner,
    pub workspace_icon: String,
    pub workspace_id: String,
    pub workspace_name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Owner {
    workspace: bool,
}
