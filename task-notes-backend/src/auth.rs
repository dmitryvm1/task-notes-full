use crate::config;
use oauth2::{basic::BasicClient, TokenResponse};
// Alternatively, this can be oauth2::curl::http_client or a custom.
use crate::diesel::prelude::*;
use model::models::NewAppUser;
use model::schema::app_user;
use crate::Pool;
use actix_http::*;
use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use diesel::query_dsl::RunQueryDsl;
use diesel::PgConnection;
use model::models::AppUser;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleProfile {
    pub id: Option<String>,
    pub email: Option<String>,
    pub verified_email: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub link: Option<String>,
    pub picture: Option<String>,
    pub gender: Option<String>,
    pub locale: Option<String>,
}

#[inline]
pub fn build_google_auth_client(config: &config::Config) -> BasicClient {
    let google_client_id = ClientId::new(config.google_client_id.clone().unwrap());
    let google_client_secret = ClientSecret::new(config.google_client_secret.clone().unwrap());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Google OAuth2 process.
    BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(format!(
            "{}google_oauth/",
            &config.domain_root_url.clone().unwrap()
        ))
        .expect("Invalid redirect URL"),
    )
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    )
}

pub async fn login(
    _template: web::Data<tera::Tera>,
    oauth_client: web::Data<BasicClient>,
) -> Result<HttpResponse, Error> {
    let _ctx = tera::Context::new();
    let client = oauth_client;
    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_owned(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_owned(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_owned(),
        ))
        .add_extra_param("access_type", "offline")
        //.set_pkce_challenge(pkce_code_challenge)
        .url();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, authorize_url.as_str())
        .finish())
}

#[derive(Debug, Default)]
pub struct WebClientError {}

impl std::fmt::Display for WebClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error sending request")
    }
}

impl ResponseError for WebClientError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        let mut res = HttpResponse::new(self.status_code());

        let buf = bytes::BytesMut::new();

        res.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            header::HeaderValue::from_str("text/plain; charset=utf-8").unwrap(),
        );

        res.set_body(body::BoxBody::new(buf))
    }
}
pub async fn google_oauth(
    req: HttpRequest,
    pool: web::Data<Pool>,
    query: web::Query<HashMap<String, String>>,
    oauth_client: web::Data<BasicClient>,
    web_client: web::Data<awc::Client>,
) -> impl Responder {
    use oauth2::reqwest::{async_http_client, http_client};
    let code = AuthorizationCode::new(query.get("code").unwrap().to_string());
    let token = oauth_client
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    if token.is_err() {
        log::debug!("{:?}", token);
        return actix_web::web::Redirect::to("/").using_status_code(StatusCode::BAD_REQUEST);
    }
    let response = web_client
        .get(&format!(
            "https://www.googleapis.com/userinfo/v2/me?access_token={}",
            token.unwrap().access_token().secret()
        ))
        .send()
        .await
        .map_err(|e| {
            log::debug!("{:?}", e);
            Error::from(WebClientError {})
        })
        .unwrap()
        .body();
    let data = response.await.unwrap();
    let json: GoogleProfile = serde_json::from_slice(&data).expect("bad gauth response");
    let user_email = json.email.clone().expect("no email");
    log::info!("Setting identity: {}", &user_email);
    Identity::login(&req.extensions(), user_email.clone()).unwrap();

    let conn: &mut PgConnection = &mut pool.get().unwrap();
    let user = app_user::dsl::app_user
        .filter(app_user::dsl::email.eq(user_email.clone()))
        .load::<model::models::AppUser>(conn);
    match user {
        Err(_) => {}
        Ok(vec) => {
            if vec.is_empty() {
                log::info!("Creating user");
                let usr = NewAppUser {
                    email: user_email.clone(),
                };
                let _res: AppUser = diesel::insert_into(model::schema::app_user::table)
                    .values(&usr)
                    .get_result(conn)
                    .unwrap();
            }
        }
    };

    actix_web::web::Redirect::to("/").using_status_code(StatusCode::FOUND)

    /*let json: GoogleProfile = serde_json::from_reader(resp.unwrap()).expect("bad gauth response");
    req.remember(json.email.expect("no email"));
    fut_ok(HttpResponse::Found().header("location", "/api/").finish()).responder()*/
}
