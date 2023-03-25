use yew::prelude::*;
use crate::components::config_panel::*;
use crate::components::game_field::*;

#[function_component]
pub fn Game() -> Html {
    html! {
        <div>
            <ConfigPanel />
            <GameField />
        </div>
    }
}
