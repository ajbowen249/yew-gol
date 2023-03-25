use yew::prelude::*;
use gloo_timers::callback::Interval;
use std::rc::Rc;
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
    timer: Option<Interval>,
    is_running: Rc<RefCell<bool>>,
}

impl Component for RunPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        RunPanel {
            timer: None,
            is_running: Rc::new(RefCell::new(false)),
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
                match self.timer {
                    Some(_) => {},
                    None => {
                        let is_running = self.is_running.clone();
                        let context = ctx.props().context.clone();
                        self.timer = Some(Interval::new(100, move || {
                            if *is_running.borrow() {
                                context.dispatch(ContextAction::Iterate);
                            }
                        }));
                    },
                };

                *(self.is_running.borrow_mut()) = true;
            },
            Msg::Stop => {
                *(self.is_running.borrow_mut()) = false;
            },
        };

        false
    }
}
