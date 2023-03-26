use yew::prelude::*;
use crate::util::{ bind_on_input, bind_on_input_with_effect };
use crate::context::*;

#[function_component]
pub fn ConfigPanel() -> Html {
    let context = use_context::<crate::context::GameContextHandle>().unwrap();
    let width_input = use_state(|| String::from(crate::context::DEFAULT_WIDTH.to_string()));
    let height_input = use_state(|| String::from(crate::context::DEFAULT_HEIGHT.to_string()));
    let interval_input = use_state(|| String::from(crate::context::DEFAULT_INTERVAL.to_string()));

    let apply = {
        let width_input = width_input.clone();
        let height_input = height_input.clone();
        let context = context.clone();

        move |_| {
            context.dispatch(ContextAction::SetSize((
                width_input.parse::<u32>().unwrap(),
                height_input.parse::<u32>().unwrap()
            )))
        }
    };

    let iterate = {
        let context = context.clone();

        move |_| {
            context.dispatch(ContextAction::Iterate);
        }
    };

    let speed_ctx = context.clone();
    let change_speed = bind_on_input_with_effect(&interval_input, Callback::from(move |value: String| {
        speed_ctx.dispatch(ContextAction::SetIterval(
            value.parse::<u32>().unwrap()
        ));
    }));

    html! {
        <div class={classes!("config-panel")}>
            <label>{ "Width" }</label>
            <input min={0} type={"number"} value={(*width_input).clone()} oninput={bind_on_input(&width_input)} /><br />

            <label>{ "Height" }</label>
            <input min={0} type={"number"} value={(*height_input).clone()} oninput={bind_on_input(&height_input)} /><br />

            <label>{ "Interval" }</label>
            <input
                type={"range"}
                min={0}
                max={500}
                value={(*interval_input).clone()}
                oninput={change_speed}
            />
            { (*interval_input).clone() }{ "ms" }
            <br />

            <button onclick={apply}>
                { "Apply" }
            </button>

            <button onclick={iterate}>
                { "Iterate" }
            </button>
        </div>
    }
}
