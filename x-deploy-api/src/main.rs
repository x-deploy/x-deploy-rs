mod kbs;
mod db;
mod config;
mod route;
mod cipher;
mod guard;
mod ovh;

use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use std::string::String;
use std::sync::Arc;
use k8s_openapi::api::apps::v1::Deployment;
use rocket::futures::{stream, StreamExt};
use ovh_api::data::kbs_cluster::KbsCluster;
use kube::{Api};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::api::PostParams;
use lazy_static::lazy_static;
use ovh_api::OvhClient;
use ovh_api::data::Project;
use crate::config::DotEnvConfig;
use crate::kbs::deploy::DeployInfo;

extern crate ovh_api;

#[macro_use]
extern crate rocket;
extern crate core;

#[derive(Clone, Deserialize)]
struct Cluster {}

lazy_static! {
    pub(crate) static ref DOTENV_CONFIG: DotEnvConfig = DotEnvConfig::from_dotenv();
}

#[post("/clusters/deploy", format = "application/json", data = "<deployment>")]
async fn deploy_in_cluster(deployment: Json<DeployInfo>) -> &'static str {
    let client = Arc::new(OvhClient::new(
        std::env::var("OVH_APPLICATION_KEY").expect("OVH_APPLICATION_KEY not found"),
        std::env::var("OVH_APPLICATION_SECRET").expect("OVH_APPLICATION_SECRET not found"), std::env::var("OVH_CONSUMER_KEY").expect("OVH_CONSUMER_KEY not found"),
    ));
    let kubeconfig = ovh_api::route::cloud::get_kubconfig(&client, &deployment.project_id, &deployment.cluster_id).await.expect("Error getting kubeconfig");
    let kube_client = kbs::connect_with_kubeconfig(kubeconfig.content.as_str()).await;
    let deployment_clone = deployment.clone();
    let deployment = Deployment {
        // Populate the metadata for the Deployment
        metadata: kube::api::ObjectMeta {
            name: Some(deployment_clone.deployment_name.as_str().parse().unwrap()),
            ..Default::default()
        },
        spec: Some(k8s_openapi::api::apps::v1::DeploymentSpec {
            replicas: Some(1), // Set the number of replicas
            selector: LabelSelector {
                match_labels: Some(std::collections::BTreeMap::from([
                    ("app".parse().unwrap(), deployment_clone.app_name.as_str().parse().unwrap()),
                ])),
                ..Default::default()
            },
            template: k8s_openapi::api::core::v1::PodTemplateSpec {
                metadata: Some(kube::api::ObjectMeta {
                    labels: Some(std::collections::BTreeMap::from([
                        ("app".parse().unwrap(), deployment_clone.app_name.as_str().parse().unwrap()),
                    ])),
                    ..Default::default()
                }),
                spec: Some(k8s_openapi::api::core::v1::PodSpec {
                    containers: vec![
                        k8s_openapi::api::core::v1::Container {
                            name: deployment_clone.app_name.to_string() + "-container",
                            image: Some(deployment_clone.image.to_string() + ":" + deployment_clone.tag.as_str()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            },
            // Specify other Deployment properties here
            ..Default::default()
        }),
        ..Default::default()
    };
    let deployments: Api<Deployment> = Api::namespaced(kube_client, "default");
    match deployments.create(&PostParams::default(), &deployment).await {
        Ok(_) => {
            return "Deployment created";
        }
        Err(_) => {
            return "Error creating deployment";
        }
    }
}

#[get("/clusters/<project_id>")]
async fn get_clusters(project_id: &str) -> Json<Vec<KbsCluster>> {
    let client = Arc::new(OvhClient::new(
        std::env::var("OVH_APPLICATION_KEY").expect("OVH_APPLICATION_KEY not found"),
        std::env::var("OVH_APPLICATION_SECRET").expect("OVH_APPLICATION_SECRET not found"),
        std::env::var("OVH_CONSUMER_KEY").expect("OVH_CONSUMER_KEY not found"),
    ));
    let clusters_id: Vec<String> = ovh_api::route::cloud::get_list_cluster_kbs(&client, project_id).await.unwrap();
    let clusters: Vec<KbsCluster> = stream::iter(clusters_id)
        .then(|id| {
            let client_clone = client.clone(); // Clone the Arc here
            async move {
                ovh_api::route::cloud::get_cluster_kbs_info(&client_clone, project_id, &id).await
            }
        })
        .filter_map(|result| async move {
            match result {
                Ok(cluster) => Some(cluster),
                Err(_) => None, // or handle the error as you see fit
            }
        })
        .collect()
        .await;

    Json(clusters)
}


#[get("/projects")]
async fn get_projects() -> Json<Vec<Project>> {
    let client = Arc::new(OvhClient::new(
        std::env::var("OVH_APPLICATION_KEY").expect("OVH_APPLICATION_KEY not found"),
        std::env::var("OVH_APPLICATION_SECRET").expect("OVH_APPLICATION_SECRET not found"),
        std::env::var("OVH_CONSUMER_KEY").expect("OVH_CONSUMER_KEY not found"),
    ));
    let projets_id: Vec<String> = ovh_api::route::cloud::get_project_list(&client).await.unwrap();
    let projets: Vec<Project> = stream::iter(projets_id)
        .then(|id| {
            let client_clone = client.clone(); // Clone the Arc here
            async move {
                ovh_api::route::cloud::get_project_info(&client_clone, &id).await
            }
        })
        .filter_map(|result| async move {
            match result {
                Ok(project) => Some(project),
                Err(_) => None, // or handle the error as you see fit
            }
        })
        .collect()
        .await;

    Json(projets)
}


#[launch]
async fn rocket() -> _ {
    let mongodb_client = mongodb::Client::with_uri_str(DOTENV_CONFIG.mongodb_url.as_str()).await;
    let mongodb_database = mongodb_client.unwrap()
        .database(DOTENV_CONFIG.mongodb_database.as_str());
    info!("Connected to mongodb");
    let redis_client = redis::Client::open(DOTENV_CONFIG.redis_url.as_str()).unwrap();
    info!("Connected to redis");
    rocket::build()
        .manage(mongodb_database)
        .manage(redis_client)
        .mount("/", routes![route::auth::register, route::auth::login, route::auth::index])
        .mount("/", routes![get_clusters, get_projects, deploy_in_cluster])
        .mount("/", routes![route::ovh::post_credentials])
}
