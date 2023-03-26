use yew::prelude::*;
use web_sys::HtmlSelectElement;
use crate::util::bind_on_input;
use crate::context::*;
use std::rc::Rc;

#[function_component]
pub fn PatternPanel() -> Html {
    let patterns = vec![
        (
            "Gilder",
            "It Glides",
            vec![
                vec![ false, false, true, ],
                vec![ true, false, true, ],
                vec![ false, true, true, ],
            ],
        ),
    ];

    let context = use_context::<crate::context::GameContextHandle>().unwrap();
    let select_node_ref = use_node_ref();

    let apply = {
        let select_node_ref = select_node_ref.clone();
        let context = context.clone();
        let patterns = patterns.clone();

        move |_| {
            if let Some(element) = select_node_ref.cast::<HtmlSelectElement>() {
                let index = element.value().parse::<i32>().unwrap();

                if index >= 0 {
                    context.dispatch(ContextAction::OverridePattern(patterns[index as usize].2.clone()));
                }
            }
        }
    };

    let mut pattern_index = 0;

    html! {
        <div class={classes!("pattern-panel")}>

            <select ref={select_node_ref}>
                <option value={"-1"}>{ "None" }</option>
                {
                    patterns.into_iter().map(|pattern| {

                        let html = html! {
                            <option value={pattern_index.to_string()}>{pattern.0}</option>
                        };

                        pattern_index = pattern_index + 1;

                        return html;
                    }).collect::<Html>()
                }
            </select>

            <button onclick={apply}>
                { "Apply" }
            </button>
        </div>
    }
}
