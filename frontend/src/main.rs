use gloo::console::log;
use gloo_net::http::Request;
use itertools::Itertools;
use serde::Deserialize;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::{ops::Deref, sync::Arc};
use wasm_bindgen::*;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

//TODO: we should import this from the indexer
#[derive(Debug, Clone, Deserialize)]
struct CrawledResource {
    url: String,
    priority: u32, //how do we even calculate this
    word: Arc<String>,
}

//We implement PartialEq, Eq and Hash to ignore the priority field.
impl PartialEq for CrawledResource {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.word == other.word
    }
}
impl Eq for CrawledResource {}

impl PartialOrd for CrawledResource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CrawledResource {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Hash for CrawledResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.word.hash(state);
    }
}

#[derive(Debug, Clone)]
struct State {
    pub search_query: String,
    pub results: Vec<CrawledResource>, //TODO: some loading?
}

#[function_component(OSSE)]
fn osse() -> Html {
    let state = use_state(|| State {
        search_query: "".to_string(),
        results: vec![],
    });

    let display_results = |results: &Vec<CrawledResource>| -> Html {
        results
            .iter()
            .sorted()
            .map(|r| {
                html! {
                    <div key={r.url.to_owned()}>
                        <a href={r.url.to_owned()}>{r.url.to_owned()}{"--"}{r.priority}</a>
                    </div>
                }
            })
            .collect::<Html>()
    };

    let search_query_changed = {
        let cloned_state = state.clone();
        Callback::from(move |event: InputEvent| {
            let target: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");
            let input = target.unchecked_into::<HtmlInputElement>().value();
            log!("Input changed: {}", &input);
            let mut state = cloned_state.deref().clone();
            state.search_query = input;
            cloned_state.set(state);
        })
    };

    let on_submit = {
        let cloned_state = state.clone();

        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let state = cloned_state.deref().clone();

            //fetch
            {
                let cloned_state = cloned_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut state = cloned_state.deref().clone();
                    let endpoint = format!("http://127.0.0.1:4444/search/{}", &state.search_query);

                    let fetched_results = Request::get(endpoint.as_str()).send().await.unwrap();

                    let fetched_json: Vec<CrawledResource> = match fetched_results.json().await {
                        Err(e) => panic!("Im panic: {}", e),
                        Ok(json) => json,
                    };

                    state.results = fetched_json;

                    cloned_state.set(state);
                });
            }

            log!("Submit: {}", state.search_query);
        })
    };

    let curr_state = state.deref().to_owned();

    html! {
        <>
            <header>
                <nav class="navbar bg-light sticky-top">
                    <div class="container-fluid">
                        <div>
                            <a href="https://github.com/Baitinq/OSSE" class="navbar-brand mb-0 h1 mx-2">{"OSSE"}</a>
                            <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                        </div>
                        <a href="https://github.com/Baitinq" class="navbar-text mb-0">{"Made by Baitinq"}</a>
                    </div>
                </nav>
            </header>
            <main class="container d-flex h-100">
                <div class="row align-self-center w-100">
                    <div class="col">
                            <b class="display-4">{"OSSE"}</b>
                            <p>{"Your favorite independent search engine."}</p>
                            <form onsubmit={on_submit}>
                                <div class="input-group input-group-lg my-2">
                                    <input oninput={search_query_changed} value={curr_state.search_query}type="text" class="form-control" placeholder="Search with OSSE" />
                                    <button class="btn btn-primary" type="submit" >{"Search!"}</button>
                                </div>
                            </form>
                            <section>
                                {display_results(&curr_state.results)}
                            </section>
                    </div>
                </div>
            </main>
            <footer>
                <nav class="navbar bg-light bottom">
                    <div class="container-fluid">
                        <div>
                            <a href="https://github.com/Baitinq/OSSE" class="navbar-brand mb-0 h1 mx-2">{"OSSE"}</a>
                            <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                        </div>
                        <a href="https://github.com/Baitinq" class="navbar-text mb-0">{"Made by Baitinq"}</a>
                    </div>
                </nav>
            </footer>
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
