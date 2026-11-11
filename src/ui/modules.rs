use crate::content::{self, Goal, Mode};
use crate::state::{use_app, Screen};
use dioxus::prelude::*;
use crate::ui::icon::{goal_icon, Icon};

#[component]
pub fn ClassMenu(subject: u8) -> Element {
    let mut app = use_app();
    let open = subject == 0;
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button { class: "back", onclick: move |_| app.screen.set(Screen::Home), Icon { name: "home" } "घर" }
                h1 { "हिन्दी" }
            }
            if open {
                div { class: "doors",
                    button {
                        class: "door",
                        onclick: move |_| app.screen.set(Screen::Goals { subject, class_no: 1 }),
                        Icon { name: "flower-2" } h2 { "कक्षा 1" }
                        p { "एक वाक्य: रीना पौधे को पानी देती है।" }
                    }
                    button {
                        class: "door",
                        onclick: move |_| app.screen.set(Screen::Goals { subject, class_no: 2 }),
                        Icon { name: "sprout" } h2 { "कक्षा 2" }
                        p { "तीन चित्र, क्रम, और जुड़े वाक्य।" }
                    }
                }
            } else {
                p { class: "hint", "बाद में" }
            }
        }
    }
}

#[component]
pub fn GoalMenu(subject: u8, class_no: u8) -> Element {
    let mut app = use_app();
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Classes { subject }),
                    Icon { name: "arrow-left" } "वापस"
                }
                h1 { "कक्षा {class_no}" }
            }
            for goal in content::GOALS {
                div { class: "module", key: "{content::goal_key(goal)}",
                    h2 { Icon { name: goal_icon(goal) } "{content::goal_title(goal)}" }
                    p { class: "aim", "{content::goal_aim(class_no, goal)}" }
                    div { class: "actions",
                        button {
                            onclick: move |_| app.screen.set(Screen::Bank { subject, class_no, goal, mode: Mode::Learn }),
                            Icon { name: "sprout" } "सीखो"
                        }
                        button {
                            onclick: move |_| app.screen.set(Screen::Bank { subject, class_no, goal, mode: Mode::Practice }),
                            Icon { name: "list-checks" } "अभ्यास"
                        }
                        button {
                            onclick: move |_| app.screen.set(Screen::Exam { subject, class_no, goal, nonce: 0 }),
                            Icon { name: "clipboard-check" } "परीक्षा"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BankPage(subject: u8, class_no: u8, goal: Goal, mode: Mode) -> Element {
    let mut app = use_app();
    let sections = content::bank(class_no, goal);
    let heading = match mode {
        Mode::Learn => "सीखो",
        Mode::Practice => "अभ्यास",
    };
    let title = content::goal_title(goal);
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Goals { subject, class_no }),
                    Icon { name: "arrow-left" } "वापस"
                }
                h1 { Icon { name: goal_icon(goal) } "{heading} · {title}" }
            }
            for (section_i, section) in sections.iter().enumerate() {
                div { key: "s-{section_i}", class: "section",
                    h2 { "{section.title}" }
                    div { class: "grid",
                        for item in section.items.iter() {
                            button {
                                key: "{item.id}",
                                class: "tile",
                                onclick: {
                                    let id = item.id.clone();
                                    move |_| app.screen.set(Screen::Session { id: id.clone(), mode, nonce: 0 })
                                },
                                "{item.title}"
                            }
                        }
                    }
                }
            }
        }
    }
}
