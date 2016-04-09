extern crate mio;
extern crate bytes;

#[macro_use]
extern crate log;
extern crate env_logger;

use mio::{EventLoop, Handler, Token, EventSet, PollOpt, TryRead, TryWrite};
use mio::tcp::*;
use mio::util::Slab;

use bytes::{Buf, Take};
use std::io::Cursor;
use std::mem;

#[derive(Debug)]
enum State {
    Reading(Vec<u8>),
    Writing(Take<Cursor<Vec<u8>>>),
    Closed,
}

impl State {
    fn mut_read_buf(&mut self) -> &mut Vec<u8> {
        match *self {
            State::Reading(ref mut buf) => buf,
            _ => panic!("connection not in reading state"),
        }
    }

    fn read_buf(&self) -> &[u8] {
        match *self {
            State::Reading(ref buf) => buf,
            _ => panic!("connection not in reading state"),
        }
    }

    fn write_buf(&self) -> &Take<Cursor<Vec<u8>>> {
        match *self {
            State::Writing(ref buf) => buf,
            _ => panic!("connection not in writing state"),
        }
    }

    fn mut_write_buf(&mut self) -> &mut Take<Cursor<Vec<u8>>> {
        match *self {
            State::Writing(ref mut buf) => buf,
            _ => panic!("connection not in Writing state"),
        }
    }

    fn try_transition_to_writing(&mut self) {
        if let Some(pos) = self.read_buf().iter().position(|b| *b == b'\n') {
            self.transition_to_writing(pos + 1);
        }
    }

    fn transition_to_writing(&mut self, pos: usize) {
        let buf = mem::replace(self, State::Closed).unwrap_read_buf(); 

        let buf = Cursor::new(buf);

        *self = State::Writing(Take::new(buf, pos));
    }

    fn try_transition_to_reading(&mut self) {
        if !self.write_buf().has_remaining() {
            let cursor = mem::replace(self, State::Closed)
                .unwrap_write_buf()
                .into_inner();
            let pos = cursor.position();
            let mut buf = cursor.into_inner();

            drain_to(&mut buf, pos as usize);

            *self = State::Reading(buf);

            self.try_transition_to_writing();
        }
    }

    fn unwrap_read_buf(self) -> Vec<u8> {
        match self {
            State::Reading(buf) => buf,
            _ => panic!("connection not in reading state"),
        }
    }

    fn unwrap_write_buf(self) -> Take<Cursor<Vec<u8>>> {
        match self {
            State::Writing(buf) => buf,
            _ => panic!("connection not in writing state"),
        }
    }
}

fn drain_to(vec: &mut Vec<u8>, count: usize) {
    for _ in 0..count {
        vec.remove(0);
    }
}

#[derive(Debug)]
struct Connection {
    socket: TcpStream,
    token: Token,
    state: State,
}

impl Connection {
    fn new(socket: TcpStream, token: Token) -> Connection {
        Connection {
            socket: socket,
            token: token,
            state: State::Reading(vec![]),
        }
    }

    fn ready(&mut self, event_loop: &mut EventLoop<Remcached>, events: EventSet) {
        debug!("  connection state=:{:?}", self.state);

        match self.state {
            State::Reading(..) => {
                assert!(events.is_readable(), "unexpected events; events={:?}", events);
                self.read(event_loop)
            }
            State::Writing(..) => {
                assert!(events.is_writable(), "unexpected events; events={:?}", events);
                self.write(event_loop)
            }
            _ => unimplemented!(),
        }
    }

    fn read(&mut self, event_loop: &mut EventLoop<Remcached>) {
        match self.socket.try_read_buf(self.state.mut_read_buf()) {
            Ok(Some(0)) => {
                debug!("    read 0 bytes from client; buffered={}", self.state.read_buf().len());

                match self.state.read_buf().len() {
                    n if n > 0 => {
                        self.state.transition_to_writing(n);

                        self.reregister(event_loop);
                    }
                    _ => self.state = State::Closed,
                }
            }
            Ok(Some(n)) => {
                debug!("read {} bytes", n);

                self.state.try_transition_to_writing();
                self.reregister(event_loop);
            }
            Ok(None) => {
                self.reregister(event_loop);
            }
            Err(e) => {
                panic!("got an error trying to read; err={:?}", e);
            }
        } 
    }

    fn write(&mut self, event_loop: &mut mio::EventLoop<Remcached>) {
        match self.socket.try_write_buf(self.state.mut_write_buf()) {
            Ok(Some(_)) => {
                self.state.try_transition_to_reading();

                self.reregister(event_loop);
            }
            Ok(None) => {
                self.reregister(event_loop);
            }
            Err(e) => {
                panic!("got an error trying to write; err={:?}", e);
            }
        }
    }

    fn reregister(&self, event_loop: &mut EventLoop<Remcached>) {
        let event_set = match self.state {
            State::Reading(..) => EventSet::readable(),
            State::Writing(..) => EventSet::writable(),
            _ => EventSet::none(),
        };

        // Why unwrap to make sure it is OK???
        event_loop.reregister(&self.socket, self.token, event_set, PollOpt::oneshot()).unwrap();
    }

    fn is_closed(&self) -> bool {
        match self.state {
            State::Closed => true,
            _ => false,
        }
    }
}

struct Remcached {
    server: TcpListener,
    connections: Slab<Connection>,
}

impl Remcached {
    fn new(server: TcpListener) -> Remcached {
        let slab = Slab::new_starting_at(Token(1), 1024);

        Remcached {
            server: server,
            connections: slab,
        }
    }
}

const SERVER: Token = Token(0);

impl Handler for Remcached {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token,
             events: EventSet) {

        debug!("Token: {:?}", token);
        match token {
            SERVER =>  {
                info!("the server socket is ready to accept connection");
                match self.server.accept() {
                    Ok(Some((socket, _))) => {
                        debug!("accepted a socket");

                        let token = self.connections
                            .insert_with(|token| Connection::new(socket, token))
                            .unwrap();

                        event_loop.register(
                            &self.connections[token].socket,
                            token,
                            EventSet::readable(),
                            PollOpt::edge() | PollOpt::oneshot()).unwrap();
                    }
                    Ok(None) => {
                        warn!("the server socket wasn't actually ready")
                    }
                    Err(e) => {
                        error!("listener.accept() error: {}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ => {
                self.connections[token].ready(event_loop, events);

                if self.connections[token].is_closed() {
                    let _ = self.connections.remove(token);
                }
            }
        }
    }
}

fn main()
{
    env_logger::init().ok().expect("Failed to init logger");
    let server = TcpListener::bind(&"127.0.0.1:9922".parse().unwrap()).unwrap();

    let mut e = EventLoop::new().unwrap();

    e.register(&server, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();

    info!("running remcache server");

    let mut remcached = Remcached::new(server);

    e.run(&mut remcached).ok().expect("Failed to start event loop");
}