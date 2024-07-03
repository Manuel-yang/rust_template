use std::net::TcpListener;
use actix_cors::Cors;
use actix_web::{dev::Server, http::header, middleware::Logger, web::{self, scope, ServiceConfig}, App, HttpServer};

use crate::{config::{AppSettings, Settings}, route::hello::hello_world_handler};

#[doc = "Application state"]
pub struct AppState {
    pub settings: Settings,
}

#[doc = "Setup the service served by the application."]
fn get_config(conf: &mut ServiceConfig, settings: &AppSettings) {
    conf.service(
    scope(&settings.api_prefix)
            .service(hello_world_handler)
    );
}

#[doc = "Create a runnable server instance."]
pub async fn run(
    tcp_listener: TcpListener,
    settings: Settings,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        // Configure CORS
        let cors = if &settings.application.cors_location != "*" {
            Cors::default()
                .allowed_origin(&settings.application.cors_location)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    header::ACCEPT,
                    header::AUTHORIZATION,
                ])
                .supports_credentials()
        } else {
            Cors::default()
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    header::ACCEPT,
                    header::AUTHORIZATION,
                ])
                .supports_credentials()
        };

        // Configure the application
        App::new()
            .app_data(web::Data::new(AppState {
                settings: settings.clone(),
            }))
            .configure(|c| get_config(c, &settings.application))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}