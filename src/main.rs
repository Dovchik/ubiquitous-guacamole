use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    run(listener)?.await
}
