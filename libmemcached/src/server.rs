use futures;

use std::net::SocketAddr;

use std::io;

use futures::Future;
use futures::stream::Stream;
use tokio_core::io::{read_to_end, write_all, TaskIo};
use tokio_core::Loop;

use storage::Storage;

use protocol::request::Request;

pub struct Server<'a> {
    addr: &'a SocketAddr,
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a SocketAddr) -> Self {
        Server{addr: addr}
    }

    pub fn serve(&self) -> io::Result<()> {
        let mut l = Loop::new().unwrap();

        let listener = l.handle().tcp_listen(self.addr);

        let storage = Storage::new();

        let srv = listener.and_then(move |socket| {
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

        try!(l.run(srv));

        Err(io::Error::new(io::ErrorKind::Interrupted, "Server stopping due to interruption"))
    }
}
