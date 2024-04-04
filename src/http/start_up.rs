use std::{net::SocketAddr, sync::Arc};

use my_http_server::MyHttpServer;

use crate::http::MyMiddleware;

pub fn setup_server(port: u16) -> MyHttpServer {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], port)));
    println!("Http server port is: {}", port);

    http_server.add_middleware(Arc::new(MyMiddleware));

    http_server
}
