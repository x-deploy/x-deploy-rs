use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::oid::ObjectId;
use bson::{doc, Bson};
use mongodb::results::UpdateResult;
use serde::{Deserialize, Serialize};

const ORGANIZATION_COLLECTION_NAME: &str = "organizations";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Organization {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "logoUrl")]
  pub logo_url: Option<String>,

  #[serde(rename = "website")]
  pub website: Option<String>,

  #[serde(rename = "contactEmail")]
  pub contact_email: Option<String>,
}

impl Organization {
  pub fn new(
    name: String,
    description: Option<String>,
    website: Option<String>,
    contact_email: Option<String>,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      name,
      description,
      logo_url: None,
      website,
      contact_email,
    }
  }
}

impl ToCollectionName for Organization {
  fn collection_name() -> String {
    String::from(ORGANIZATION_COLLECTION_NAME)
  }
}

impl CommonCollection<Organization> {
  pub async fn update_info(
    &self,
    org_id: &ObjectId,
    name: String,
    description: Option<String>,
    website: Option<String>,
    contact_email: Option<String>,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": org_id
    };
    let update = doc! {
      "$set": {
        "name": name,
        "description": description,
        "website": website,
        "contactEmail": contact_email,
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    Ok(result)
  }

  pub async fn update_logo_url(
    &self,
    org_id: &ObjectId,
    logo_url: &Option<String>,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": org_id
    };
    let bson_logo = match logo_url {
      Some(url) => Bson::String(url.clone()),
      None => Bson::Null,
    };
    let update = doc! {
      "$set": {
        "logoUrl": bson_logo
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    Ok(result)
  }
}
