pub mod client;
use client::server::{createSocket};
fn main() {
  createSocket("localhost:8000", String::from("<h1> Hello, world! </h1>"));
}