#[macro_use]
extern crate log;

extern crate remcached;
extern crate memcached;

use std::env;
use std::process::exit;
use std::net::SocketAddr;

use remcached::logger::RemcachedLogger;

use log::LogLevel;

fn main()
{
    RemcachedLogger::new(LogLevel::Debug).init().unwrap();

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = match addr.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(err) => {
            println!("Invalid addr: {} err: {}", addr, err);
            exit(-1)
        }
    };

    println!("Listening on: {}", addr); 

    let server = memcached::Server::new(addr);

    if let Err(err) = server.serve() {
        println!("Failed to start server: {}", err);
    } else {
        println!("Server stopped");
    }
}