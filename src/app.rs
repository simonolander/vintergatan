use crate::state::Action::{FinishedLoading, StartLoading};
use crate::state::{LoadedState, State};
use yew::platform::spawn_local;
use yew::prelude::*;

async fn load_state() -> LoadedState {
    // Simulate a delay and return the loaded state
    use wasm_timer::Delay;
    Delay::new(std::time::Duration::from_secs(5)).await.unwrap();

    LoadedState {
        // Populate the loaded state fields
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(State::default);

    let load = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(StartLoading))
    };

    {
        let state = state.clone();
        use_effect_with(state, |state| {

        });
        // use_effect(|| {
        //     // if let State::Loading = **state {
        //         spawn_local(async move {
        //             let loaded_state = load_state().await;
        //             state.dispatch(FinishedLoading(loaded_state));
        //         });
        //     // }
        //     || {}
        // });
    }

    html! {
        <>
            {
                match *state {
                    State::Initial => {html!(<button onclick={load}>{"Load"}</button>)}
                    State::Loading => {html!(<p>{"Loading..."}</p>)}
                    State::Loaded(_) => {html!(<button onclick={load}>{"Reload"}</button>)}
                }
            }
        </>
    }
}
