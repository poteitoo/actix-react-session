use actix_cors::Cors;
use actix_session::{
    config::PersistentSession, storage::RedisActorSessionStore, SessionMiddleware,
};
use actix_web::{cookie, middleware, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    log::info!("starting HTTP server at http://localhost:8000");

    HttpServer::new(|| {
        let private_key = actix_web::cookie::Key::generate();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    // .allow_any_origin()
                    .allow_any_method()
                    .allowed_origin("https://localhost:3000")
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("redis:6379"),
                    private_key.clone(),
                )
                .cookie_secure(false)
                .session_lifecycle(
                    PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                )
                .build(),
            )
            .route("/login", web::post().to(auth::login))
            .route("/validate", web::get().to(auth::validate))
    })
    .workers(1)
    .bind_openssl(("127.0.0.1", 8000), builder)?
    // .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
