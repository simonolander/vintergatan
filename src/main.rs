use leptos::prelude::*;
use crate::app::App;

mod model;
mod app;
mod state;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
