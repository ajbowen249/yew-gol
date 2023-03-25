use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn router(routes: Route) -> Html {
    match routes {
        Route::Counter => html! { <crate::views::game::Game /> },
        Route::NotFound => html! { <crate::views::not_found::NotFound /> },
    }
}
