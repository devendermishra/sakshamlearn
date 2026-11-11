mod content;
mod exam;
mod lock;
mod points;
mod score;
mod session;
mod speech;
mod state;
mod ui;

use dioxus::prelude::*;
use state::{AppState, Screen};

const CSS: Asset = asset!("/assets/app.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let store = use_signal(points::Store::load);
    let screen = use_signal(|| Screen::Home);
    let mut ready = use_signal(|| false);
    use_context_provider(|| AppState { store, screen });
    use_effect(move || {
        let snapshot = store();
        if ready() {
            snapshot.save();
        } else {
            ready.set(true);
        }
    });
    use_effect(move || {
        lock::apply(store().lock_on);
    });

    rsx! {
        document::Stylesheet { href: CSS }
        Shell {}
    }
}

#[component]
fn Shell() -> Element {
    let app = use_app_screen();
    let screen = app.screen.read().clone();
    rsx! {
        div { class: "app",
            match screen {
                Screen::Home => rsx! { ui::home::Home {} },
                Screen::Classes { subject } => rsx! { ui::modules::ClassMenu { subject } },
                Screen::Goals { subject, class_no } => rsx! { ui::modules::GoalMenu { subject, class_no } },
                Screen::Bank { subject, class_no, goal, mode } => rsx! {
                    ui::modules::BankPage { subject, class_no, goal, mode }
                },
                Screen::Exam { subject, class_no, goal, nonce } => rsx! {
                    exam::ExamScreen {
                        key: "{subject}-{class_no}-{nonce}-{content::goal_key(goal)}",
                        subject,
                        class_no,
                        goal,
                        nonce,
                    }
                },
                Screen::Skills { mode } => rsx! { ui::home::SkillMenu { mode } },
                Screen::Group { group, mode } => rsx! { ui::home::GroupPage { group, mode } },
                Screen::Matra { consonant, mode } => rsx! { ui::home::MatraPage { consonant, mode } },
                Screen::Session { id, mode, nonce } => rsx! {
                    session::SessionScreen { key: "{id}-{nonce}", id, mode }
                },
                Screen::Rewards => rsx! { ui::rewards::RewardBoard {} },
                Screen::Parent => rsx! { ui::parent::ParentGate {} },
            }
        }
    }
}

fn use_app_screen() -> state::AppState {
    use_context()
}
