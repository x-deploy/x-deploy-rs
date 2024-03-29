use crate::guard::auth::Auth;
use crate::route::organization::project::cluster::dto::{
  ClusterInfoResponse, CreateClusterRequest,
};
use crate::route::{ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
pub mod dto;

#[deprecated]
#[utoipa::path(
  post,
  operation_id = "Create a new cluster",
  path = "/organization/<org_id>/project/<project_id>/cluster",
  tag = "Organization Project Clusters",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Create a new cluster", body = SuccessMessage),
  ),
  request_body = CreateClusterRequest,
)]
#[post("/organization/<org_id>/project/<project_id>/cluster", data = "<body>")]
pub async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
  body: Json<CreateClusterRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, auth, org_id, project_id, body).await
}

#[deprecated]
#[utoipa::path(
  get,
  operation_id = "Get all clusters of a project",
  path = "/organization/<org_id>/project/<project_id>/cluster",
  tag = "Organization Project Clusters",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Get all clusters of a project", body = Vec<ClusterInfoResponse>),
  ),
)]
#[get(
  "/organization/<org_id>/project/<project_id>/cluster",
  format = "application/json"
)]
pub async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
) -> ApiResult<Vec<ClusterInfoResponse>> {
  controller::get_all(db, auth, org_id, project_id).await
}

#[deprecated]
#[utoipa::path(
  get,
  operation_id = "Get a cluster of a project",
  path = "/organization/<org_id>/project/<project_id>/cluster/<cluster_id>",
  tag = "Organization Project Clusters",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Get a cluster of a project", body = ClusterInfoResponse),
  ),
)]
#[get(
  "/organization/<org_id>/project/<project_id>/cluster/<cluster_id>",
  format = "application/json"
)]
pub async fn get(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
  cluster_id: &str,
) -> ApiResult<ClusterInfoResponse> {
  controller::get(db, auth, org_id, project_id, cluster_id).await
}
