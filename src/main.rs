use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use webtest::configuration::get_configuration;
use webtest::startup::run;
use webtest::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("webtest".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    run(listener, connection_pool)?.await
}
