use yew::prelude::*;
use gloo_timers::callback::Interval;
use std::cell::RefCell;
use crate::context::*;

pub enum Msg {
    Run,
    Stop,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub context: GameContextHandle,
}

// Using a struct component to make the timer lifetime easier to deal with.

pub struct RunPanel {
    timer: RefCell<Option<Interval>>,
}

impl Component for RunPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        RunPanel {
            timer: RefCell::new(None),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let run = ctx.link().callback(|_| Msg::Run);
        let stop = ctx.link().callback(|_| Msg::Stop);

        html! {
            <>
                <button onclick={run}>{"Run"}</button>
                <button onclick={stop}>{"Stop"}</button>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Run => {
                let context = ctx.props().context.clone();
                self.timer = RefCell::new(Some(Interval::new(100, move || {
                    context.dispatch(ContextAction::Iterate);
                })));
            },
            Msg::Stop => {
                let old_timer = self.timer.replace(None);
                match old_timer {
                    Some(timer) => { timer.cancel(); },
                    None => {},
                };
            },
        };

        false
    }
}
