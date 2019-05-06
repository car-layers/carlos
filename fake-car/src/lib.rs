extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement, SvgImageElement};

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
    console::log_1(&"Oscar says hi!".into());
    Ok(())
}
