use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| 0);

    let incr = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decr = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html! {
        <main>
            <p>{"Current count:"} { *state }</p>
            <button onclick={incr}>{"+"}</button>
            <button onclick={decr}>{"-"}</button>
        </main>
    }
}
