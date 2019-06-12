extern crate ws;

use carlos::presence;
use std::cell::Cell;
use std::rc::Rc;
use ws::{listen, Error, ErrorKind, Message};

fn main() {
    let vehicle_online = Rc::new(Cell::new(false));
    listen("0.0.0.0:3000", |out| {
        let online = vehicle_online.clone();
        move |msg: Message| {
            let role = presence::from(msg.into_data())
                .map_err(|_| Error::new(ErrorKind::Internal, "Couldn't parse capnp message"))?;
            println!("{:?}", role);
            match role {
                presence::Role::Vehicle => {
                    online.set(true);
                }
                presence::Role::Driver => {
                    if online.get() {
                        out.send("online!")?;
                    }
                }
            }
            Ok(())
        }
    })
    .unwrap();
}
