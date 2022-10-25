use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
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
                        <form>
                            <div class="input-group input-group-lg my-2">
                                <input type="text" class="form-control" placeholder="Search with OSSE" />
                                <button class="btn btn-primary" type="button" >{"Search"}</button>
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

//Your favorite search engine in navbar
//Search in middle

fn main() {
    yew::start_app::<App>();
}
