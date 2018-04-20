// web socket server
extern crate ws;

use ws::{listen, CloseCode, Message, Sender, Handler, Result};


struct Server {
    ws: Sender,
    start: u16
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        self.start = self.start + 1;
        self.ws.broadcast(format!("{:03}: {}", self.start, msg))
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        println!("lost a connection");
    }
}

fn main() {


    listen("localhost:3012", |out| {
        Server {
            ws: out,
            start: 0
        }
    }).unwrap()
}
