use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Game,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn router(routes: Route) -> Html {
    match routes {
        Route::Game => html! { <crate::views::game::Game /> },
        Route::NotFound => html! { <crate::views::not_found::NotFound /> },
    }
}
