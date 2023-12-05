use bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

pub(crate) const ORGANIZATION_COLLECTION_NAME: &str = "users";

#[derive(Deserialize, Serialize, Debug)]
pub struct Organization {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "website")]
    pub website: String,

    #[serde(rename = "contactEmail")]
    pub contact_email: String,

    #[serde(rename = "owner")]
    pub owner: ObjectId,

    #[serde(rename = "members")]
    pub members: Vec<ObjectId>,
}