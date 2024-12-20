use std::cell::RefCell;
use std::rc::Rc;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, MouseEvent};

pub fn app() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let p = document.create_element("p")?;
    body.append_child(&p)?;

    let button = document.create_element("button")?;
    button.set_text_content(Some("Click me!"));
    body.append_child(&button)?;

    let counter = Rc::new(RefCell::new(0));
    p.set_text_content(Some(&(*counter).borrow().to_string()));

    let p = Rc::new(p);
    {
        let p = Rc::clone(&p);
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            *counter.borrow_mut() += 1;
            p.set_text_content(Some(&(*counter).borrow().to_string()));
        });
        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
