extern crate coio;
use consts::SocketType;
use std::thread;
//use inproc::InprocManager;
use rep;
//use req;
use socket::ZmqSocket;
use socket_base::SocketBase;


/// Before using any ØMQ code you must create a ØMQ *context*.
pub struct Context {
//	tx: mioco::MailboxOuterEnd<Fn()>,
//    inproc_mgr: InprocManager,
    _val: i32,
}

impl Context {
    /// Create a new ØMQ context.
    pub fn new() -> Context {
        Context {
//			inproc_mgr: InprocManager::new(),
			_val: 0,
        }
    }

    pub fn run(&mut self) {
        use coio::{run, spawn, sched};

		spawn(|| {
			for _ in 0..10 {
				println!("Heil Hydra");
				sched();
			}
		});

		run(1);

//        self._mioco.start(move |_| {
//			Ok(())
//        });
    }

    /// This function shall create a ØMQ socket within the specified *context* and return a box of
    /// the newly created socket. The *type_* argument specifies the socket type, which determines
    /// the semantics of communication over the socket.
    ///
    /// The newly created socket is initially unbound, and not associated with any endpoints.
    /// In order to establish a message flow a socket must first be connected to at least one
    /// endpoint with [`connect`](trait.ZmqSocket.html#tymethod.connect), or at least one endpoint
    /// must be created for accepting incoming connections with
    /// [`bind`](trait.ZmqSocket.html#tymethod.bind).
    ///
    pub fn socket(&self, type_: SocketType) -> Box<ZmqSocket + Send> {
        let base = SocketBase::new();//self.inproc_mgr.chan());
        match type_ {
//            SocketType::REQ => Box::new(req::new(base) as Box<ZmqSocket + Send>),
//            SocketType::REP => Box::new(rep::new(base)) as Box<ZmqSocket + Send>,
			_ => Box::new(rep::new(base)) as Box<ZmqSocket + Send>,
        }
    }
}


#[cfg(test)]
mod test {
    use ctx::Context;

    #[test]
    fn test_new() {
        Context::new().run();
    }
}
