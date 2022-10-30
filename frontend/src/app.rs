use gloo_net::http::Request;
use itertools::Itertools;
use wasm_bindgen::*;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use lib::lib::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct ResultComponentProps {
    result: IndexedResource,
}

#[function_component(ResultComponent)]
fn result_component(props: &ResultComponentProps) -> Html {
    html! {
        <div>
            <a href={props.result.url.clone()}>
                {props.result.url.clone()}{"--"}{props.result.title.clone()}{"----"}{props.result.description.clone()}{format!("PRIO: {}", props.result.priority)}
            </a>
        </div>
    }
}

pub struct OSSE {
    pub search_query: String,
    pub results: Option<Vec<IndexedResource>>, //TODO: some loading?
}

#[derive(Properties, PartialEq, Eq)]
pub struct OSSEProps {
    pub api_endpoint: String,
}

pub enum OSSEMessage {
    SearchSubmitted,
    SearchChanged(String),
    SearchFinished(Vec<IndexedResource>),
}

impl Component for OSSE {
    type Message = OSSEMessage;
    type Properties = OSSEProps;

    fn create(ctx: &Context<Self>) -> Self {
        OSSE {
            search_query: "".to_string(),
            results: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            OSSEMessage::SearchSubmitted => {
                let search_query = self.search_query.clone();
                let api_endpoint = ctx.props().api_endpoint.clone();
                ctx.link().send_future(async move {
                    let endpoint = format!("{}/search/{}", api_endpoint, search_query);

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
        let onsubmit = ctx.link().callback(|event: SubmitEvent| {
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

//Your favorite search engine in navbar
//Search in middle