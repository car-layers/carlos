extern crate carlos;
extern crate wasm_bindgen;
extern crate web_sys;

use carlos::presence::{self, Role};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Event, MessageEvent, WebSocket};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let p = presence::who(Role::Driver, "Driver");
    let socket = WebSocket::new("ws://192.168.0.3:3000")?;
    let on_conn = Closure::once_into_js(move |e: Event| {
        let mut p = presence::pack(p.into_inner()).unwrap();
        e.target()
            .expect("event has no target")
            .unchecked_into::<WebSocket>()
            .send_with_u8_array(&mut p)
            .unwrap();
    });
    let on_msg = Closure::once_into_js(move |e: MessageEvent| {
        console::log_1(&e.data());
    });
    socket.add_event_listener_with_callback("open", on_conn.unchecked_ref())?;
    socket.add_event_listener_with_callback("message", on_msg.unchecked_ref())?;
    Ok(())
}
