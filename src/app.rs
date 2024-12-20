use crate::model::border::Border;
use crate::model::position::Position;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Element, Event, HtmlElement, MouseEvent};

pub struct App {
    counter: i32,
    button: Element,
}

impl App {
    pub fn new() -> Result<Rc<RefCell<Self>>, JsValue> {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        let button = document.create_element("button")?;
        button.set_inner_html("Click me!");
        body.append_child(&button)?;

        let app = App {
            counter: 0,
            button: button.clone(),
        };

        let app = Rc::new(RefCell::new(app));

        {
            let app_ref = Rc::clone(&app);
            let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
                let mut app = app_ref.borrow_mut();
                app.on_click();
            });
            button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(app)
    }

    fn on_click(&mut self) {
        self.counter += 1;
        self.button.set_text_content(Some(&self.counter.to_string()));
    }
}
