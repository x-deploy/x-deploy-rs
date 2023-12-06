use bson::{doc, oid};
use crate::route::{Message, MessageResult};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::cipher::token::Token;
use crate::{custom_response, get_organization_by_id};

#[openapi(tag = "Organization Member")]
#[get("/organization/<id>/member", format = "application/json")]
pub(crate) async fn get(
    db: &State<Database>,
    token: Token,
    id: String,
) -> MessageResult {
    let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[openapi(tag = "Organization Member")]
#[delete("/organization/<id>/member/<member_id>", format = "application/json")]
pub(crate) async fn delete(
    db: &State<Database>,
    token: Token,
    id: String,
    member_id: String,
) -> MessageResult {
    let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}