use crate::config::Config;
use crate::fairing::cors::Cors;
use lazy_static::lazy_static;
use rocket::futures::StreamExt;
use rocket::serde::Deserialize;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[macro_use]
extern crate rocket;

mod cipher;
mod config;
mod error;
mod fairing;
mod guard;
mod ovh;
mod permission;
mod responder;
mod route;
pub mod utils;

extern crate ovh_api;

lazy_static! {
  pub(crate) static ref CONFIG: Config = Config::from_config_file();
}

#[derive(OpenApi)]
#[openapi(
    info(
      title = "X-Deploy API",
      description = "My Api description",
      version = "1.0.0",
      contact(
        name = "X-Deploy",
        email = "contact@x-deploy.com"
      )
    ),
    paths(
        // Auth
        route::auth::login,
        route::auth::magic_link,
        route::auth::register,
        route::auth::two_factor,
        route::auth::two_factor_recovery,
        route::auth::forgot_password,
        route::auth::reset_password,
        // Account
        route::account::get_info,
        route::account::verify_email,
        route::account::change_password,
        route::account::change_phone,
        route::account::info_2fa,
        route::account::setup_2fa,
        route::account::enable_2fa,
        route::account::disable_2fa,
        // Invitation
        route::invitation::get_all,
        route::invitation::response,
        // Organization
        route::organization::all,
        route::organization::new,
        route::organization::get_by_id,
        route::organization::update,
        route::organization::delete,
        route::organization::transfer,
        // Organization Invitiation
        route::organization::invitation::get_all,
        route::organization::invitation::new_invitation,
        route::organization::invitation::delete_invitation,
        // Organization Api Keys
        route::organization::api_key::new,
        route::organization::api_key::get,
        route::organization::api_key::get_by_id,
        route::organization::api_key::delete,
        // Organization Members
        route::organization::member::get_all,
        route::organization::member::delete,
        // Organization Project
        route::organization::project::new,
        route::organization::project::get_by_id,
        route::organization::project::update,
        route::organization::project::delete,
        // Organization Credentials Docker Hub
        route::organization::credentials::docker_hub::new,
        route::organization::credentials::docker_hub::get,
        route::organization::credentials::docker_hub::get_all,
        route::organization::credentials::docker_hub::update,
        route::organization::credentials::docker_hub::delete,
        // Organization Credentials Aws
        route::organization::credentials::aws::new,
        route::organization::credentials::aws::get,
        route::organization::credentials::aws::get_all,
        route::organization::credentials::aws::update,
        route::organization::credentials::aws::delete,
        // Cloud Provider
        route::cloud_provider::all,
        // Cloud Provider Aws
        route::cloud_provider::aws::all_region,
        route::cloud_provider::aws::instance_types,
    ),
    components(schemas(
        // Global
        route::SuccessMessage,
        route::ErrorMessage,
        // Auth
        route::auth::dto::LoginRequest,
        route::auth::dto::LoginResponse,
        route::auth::dto::MagicLinkRequest,
        route::auth::dto::RegisterRequest,
        route::auth::dto::TwoFactorRecoveryRequest,
        route::auth::dto::TwoFactorCode,
        route::auth::dto::ForgotPasswordRequest,
        route::auth::dto::ResetPasswordRequest,
        // Account
        route::account::dto::GetAccountInfoResponse,
        route::account::dto::VerifyEmailRequest,
        route::account::dto::ChangePasswordRequest,
        route::account::dto::ChangePhoneRequest,
        route::account::dto::TwoFactorSetupRequest,
        route::account::dto::TwoFactorSetupResponse,
        route::account::dto::TwoFactorInfoRequest,
        route::account::dto::TwoFactorInfoResponse,
        route::account::dto::TwoFactorCodeRequest,
        // Invitation
        route::invitation::dto::InvitationInfoResponse,
        route::invitation::dto::InvitationInfoUser,
        route::invitation::dto::InvitationInfoOrganization,
        route::invitation::dto::InvitationResponseRequest,
        // Organization
        route::organization::dto::CreateOrganizationRequest,
        route::organization::dto::TransferOrganizationRequest,
        route::organization::dto::OrganizationInfoResponse,
        route::organization::dto::UpdateOrganizationRequest,
        route::organization::dto::DeleteOrganizationRequest,
        // Organization Invitiation
        route::organization::invitation::dto::NewOrganizationInvitationRequest,
        route::organization::invitation::dto::OrganizationInvitationInfoResponse,
        route::organization::invitation::dto::OrganizationInvitationInfoUser,
        // Organization Members
        route::organization::member::dto::MemberInfoResponse,
        // Organization Api Keys
        route::organization::api_key::dto::CreateApiKeyRequest,
        // Organization Project
        route::organization::project::dto::CreateProjectRequest,
        route::organization::project::dto::ProjectInfoResponse,
        route::organization::project::dto::UpdateProjectInfoRequest,
        // Organization Credentials Docker Hub
        route::organization::credentials::docker_hub::dto::DockerHubInfoResponse,
        route::organization::credentials::docker_hub::dto::NewDockerHubRequest,
        route::organization::credentials::docker_hub::dto::UpdateDockerHubCredentialsRequest,
        // Organization Credentials Aws
        route::organization::credentials::aws::dto::AwsCredentialsInfoResponse,
        route::organization::credentials::aws::dto::NewAwsCredentialsRequest,
        route::organization::credentials::aws::dto::UpdateAwsCredentialsRequest,
        // Cloud Provider
        route::cloud_provider::dto::CloudProviderResponse,
        // Cloud Provider Aws
        route::cloud_provider::aws::dto::CloudProviderAwsRegion,
        route::cloud_provider::aws::dto::CloudProviderAwsInstance,
    ))
)]
struct ApiDoc;

#[rocket::launch]
async fn rocket() -> _ {
  let mongodb_client =
    mongodb::Client::with_uri_str(CONFIG.mongodb_url.as_str())
      .await
      .expect("Failed to connect to mongodb");
  let mongodb_database =
    mongodb_client.database(CONFIG.mongodb_database.as_str());
  let redis_client = redis::Client::open(CONFIG.redis_url.as_str()).unwrap();

  // Catchers

  let catcher_list = catchers![
    responder::not_found,
    responder::unauthorized,
    responder::forbidden,
    responder::internal_server_error,
    responder::unprocessable_entity
  ];

  // Routes

  let routes = routes![
    // Auth
    route::auth::register,
    route::auth::login,
    route::auth::magic_link,
    route::auth::two_factor,
    route::auth::reset_password,
    route::auth::forgot_password,
    // Account
    route::account::get_info,
    route::account::verify_email,
    route::account::change_password,
    route::account::change_phone,
    route::account::info_2fa,
    route::account::setup_2fa,
    route::account::enable_2fa,
    route::account::disable_2fa,
    // Invitation
    route::invitation::get_all,
    route::invitation::response,
    // Organization
    route::organization::all,
    route::organization::new,
    route::organization::get_by_id,
    route::organization::update,
    route::organization::delete,
    route::organization::transfer,
    // Organization Api Keys
    route::organization::api_key::new,
    route::organization::api_key::get,
    route::organization::api_key::get_by_id,
    route::organization::api_key::delete,
    // Organization Members
    route::organization::member::get_all,
    route::organization::member::delete,
    // Organization Project
    route::organization::project::new,
    route::organization::project::get_by_id,
    route::organization::project::update,
    route::organization::project::delete,
    // Organization Aws Credentials
    route::organization::credentials::aws::new,
    route::organization::credentials::aws::get,
    route::organization::credentials::aws::get_all,
    route::organization::credentials::aws::update,
    route::organization::credentials::aws::delete,
    // Organization Azure Credentials
    route::organization::credentials::azure::new,
    route::organization::credentials::azure::get,
    route::organization::credentials::azure::delete,
    // Organization Google Cloud Credentials
    route::organization::credentials::google_cloud::new,
    route::organization::credentials::google_cloud::get,
    route::organization::credentials::google_cloud::delete,
    // Organization Ovh Credentials
    route::organization::credentials::ovh::new,
    route::organization::credentials::ovh::get,
    route::organization::credentials::ovh::delete,
    // Organization Credentials Docker Hub
    route::organization::credentials::docker_hub::new,
    route::organization::credentials::docker_hub::get,
    route::organization::credentials::docker_hub::get_all,
    route::organization::credentials::docker_hub::update,
    route::organization::credentials::docker_hub::delete,
    // Cloud Provider Aws
    route::cloud_provider::aws::all_region,
    route::cloud_provider::aws::instance_types,
  ];

  let doc = ApiDoc::openapi();

  let swagger_ui = SwaggerUi::new("/swagger-ui/<_..>")
    .url("/api-docs/openapi.json", ApiDoc::openapi());

  let redoc_ui = Redoc::with_url("/redoc", ApiDoc::openapi());

  rocket::build()
    .attach(Cors)
    .manage(mongodb_database)
    .manage(redis_client)
    .register("/", catcher_list)
    .mount("/", swagger_ui)
    .mount("/", redoc_ui)
    .mount("/", routes)
    .configure(rocket::Config {
      address: "0.0.0.0".parse().unwrap(),
      port: 8000,
      ..rocket::Config::default()
    })
}
