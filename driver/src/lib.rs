extern crate carlos;
extern crate wasm_bindgen;
extern crate web_sys;

use carlos::presence::{self, Role};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, WebSocket};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let p = presence::who(Role::Driver, "Driver");
    let socket = WebSocket::new("ws://192.168.0.3:3000")?;
    let cb = Closure::once_into_js(move |e: Event| {
        let mut p = presence::pack(p.into_inner()).unwrap();
        e.target()
            .expect("event has no target")
            .unchecked_into::<WebSocket>()
            .send_with_u8_array(&mut p)
            .unwrap();
    });
    socket.add_event_listener_with_callback("open", cb.unchecked_ref())?;
    Ok(())
}
