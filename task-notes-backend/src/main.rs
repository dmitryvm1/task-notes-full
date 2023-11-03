extern crate json;
#[macro_use]
extern crate serde_derive;
extern crate diesel;
extern crate tera;
extern crate dotenv;

pub mod api;
pub mod auth;
pub mod config;
// pub mod auth_middleware;

use actix_session::config::PersistentSession;
use actix_web::cookie::time::Duration;
use actix_web::{cookie::Key, middleware, web, App, HttpServer};

use actix_cors::Cors;
use api::*;

use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
// use actix_web::client::Client;
use awc::Client;
use tera::Tera;

/// This handler uses json extractor
//fn index(item: web::Json<MyObj>) -> HttpResponse {
//println!("model: {:?}", &item);
//HttpResponse::Ok().json(item.0) // <- send response
//}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
/*fn index_manual(
    payload: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // payload is a stream of Bytes objects
    payload
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        .and_then(|body| {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<MyObj>(&body)?;
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        })
}
*/

/// This handler manually load request payload and parse json-rust
/*fn index_mjsonrust(pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(|body| {
        // body is loaded, now we can deserialize json-rust
        let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(injson.dump()))
    })
}
*/
/*
fn index(id: Identity) -> String {
    format!("Hello {}", id.identity().unwrap_or("Anonymous".to_owned()))
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("../model/.env").ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,backend=debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app = move || {
        //Initialize AppState
        let secret_key = Key::generate();
        let web_client = Client::default();
        let tera = Tera::new("templates/**/*").unwrap();
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allow_any_header()
            .supports_credentials();

        const ONE_MINUTE: Duration = Duration::minutes(60);

        //Initialize OAuth
        let oauth_client = auth::build_google_auth_client(&config::Config::read_from_env());
        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(oauth_client))
            .app_data(web::Data::new(web_client))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(
                web::resource("/api/task")
                    .app_data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(create_task))
                    .route(web::get().to(get_tasks))
                    .route(web::delete().to(delete_task)),
            )
            .service(
                web::resource("/api/project")
                    .app_data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(create_project))
                    .route(web::get().to(get_projects))
                    .route(web::delete().to(delete_project))
                    .route(web::patch().to(update_project)),
            )
            .service(web::resource("/login").route(web::get().to(auth::login)))
            .service(web::resource("/api/logout").to(logout))
            .service(web::resource("/google_oauth/").route(web::get().to(auth::google_oauth)))
            .service(actix_files::Files::new("/assets", "./assets/").use_last_modified(false))
            .service(
                actix_files::Files::new("/", "../task-notes-gui/dist")
                    .index_file("index.html")
                    //TODO ONLY FOR DEVELOPMENT
                    .use_etag(false)
                    .use_last_modified(false),
            )
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth-example".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                    .build(),
            )
            .wrap(middleware::DefaultHeaders::new().header("Cache-Control", "no-cache"))
            // enable logger
            // .wrap(auth_middleware::Auth)
            .wrap(middleware::Logger::default())
            .wrap(cors)
    };
    HttpServer::new(app)
        .bind(format!("{}:{}", get_bind_host(), get_server_port()))?
        .run()
        .await
}

fn get_bind_host() -> String {
    std::env::var("TM_BIND_HOST")
        .ok()
        .unwrap_or("0.0.0.0".to_owned())
}

fn get_server_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8180)
}
