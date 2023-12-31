use crate::db::project::{Project, PROJECT_COLLECTION_NAME};
use crate::db::query::cursor_to_vec;
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::Collection;
use mongodb::Database;

impl Project {}

pub async fn query_project_new(
  db: &Database,
  project: &Project,
) -> CommonResult<InsertOneResult> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let result = collection.insert_one(project, None).await?;
  Ok(result)
}

pub async fn query_project_get_with_org(
  db: &Database,
  org_id: &ObjectId,
) -> CommonResult<Vec<Project>> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id
  };
  let cursor = collection.find(filter, None).await?;
  let result = cursor_to_vec(cursor).await?;
  return Ok(result);
}

pub async fn query_project_get_with_org_and_id(
  db: &Database,
  org_id: &ObjectId,
  project_id: &ObjectId,
) -> CommonResult<Option<Project>> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id,
    "_id": project_id
  };
  let result = collection.find_one(filter, None).await;
  Ok(result?)
}

pub async fn query_project_update(
  db: &Database,
  org_id: &ObjectId,
  project_id: &ObjectId,
  name: &str,
  description: &str,
) -> CommonResult<UpdateResult> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id,
    "_id": project_id
  };
  let update = doc! {
    "$set": {
      "name": name,
      "description": description
    }
  };
  let result = collection.update_one(filter, update, None).await?;
  Ok(result)
}

pub async fn query_project_delete(
  db: &Database,
  org_id: &ObjectId,
  project_id: &ObjectId,
) -> CommonResult<DeleteResult> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id,
    "_id": project_id
  };
  let result = collection.delete_one(filter, None).await?;
  Ok(result)
}
