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
    /// not documented by this field can be omitted
    #[serde(default = "default_workspace")]
    workspace: bool,
    // TODO: should be an enum
    r#type: String,
    user: Option<User>,
}

fn default_workspace() -> bool {
    false
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[test]
    fn deserializes_correctly() {
        let json = json!({
          "access_token": "redacted_access_token",
          "token_type": "bearer",
          "bot_id": "redacted_bot_id",
          "workspace_name": "redacted_workspace_name",
          "workspace_icon": null,
          "workspace_id": "redacted_workspace_id",
          "owner": {
            "type": "user",
            "user": {
              "object": "user",
              "id": "redacted_user_id",
              "name": "redacted_user_name",
              "avatar_url": "redacted_avatar_url",
              "type": "person",
              "person": { "email": "redacted_email" }
            }
          },
          "duplicated_template_id": null
        });

        let token: super::Token = serde_json::from_value(json).unwrap();

        assert_eq!(token.access_token, "redacted_access_token");
        assert_eq!(token.bot_id, "redacted_bot_id");
    }
}
