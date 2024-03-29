use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "name": "My Stunning Organization name",
    "description": "My Stunning Organization description",
}))]
pub(crate) struct CreateProjectRequest {
  #[serde(rename = "name")]
  pub(crate) name: String,

  #[serde(rename = "description")]
  pub(crate) description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "id": "5f9b3b4b9b9b9b9b9b9b9b9b",
    "name": "My Stunning Organization name",
    "description": "My Stunning Organization description",
    "logoUrl": "https://my-stunning-organization-project.net/logo.png",
    "organizationId": "5f9b3b4b9b9b9b9b9b9b9b9b",
}))]
pub(crate) struct ProjectInfoResponse {
  #[serde(rename = "id")]
  pub(crate) id: String,

  #[serde(rename = "name")]
  pub(crate) name: String,

  #[serde(rename = "description")]
  pub(crate) description: Option<String>,

  #[serde(rename = "logoUrl")]
  pub(crate) logo_url: Option<String>,

  #[serde(rename = "organizationId")]
  pub(crate) organization_id: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "name": "My Stunning New Organization name",
    "description": "My Stunning New Organization description",
}))]
pub(crate) struct UpdateProjectInfoRequest {
  #[serde(rename = "name")]
  pub(crate) name: String,
  #[serde(rename = "description")]
  pub(crate) description: Option<String>,
}
