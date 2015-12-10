//extern crate mio;
extern crate coio;
//use options::Options;
use result::{ZmqError, ZmqResult};
use socket_base::SocketMessage;
//use stream_engine::StreamEngine;
//
//use std::old_io::Acceptor;
//use std::old_io::net::tcp::TcpAcceptor;
//use std::sync::Arc;
//use std::sync::RwLock;
use coio::sync::mpsc::Sender;
use std::sync::mpsc::SendError;
//use std::thread::Thread;
//
//
//static ACCEPT_TIMEOUT: u64 = 1000;
//
//
pub struct TcpListener {
	listener: coio::net::tcp::TcpListener,
    chan_to_socket: Sender<ZmqResult<SocketMessage>>,
//    options: Arc<RwLock<Options>>,
}

impl TcpListener {
    fn run(&mut self) -> Result<(), SendError<ZmqResult<SocketMessage>>> {
        loop {
//            self.acceptor.set_timeout(Some(ACCEPT_TIMEOUT));
            match self.listener.accept() {
                Ok(stream) => {
                    if self.chan_to_socket.send(Ok(SocketMessage::Ping)).is_err() {
                        return Ok(());
                    }

//                    let options = self.options.clone();
//                    StreamEngine::spawn_new(stream, options, self.chan_to_socket.clone(), None);
                }
                Err(e) =>
                    try!(self.chan_to_socket.send(Err(ZmqError::from_io_error(e)))),
            }
        }
    }

    pub fn spawn_new(listener_: coio::net::TcpListener,
                     chan: Sender<ZmqResult<SocketMessage>>) {
//                     options: Arc<RwLock<Options>>) {
        coio::spawn(move || {
            let mut listener = TcpListener {
                listener: listener_,
//                options: options,
                chan_to_socket: chan,
            };
            let _ = listener.run();
        });
    }
}
