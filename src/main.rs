use crate::server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run();
}
