use futures;

use std::net::SocketAddr;

use std::io;
use std::rc::Rc;
use std::iter::repeat;
use std::cell::RefCell;

use futures::Future;
use futures::stream::{self, Stream};
use tokio_core::io::Io;
use tokio_core::io::{read_to_end, copy, write_all};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

use storage::Storage;

use protocol::request::Request;


use util::escape;


#[derive(Debug, Clone)]
pub struct Server{
    addr: SocketAddr,
    storage: Storage,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server{addr: addr, storage: Storage::new()}
    }

    pub fn serve(&self) -> io::Result<()> {
        let mut l = Core::new().unwrap();

        let handle = l.handle();
        //let server_stream = stream::iter(repeat(Ok(Rc::new(RefCell::new(self)))));

        let addr = &self.addr;

        let bound_socket = TcpListener::bind(&addr, &handle).unwrap();
        println!("Listern on: {}", addr);

        let done = bound_socket.incoming().for_each(|(socket, cli_addr)| {
            let pair = futures::lazy(|| Ok(socket.split()));

            let r = pair.and_then(move |(reader, writer)| {
                copy(reader, writer)
                    /*
                read_to_end(reader, vec!()).and_then(move |(_, req_buf)| {
                    match server.borrow_mut().handle_request(&cli_addr, &req_buf) {
                        Ok(res) => write_all(writer, res),
                        Err(e) => write_all(writer, vec![]), //fixme error response
                    }
                    write_all(writer, req_buf)
                })
                    */
            });

            handle.spawn(r.then(move |result| {
                println!("wrote {:?} bytes to {}", result, cli_addr);
                Ok(()) 
            }));

            Ok(())
        });

        try!(l.run(done));

        Err(io::Error::new(io::ErrorKind::Interrupted, "Server stopping due to interruption"))
    }

    fn handle_request(&self, cli_addr: &SocketAddr, req_buf : &Vec<u8>) -> Result<Vec<u8>, io::Error>  {
        println!("Get request from: {} request: {:?}", cli_addr, String::from_utf8(req_buf.clone()));
        Ok(vec![])
    }
}
