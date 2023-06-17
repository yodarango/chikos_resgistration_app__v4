mod components;
mod routes;

use components::views::home::Home;
use yew_router::prelude::*;
use yew::prelude::*;
use routes::Route;


#[function_component(Main)]
fn app() -> Html {

    html! {
        <main>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </main>
    }
}
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::Admin => html! {
            <h1>{"Hello Admin"}</h1>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


fn main() {
    yew::Renderer::<Main>::new().render();
}