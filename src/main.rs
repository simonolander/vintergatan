use crate::app::App;

mod app;
mod model;
mod state;
mod ui;

fn main() {
    console_error_panic_hook::set_once();
    App::new().expect("Failed to create application");
}
