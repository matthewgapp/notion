use serde::{Deserialize, Serialize};

use crate::models::users::User;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub access_token: String,
    pub bot_id: String,
    /// not documented
    pub token_type: Option<String>,
    pub duplicated_template_id: Option<String>,
    pub owner: Owner,
    /// this can be null which isn't documented
    pub workspace_icon: Option<String>,
    pub workspace_id: String,
    pub workspace_name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Owner {
    workspace: bool,
    // TODO: should be an enum
    r#type: String,
    user: Option<User>,
}
