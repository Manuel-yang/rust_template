use std::net::TcpListener;
use hello_world::config::{create_logging_subscriber, get_configuration, init_sub};
use hello_world::server::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = create_logging_subscriber("api".into(), "info".into());
    init_sub(subscriber);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let settings = get_configuration().expect("failed to read app configuration");
    
    let listener = TcpListener::bind(settings.application.get_addr()).expect("faild to bind");

    run(listener, settings).await?.await

}