use yew_router::prelude::*;
use yew::prelude::*;

pub mod views;
pub mod router;
pub mod components;
pub mod context;
pub mod util;
pub mod game;

#[function_component]
fn App() -> Html {
    html! {
        <context::GameContextProvider>
            <BrowserRouter>
                <Switch<router::Route> render={router::router} />
            </BrowserRouter>
        </context::GameContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
