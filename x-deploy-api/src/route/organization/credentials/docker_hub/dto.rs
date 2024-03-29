use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "My Docker Hub",
  "description": "My Docker Hub description",
  "accessToken": "dckr_pat_sPeNJz856Sp7mOkod8oPRO1OBGE",
}))]
pub struct NewDockerHubRequest {
  pub name: String,
  pub description: Option<String>,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "id": "5f9b3b2b9d3f6c0007f7e7b1",
  "name": "My Docker Hub",
  "description": "My Docker Hub description",
  "created_at": "2020-10-30T14:30:51.000Z"
}))]
pub struct DockerHubInfoResponse {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "My Docker Hub",
  "description": "My Docker Hub description",
}))]
pub struct UpdateDockerHubCredentialsRequest {
  pub name: String,
  pub description: Option<String>,
}
