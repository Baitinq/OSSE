use crate::Route;
use gloo_net::http::Request;
use itertools::Itertools;
use lib::lib::*;
use stylist::style;
use wasm_bindgen::*;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct ResultComponentProps {
    result: IndexedResource,
}

#[function_component(ResultComponent)]
fn result_component(props: &ResultComponentProps) -> Html {
    let style = style!(
        r#"
        a {
            text-decoration: none;
        }

        .url {
            font-size: 0.75em;
        }

        .title {
            font-size: 1.25em;
        }
        .title:hover {
            text-decoration: underline;
        }

        .description {
            font-size: 1em;
        }
    "#
    )
    .unwrap();
    let style = style.get_class_name().to_owned();
    html! {
        <div class={format!("mb-4 {}", style)}>
            <a href={props.result.url.clone()}>
                <p class="url text-muted mb-0">{props.result.url.clone()}</p>
                <p class="title mb-1">{props.result.title.clone()}</p>
            </a>
            <p class="description">
                {match props.result.description.clone().as_str() {
                    "" => "No Description.",
                    otherwise => otherwise,
                }}{format!("PRIO: {}", props.result.priority)}
            </p>
        </div>
    }
}

pub struct OSSE {
    pub search_query: String,
    pub results: Option<Result<Vec<IndexedResource>, String>>, //TODO: some loading?
}

#[derive(Properties, PartialEq, Eq)]
pub struct OSSEProps {
    pub api_endpoint: String,
    pub initial_search_query: Option<String>,
}

pub enum OSSEMessage {
    SearchSubmitted,
    SearchChanged(String),
    SearchFinished(Result<Vec<IndexedResource>, String>),
}

impl Component for OSSE {
    type Message = OSSEMessage;
    type Properties = OSSEProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut search_query = String::from("");

        //we push an SearchSubmitted message if inital_search_query is not none
        if let Some(initial_search_query) = ctx.props().initial_search_query.clone() {
            search_query = initial_search_query;
            ctx.link().send_message(OSSEMessage::SearchSubmitted);
        }

        //WE may have data race between the future and the actual creation.
        OSSE {
            search_query: urlencoding::decode(search_query.as_str())
                .to_owned()
                .unwrap()
                .to_string(),
            results: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            OSSEMessage::SearchSubmitted => {
                let api_endpoint = ctx.props().api_endpoint.clone();
                let search_query = self.search_query.clone();
                let navigator = ctx.link().navigator().unwrap();

                navigator.push(&Route::OSSESearch {
                    query: urlencoding::encode(search_query.as_str()).to_string(),
                });

                ctx.link().send_future(async move {
                    let endpoint = format!("{}/search/{}", api_endpoint, search_query);

                    let fetched_response = match Request::get(endpoint.as_str()).send().await {
                        Ok(response) => response,
                        Err(_) => {
                            return OSSEMessage::SearchFinished(Err(
                                "Failed to connect to the API!".to_string(),
                            ))
                        }
                    };

                    let fetched_results: Vec<IndexedResource> = match fetched_response.json().await
                    {
                        Err(_) => {
                            return OSSEMessage::SearchFinished(Err(
                                "Internal API Error!".to_string()
                            ))
                        }
                        Ok(json) => json,
                    };

                    OSSEMessage::SearchFinished(Ok(fetched_results))
                });

                false
            }
            OSSEMessage::SearchChanged(search_query) => {
                self.search_query = search_query;

                true
            }
            OSSEMessage::SearchFinished(search_results) => {
                match search_results {
                    Ok(results) => {
                        self.results = Some(Ok(results));
                    }
                    Err(error) => {
                        self.results = Some(Err(error));
                    }
                };

                true
            }
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

        let display_results =
            |maybe_results: &Option<Result<Vec<IndexedResource>, String>>| -> Html {
                if maybe_results.is_none() {
                    return html! {};
                }

                let results = maybe_results.as_ref().unwrap();

                if results.is_err() {
                    return html! {
                        <p>{format!("ERROR: {}", results.as_ref().err().unwrap())}</p>
                    };
                }

                let results = results.as_ref().unwrap();

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
                            <a href="/" class="navbar-brand h1 mx-2">{"OSSE"}</a>
                            <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                        </div>
                        <a href="https://github.com/Baitinq/OSSE" class="navbar-text">{"Made by Baitinq"}</a>
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
                            <a href="/" class="navbar-brand h1 mx-2">{"OSSE"}</a>
                            <span class="navbar-text mb-0">{"| Your favorite independent search engine."}</span>
                        </div>
                        <a href="https://github.com/Baitinq/OSSE" class="navbar-text">{"Made by Baitinq"}</a>
                    </div>
                </nav>
            </footer>
        </>
        }
    }
}

//Your favorite search engine in navbar
//Search in middle
