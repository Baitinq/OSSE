use gloo::console::log;
use std::hash::{Hash, Hasher};
use std::{ops::Deref, sync::Arc};
use wasm_bindgen::*;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

//TODO: we should import this from the indexer
#[derive(Debug, Clone)]
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
        state.results = vec![
            CrawledResource {
                url: "http://example.com".to_string(),
                priority: 12,
                word: Arc::new("example".to_string()),
            },
            CrawledResource {
                url: "http://test.com".to_string(),
                priority: 17,
                word: Arc::new("test".to_string()),
            },
        ];
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
                        <b class="display-4">{"OSSE"}</b>
                        <p>{"Your favorite independent search engine."}</p>
                        <form onsubmit={on_submit}>
                            <div class="input-group input-group-lg my-2">
                                <input oninput={search_query_changed} value={curr_state.search_query}type="text" class="form-control" placeholder="Search with OSSE" />
                                <button class="btn btn-primary" type="submit" >{"Search!"}</button>
                            </div>
                        </form>
                        <section>
                            {curr_state.results.into_iter().map(|r| {
                                html!{
                                    <div key={r.url.to_owned()}>{ format!("Result: {:?}!", r) }</div>
                                }
                            }).collect::<Html>()}
                        </section>
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
