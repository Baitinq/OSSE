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
pub struct IndexedResource {
    url: String,
    title: String,
    description: String,
    priority: u32,
    word: Arc<String>,
}

//We implement PartialEq, Eq and Hash to ignore the priority field.
impl PartialEq for IndexedResource {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.word == other.word
    }
}
impl Eq for IndexedResource {}

impl PartialOrd for IndexedResource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IndexedResource {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl Hash for IndexedResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.word.hash(state);
    }
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct ResultComponentProps {
    result: IndexedResource,
}

#[function_component(ResultComponent)]
fn result_component(props: &ResultComponentProps) -> Html {
    html! {
        <a href={props.result.url.clone()}>
            {props.result.url.clone()}{"--"}{props.result.title.clone()}{"----"}{props.result.description.clone()}{format!("PRIO: {}", props.result.priority)}
        </a>
    }
}

pub struct OSSE {
    pub search_query: String,
    pub results: Option<Vec<IndexedResource>>, //TODO: some loading?
}

pub enum OSSEMessage {
    SearchSubmitted,
    SearchChanged(String),
    SearchFinished(Vec<IndexedResource>),
}

impl Component for OSSE {
    type Message = OSSEMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        OSSE {
            search_query: "".to_string(),
            results: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            OSSEMessage::SearchSubmitted => {
                let search_query = self.search_query.clone();
                ctx.link().send_future(async move {
                    let endpoint = format!("http://127.0.0.1:4444/search/{}", search_query);

                    let fetched_response = Request::get(endpoint.as_str()).send().await.unwrap();

                    let fetched_results: Vec<IndexedResource> = match fetched_response.json().await {
                        Err(e) => panic!("Im panic: {}", e),
                        Ok(json) => json,
                    };

                    OSSEMessage::SearchFinished(fetched_results)
                });
                
                false
            },
            OSSEMessage::SearchChanged(search_query) => {
                self.search_query = search_query;

                true
            },
            OSSEMessage::SearchFinished(search_results) => {
                self.results = Some(search_results);

                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();

            OSSEMessage::SearchSubmitted
        });

        let oninput = ctx.link().callback(|event: InputEvent| {
            let target: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");
            let input = target.unchecked_into::<HtmlInputElement>().value();

            OSSEMessage::SearchChanged(input)
        });

        let display_results = |maybe_results: &Option<Vec<IndexedResource>>| -> Html {
            let maybe_results = maybe_results.as_ref();
            if maybe_results.is_none() {
                return html! {};
            }

            let results = maybe_results.unwrap();
            if !results.is_empty() {
                results
                    .iter()
                    .sorted()
                    .map(|r| {
                        html! {
                            <div key={r.url.to_owned()}>
                                <ResultComponent result={r.clone()} />
                            </div>
                        }
                    })
                    .collect::<Html>()
            } else {
                html! {
                    <p>{"No results!"}</p>
                }
            }
        };

        html! {
            <>
            <header>
                <nav class="navbar bg-light sticky-top">
                    <div class="container-fluid">
                        <div>
                            <a href="https://github.com/Baitinq/OSSE" class="navbar-brand h1 mx-2">{"OSSE"}</a>
                            <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                        </div>
                        <a href="https://github.com/Baitinq" class="navbar-text">{"Made by Baitinq"}</a>
                    </div>
                </nav>
            </header>
            <main style="display: flex; flex-direction: column; min-height: 100vh; align-items: center; justify-content: center;">
                //SET AT MIDDLE OF VIEWPORT IF NO SEARCHING AND TOP 25% IF SEARCHING
                <div class="container">
                    <div class="row">
                        <div class="col">
                                <section class="my-5">
                                    <b class="display-4">{"OSSE"}</b>
                                    <p>{"Your favorite independent search engine."}</p>
                                    <form {onsubmit}>
                                        <div class="input-group input-group-lg my-2">
                                            <input {oninput} value={self.search_query.clone()}type="text" class="form-control" placeholder="Search with OSSE" />
                                            <button class="btn btn-primary" type="submit" >{"Search!"}</button>
                                        </div>
                                    </form>
                                </section>
                                <section>
                                    {display_results(&self.results)}
                                </section>
                        </div>
                    </div>
                </div>
            </main>
            <footer class="mt-5">
                <nav class="navbar bg-light">
                    <div class="container-fluid">
                        <div>
                            <a href="https://github.com/Baitinq/OSSE" class="navbar-brand h1 mx-2">{"OSSE"}</a>
                            <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                        </div>
                        <a href="https://github.com/Baitinq" class="navbar-text">{"Made by Baitinq"}</a>
                    </div>
                </nav>
            </footer>
        </>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <OSSE />
    </>
    }
}

//Your favorite search engine in navbar
//Search in middle

fn main() {
    yew::start_app::<App>();
}
