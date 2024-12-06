mod utils;
mod client;
mod server;

use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("\nExpected arguments but found none! Type --help to get the list of commands");
    }

    if args.len() == 2 {
        for arg in &args {
            if arg == "--help" {
                println!("\nserver - starts the server listening at port 7878");
                println!("\nclient - starts the server connecting to localhost at port 7878");
                return
            }
        }
    }

    let server = &args[1] == "server";
    
    if server {
        let mut root: Option<String> = None;
        let mut port: Option<i16> = None;

        for (index, arg) in args.iter().enumerate() {

            if arg == "--help" {
                println!("\n--port <port_number> (default: 7878)");
                println!("--root <path> (default: ./)\n");
                return
            }

            if arg == "--port" {
                if let Some(arg_value) = args.get(index + 1) {
                    if let Ok(value) = arg_value.parse::<i16>() {
                        port = Some(value);
                    } else {
                        panic!("Error: Invalid port!")
                    }
                } else {
                    panic!("Error: Invalid port!");
                }
            }
    
            if arg == "--root" {
                let arg_value = args.get(index + 1).unwrap();
    
                if let true = Path::new(arg_value).is_dir() {
                    root = Some(arg_value.clone());
                } else {
                    panic!("Error: Invalid root!");
                }
            }
        }

        server::connection::start(root, port);
    } else {
        let mut addr: Option<String> = None;

        for (index, arg) in args.iter().enumerate() {

            if arg == "--help" {
                println!("\n--addr <ip:port> (default: 127.0.0.1:7878)");
                return
            }
    
            if arg == "--addr" {
                let arg_value = args.get(index + 1).unwrap();
    
                if let true = Path::new(arg_value).is_dir() {
                    addr = Some(arg_value.clone());
                } else {
                    panic!("Error: Invalid address!");
                }
            }
        }

        client::connection::start(addr);
    }
}