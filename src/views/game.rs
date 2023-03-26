use yew::prelude::*;
use crate::components::config_panel::*;
use crate::components::game_field::*;
use crate::components::pattern_panel::*;
use crate::components::run_panel::*;

#[function_component]
pub fn Game() -> Html {
    let context = use_context::<crate::context::GameContextHandle>().unwrap();

    html! {
        <div>
            <ConfigPanel />
            <RunPanel {context} />
            <PatternPanel />
            <GameField />
        </div>
    }
}
