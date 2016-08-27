#[macro_use]
extern crate log;
extern crate env_logger;

extern crate futures;
extern crate tokio_core;

use std::env;
use std::process::exit;
use std::net::SocketAddr;

use futures::Future;
use futures::stream::Stream;
use tokio_core::io::{read_to_end, write_all, TaskIo};
use tokio_core::Loop;


fn main()
{
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = match addr.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(err) => {
            println!("Invalid addr: {} err: {}", addr, err);
            exit(-1)
        }
    };

    env_logger::init().ok().expect("Failed to init logger");

    let mut l = Loop::new().unwrap();

    let listener = l.handle().tcp_listen(&addr);

    let srv = listener.and_then(move |socket| {
        println!("Listening on: {}", addr); 

        socket.incoming().for_each(|(socket, cli_addr)| {
            let socket = futures::lazy(|| futures::finished(TaskIo::new(socket))); 
            let pair = socket.map(|s| s.split());

            let r = pair.and_then(move |(reader, writer)| {
                read_to_end(reader, vec!()).and_then(move |(_, buf)| {
                    println!("Get request from: {} request: {:?}", cli_addr, buf);
                    write_all(writer, buf)
                })
            });

            r.forget();

            Ok(())
        })
    });

    l.run(srv).unwrap();
}