use std::rc::Rc;
use yew::prelude::*;

pub const DEFAULT_WIDTH: u32 = 100;
pub const DEFAULT_HEIGHT: u32 = 100;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameContext {
    pub width: u32,
    pub height: u32,
    pub field: Vec<Vec<bool>>,
}

pub enum ContextAction {
    SetSize((u32, u32)),
    SetCell((u32, u32, bool)),
    Iterate,
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
            ContextAction::Iterate => {
                state.field = crate::game::iterate_field(&state.field);
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
        field: crate::game::init_field(DEFAULT_WIDTH, DEFAULT_HEIGHT),
    });

    html! {
        <ContextProvider<GameContextHandle> context={context}>
            {props.children.clone()}
        </ContextProvider<GameContextHandle>>
    }
}
