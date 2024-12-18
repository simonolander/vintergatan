use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Document, HtmlElement};
use crate::model::board::Board;
use crate::model::game::Game;
use crate::model::universe::Universe;

pub mod model;

fn render_generating(body: &HtmlElement, document: &Document) -> Result<(), JsValue> {
    body.set_inner_html("");
    let p = document.create_element("p")?;
    p.set_text_content(Some("Generating..."));
    body.append_child(&p)?;
    Ok(())
}

fn init(body: &HtmlElement, document: &Document) -> Result<(), JsValue> {
    body.set_inner_html("");
    // render_generating(body, document)?;
    // let universe = Universe::generate(10, 10);
    let board = Board::new(10, 10);
    let mut game = Game::new();
    game.start()?;
    // let board = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    // board.set_id("board");
    // body.append_child(&board)?;

    Ok(())
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    init(&body, &document)?;
    Ok(())
}
