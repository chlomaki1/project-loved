use std::{env, io};
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use errors::LovedError;
use state::LovedState;

pub mod routes;
pub mod service;
pub mod state;
pub mod errors;

#[cfg(unix)]
const SOCKET_PATH: &str = "/tmp/loved_server.sock";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("A proper environmental file has not been found");

    // Initialize the state
    let state = LovedState::new().await;
    let workers = if env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()) == "production" {
        8
    } else {
        4
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::JsonConfig::default().error_handler(|err, _| LovedError::from(err).into()))
            .app_data(web::Data::new(state.clone()))
            // /oauth
            .service(
                web::scope("/oauth")
                    .service(routes::oauth::start_token)
                    .service(routes::oauth::login_token_callback),
            )
            .default_service(web::route().to(routes::handle_default))
    })
    .workers(workers);

    #[cfg(unix)]
    {
        let _ = fs::remove_file(SOCKET_PATH); // Remove old socket if exists
        server.bind_uds(SOCKET_PATH)?.run().await
    }

    #[cfg(not(unix))]
    {
        let port: u16 = env::var("SERVER_PORT")
            .expect("A port must be provided")
            .parse()
            .unwrap();

        server.bind(("127.0.0.1", port))?.run().await
    }
}
