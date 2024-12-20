use crate::state::State::{Initial, Loaded, Loading};
use crate::state::{LoadedState, State};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Game(state: ReadSignal<LoadedState>) -> impl IntoView {
    None::<String>
}

#[component]
pub fn App() -> impl IntoView {
    let state = RwSignal::new(State::default());

    Effect::new(move |_| {
        if let Initial = state.get() {
            state.set(Loading);
            spawn_local(
                async move {
                    gloo_timers::future::TimeoutFuture::new(1000).await;
                    let loaded_state = LoadedState::generate(10);
                    state.set(Loaded(loaded_state));
                }
            );
        }
    });

    view! {
        <Show when=move || !state.get().is_loaded()>
            <p>{"Loading..."}</p>
        </Show>

    }
}
