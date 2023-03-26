use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::*;

#[function_component]
pub fn NotFound() -> Html {
    let navigator = use_navigator().unwrap();
    navigator.push(&Route::Game);

    html! {
        <div>
            { "Not found!" }
        </div>
    }
}
