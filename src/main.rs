// web socket server
extern crate ws;

use ws::{listen, CloseCode, Message, Sender, Handler, Result};
use std::time:: Instant;


struct Server {
    ws: Sender,
    start: Instant
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        let elapsed = self.start.elapsed();
        self.ws.broadcast(format!("{}: {}", elapsed.subsec_millis(), msg))
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        println!("lost a connection");
    }
}

fn main() {


    listen("localhost:3012", |out| {
        Server {
            ws: out,
            start: Instant::now()
        }
    }).unwrap()
}
