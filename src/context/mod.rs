use std::rc::Rc;
use yew::prelude::*;

pub const DEFAULT_WIDTH: usize = 100;
pub const DEFAULT_HEIGHT: usize = 100;
pub const DEFAULT_INTERVAL: u32 = 100;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameContext {
    pub width: usize,
    pub height: usize,
    pub interval: u32,
    pub field: Vec<Vec<bool>>,
}

pub enum ContextAction {
    SetSize((usize, usize)),
    SetCell((u32, u32, bool)),
    SetIterval(u32),
    Iterate,
    OverridePattern(Vec<Vec<bool>>),
}

impl Reducible for GameContext {
    type Action = ContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            ContextAction::SetSize((width, height)) => {
                state.width = width;
                state.height = height;
                state.field = crate::game::init_field(width, height);
            },
            ContextAction::SetCell((x, y, value)) => {
                state.field[y as usize][x as usize] = value;
            },
            ContextAction::SetIterval(interval) => {
                state.interval = interval;
            },
            ContextAction::Iterate => {
                state.field = crate::game::iterate_field(&state.field);
            },
            ContextAction::OverridePattern(value) => {
                if value.len() > state.height as usize || value[0].len() > state.width {
                    state.width = value[0].len();
                    state.height = value.len();
                }

                state.field = crate::game::init_field(state.width, state.height);

                for row in 0..value.len() {
                    for col in 0..value[row].len() {
                        state.field[row][col] = value[row][col];
                    }
                }
            },
        }

        state.into()
    }
}

pub type GameContextHandle = UseReducerHandle<GameContext>;

#[derive(Properties, Debug, PartialEq)]
pub struct GameContextProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn GameContextProvider(props: &GameContextProviderProps) -> Html {
    let context = use_reducer(|| GameContext {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        interval: DEFAULT_INTERVAL,
        field: crate::game::init_field(DEFAULT_WIDTH, DEFAULT_HEIGHT),
    });

    html! {
        <ContextProvider<GameContextHandle> context={context}>
            {props.children.clone()}
        </ContextProvider<GameContextHandle>>
    }
}
