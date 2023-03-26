use yew::prelude::*;
use crate::components::config_panel::*;
use crate::components::game_field::*;
use crate::components::pattern_panel::*;
use crate::components::run_panel::*;

#[function_component]
pub fn Game() -> Html {
    let context = use_context::<crate::context::GameContextHandle>().unwrap();

    html! {
        <div class={classes!("game-main")}>
            <div class={classes!("top-bar")}>
                <div><ConfigPanel /></div>
                <div><RunPanel {context} /></div>
                <div><PatternPanel /></div>
            </div>
            <GameField />
        </div>
    }
}
