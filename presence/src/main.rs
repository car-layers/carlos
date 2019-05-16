extern crate ws;

use carlos::presence;
use ws::{listen, Error, ErrorKind, Message};

fn main() {
    listen("0.0.0.0:3000", |_out| {
        println!("connected");
        move |msg| {
            let data = match msg {
                Message::Binary(data) => data,
                Message::Text(_) => return Err(Error::new(ErrorKind::Protocol, "Wrong data type")),
            };
            let role = presence::from(data)
                .map_err(|_| Error::new(ErrorKind::Internal, "Couldn't parse capnp message"))?;
            println!("{:?}", role);
            Ok(())
        }
    })
    .unwrap();
}
