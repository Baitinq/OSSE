use gloo::console::log;
use std::ops::Deref;
use wasm_bindgen::*;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug, Clone)]
struct State {
    pub search_query: String,
}

#[function_component(OSSE)]
fn osse() -> Html {
    let state = use_state(|| State {
        search_query: "".to_string(),
    });

    let cloned_state = state.clone();
    let search_query_changed = Callback::from(move |event: InputEvent| {
        let target: EventTarget = event
            .target()
            .expect("Event should have a target when dispatched");
        let input = target.unchecked_into::<HtmlInputElement>().value();
        log!("Input changed: {}", &input);
        let mut state = cloned_state.deref().clone();
        state.search_query = input;
        cloned_state.set(state);
    });

    let cloned_state = state.clone();
    let on_submit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let mut state = cloned_state.deref().clone();
        log!("Submit:{}", state.search_query);
        state.search_query = "".to_string();
        cloned_state.set(state);
    });

    let curr_state = state.deref().to_owned();

    html! {
        <>
        <nav class="navbar bg-light sticky-top">
            <div class="container-fluid">
                <div>
                    <a href="https://github.com/Baitinq/OSSE" class="navbar-brand mb-0 h1 mx-2">{"OSSE"}</a>
                    <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                </div>
                <a href="https://github.com/Baitinq" class="navbar-text mb-0">{"Made by Baitinq"}</a>
            </div>
        </nav>
        <div class="container d-flex h-100">
            <div class="row align-self-center w-100">
                <div class="col">
                        <b class="display-4 text-truncate">{"OSSE"}</b>
                        <p>{"Your favorite independent search engine."}</p>
                        <form onsubmit={on_submit}>
                            <div class="input-group input-group-lg my-2">
                                <input oninput={search_query_changed} value={curr_state.search_query}type="text" class="form-control" placeholder="Search with OSSE" />
                                <button class="btn btn-primary" type="submit" >{"Search!"}</button>
                            </div>
                        </form>
                </div>
            </div>
        </div>
        <nav class="navbar bg-light bottom">
            <div class="container-fluid">
                <div>
                    <a href="https://github.com/Baitinq/OSSE" class="navbar-brand mb-0 h1 mx-2">{"OSSE"}</a>
                    <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                </div>
                <a href="https://github.com/Baitinq" class="navbar-text mb-0">{"Made by Baitinq"}</a>
            </div>
        </nav>
    </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <OSSE/>
    </>
    }
}

//Your favorite search engine in navbar
//Search in middle

fn main() {
    yew::start_app::<App>();
}
