mod app;
mod model;
mod state;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}