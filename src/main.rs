pub mod client;
use client::server::start_server;
fn main() {
  start_server("localhost:8000", String::from("<h1> Hello, world! </h1>"));
}