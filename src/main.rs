mod utils;
mod client;
mod server;

use std::env;

use client::start_client;
use server::start_server;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server = &args[1] == "server";

    if server {
        start_server();
    } else {
        start_client();
    }
}