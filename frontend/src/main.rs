mod app;

use yew::prelude::*;
use yew_router::prelude::*;
use app::OSSE;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    OSSEHome,
    #[at("/search/:query")]
    OSSESearch { query: String },
}

fn switch_routes(routes: Route) -> Html {
    match routes {
        Route::OSSEHome => html! {
            <OSSE api_endpoint={"http://127.0.0.1:4444"}/>
        },
        Route::OSSESearch { query } => html! {
            <OSSE api_endpoint={"http://127.0.0.1:4444"}/>
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
