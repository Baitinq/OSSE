mod app;

use app::OSSE;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    OSSEHome,
    #[at("/search/")]
    OSSEHomeEmptySearch,
    #[at("/search/:query")]
    OSSESearch { query: String },
}

fn switch_routes(routes: Route) -> Html {
    let location = window().unwrap().location();
    let api_endpoint = format!(
        "{}//{}:{}/api",
        location.protocol().unwrap(),
        location.hostname().unwrap(),
        4444
    );
    match routes {
        Route::OSSEHome | Route::OSSEHomeEmptySearch => html! {
            <OSSE api_endpoint={api_endpoint} initial_search_query={None as Option<String>} />
        },
        Route::OSSESearch { query } => html! {
            <OSSE api_endpoint={api_endpoint} initial_search_query={Some(query)} />
        },
    }
}

#[function_component(App)]
fn yew_app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch_routes} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
