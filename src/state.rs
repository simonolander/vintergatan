use crate::state::State::{Initial, Loaded, Loading};
use std::rc::Rc;
use yew::Reducible;

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Initial,
    Loading,
    Loaded(LoadedState),
}

#[derive(Debug, Eq, PartialEq)]
pub struct LoadedState {}

pub enum Action {
    StartLoading,
    FinishedLoading(LoadedState),
}

impl Default for State {
    fn default() -> Self {
        Initial
    }
}

impl Reducible for State {
    type Action = Action;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::StartLoading => Loading.into(),
            Action::FinishedLoading(loaded_state) => Loaded(loaded_state).into(),
        }
    }
}
