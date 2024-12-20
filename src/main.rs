use crate::app::app;

mod app;
mod model;
mod state;

fn main() {
    console_error_panic_hook::set_once();
    app().unwrap();
}
