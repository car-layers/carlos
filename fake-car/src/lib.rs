extern crate carlos;
extern crate wasm_bindgen;
extern crate web_sys;

use carlos::presence::{self, Role};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    console, window, CanvasRenderingContext2d, Event, HtmlCanvasElement, SvgImageElement, WebSocket,
};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let ctx = document
        .get_element_by_id("world")
        .expect("canvas element not found")
        .unchecked_into::<HtmlCanvasElement>()
        .get_context("2d")?
        .expect("failed getting 2d context")
        .unchecked_into::<CanvasRenderingContext2d>();
    let oscar = document
        .query_selector("#world img")?
        .expect("img element not found")
        .unchecked_into::<SvgImageElement>();

    ctx.draw_image_with_svg_image_element(&oscar, 0.0, 0.0)?;

    // telling server I'm alive
    let p = presence::who(Role::Vehicle, "Oscar");
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

    console::log_1(&"Oscar says hi!".into());
    Ok(())
}
