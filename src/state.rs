use crate::content::{Goal, Group, Mode};
use crate::points::Store;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Screen {
    Home,
    Classes { subject: u8 },
    Goals { subject: u8, class_no: u8 },
    Bank { subject: u8, class_no: u8, goal: Goal, mode: Mode },
    Exam { subject: u8, class_no: u8, goal: Goal, nonce: u32 },
    #[allow(dead_code)]
    Skills { mode: Mode },
    Group { group: Group, mode: Mode },
    Matra { consonant: usize, mode: Mode },
    Session { id: String, mode: Mode, nonce: u32 },
    Rewards,
    Parent,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub store: Signal<Store>,
    pub screen: Signal<Screen>,
}

pub fn use_app() -> AppState {
    use_context::<AppState>()
}
