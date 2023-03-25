use yew::prelude::*;
use crate::context::*;

const HTML_NBSP: &str = "\u{00a0}";

#[function_component]
pub fn GameField() -> Html {
    let context = use_context::<crate::context::GameContextHandle>().unwrap();

    let mut x = 0;
    let mut y = 0;

    html! {
        <div class={classes!("game-field")}>
        {
            ((*context).clone()).field.into_iter().map(|row| {

                let row_val = html! {
                    <div class={classes!("game-row")}>
                    {
                        row.into_iter().map(|cell| {
                            let mut cell_classes = classes!("game-cell");
                            if cell {
                                cell_classes.push("game-cell-alive");
                            }

                            let onclick = {
                                let context = context.clone();

                                move |_| {
                                    context.dispatch(ContextAction::SetCell((
                                        x,
                                        y,
                                        !cell
                                    )));
                                }
                            };

                            let cell_val = html! {
                                <div class={cell_classes} onclick={onclick}>
                                    {HTML_NBSP}
                                </div>
                            };

                            x = x + 1;

                            cell_val
                        }).collect::<Html>()
                    }
                    </div>
                };

                y = y + 1;
                x = 0;

                row_val
            }).collect::<Html>()
        }
        </div>
    }
}
