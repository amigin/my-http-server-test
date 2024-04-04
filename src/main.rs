use std::sync::Arc;

use rust_extensions::AppStates;

mod http;

#[tokio::main]
async fn main() {
    let mut server = http::setup_server(8181);

    server.start(
        Arc::new(AppStates::create_initialized()),
        my_logger::LOGGER.clone(),
    );
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
