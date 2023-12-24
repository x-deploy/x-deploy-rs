use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::guard::token::Token;
use crate::route::{custom_message, ApiResponse, SuccessMessage};
use bson::oid;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

#[post(
  "/organization/<id>/credentials/google-cloud",
  format = "application/json"
)]
pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

#[get(
  "/organization/<id>/credentials/google-cloud",
  format = "application/json"
)]
pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

#[delete(
  "/organization/<id>/credentials/google-cloud",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
