use std::env;
use std::path;

use crate::server::Server;
use crate::website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}{}public", env!("CARGO_MANIFEST_DIR"), path::MAIN_SEPARATOR);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080");
    server.run(WebsiteHandler::new(public_path));
}
