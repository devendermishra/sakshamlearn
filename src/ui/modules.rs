use crate::content::{self, Goal, Lang, Mode};
use crate::state::{use_app, Screen};
use crate::ui::icon::{goal_icon, Icon};
use dioxus::prelude::*;

#[component]
pub fn ClassMenu(subject: u8) -> Element {
    let mut app = use_app();
    let math = subject == 2;
    let lang = if math {
        Lang::from_store(&app.store.read().math_lang)
    } else {
        Lang::Hi
    };
    let open = subject == 0 || math;
    let mut store_sig = app.store;
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button { class: "back", onclick: move |_| app.screen.set(Screen::Home), Icon { name: "home" } "{content::phrase(\"home\", lang)}" }
                h1 { "{content::subject_heading(subject, lang)}" }
            }
            if math {
                div { class: "lang",
                    button {
                        class: if lang == Lang::Hi { "on" } else { "" },
                        onclick: move |_| {
                            store_sig.write().math_lang = "hi".to_string();
                            store_sig.write().save();
                        },
                        "हिन्दी"
                    }
                    button {
                        class: if lang == Lang::En { "on" } else { "" },
                        onclick: move |_| {
                            store_sig.write().math_lang = "en".to_string();
                            store_sig.write().save();
                        },
                        "English"
                    }
                }
            }
            if open {
                div { class: "doors",
                    button {
                        class: "door",
                        onclick: move |_| app.screen.set(Screen::Goals { subject, class_no: 1 }),
                        Icon { name: "flower-2" } h2 { "{content::phrase(\"class\", lang)} 1" }
                        p { "{content::class_blurb(subject, 1, lang)}" }
                    }
                    button {
                        class: "door",
                        onclick: move |_| app.screen.set(Screen::Goals { subject, class_no: 2 }),
                        Icon { name: "sprout" } h2 { "{content::phrase(\"class\", lang)} 2" }
                        p { "{content::class_blurb(subject, 2, lang)}" }
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
    let lang = if subject == 2 {
        Lang::from_store(&app.store.read().math_lang)
    } else {
        Lang::Hi
    };
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Classes { subject }),
                    Icon { name: "arrow-left" } "{content::phrase(\"back\", lang)}"
                }
                h1 { "{content::phrase(\"class\", lang)} {class_no}" }
            }
            for goal in content::goals_for(subject) {
                div { class: "module", key: "{content::goal_key(*goal)}",
                    h2 { Icon { name: goal_icon(*goal) } "{content::goal_title_in(*goal, lang)}" }
                    p { class: "aim", "{content::goal_aim_in(class_no, *goal, lang)}" }
                    div { class: "actions",
                        button {
                            onclick: move |_| app.screen.set(Screen::Bank { subject, class_no, goal: *goal, mode: Mode::Learn }),
                            Icon { name: "sprout" } "{content::phrase(\"learn\", lang)}"
                        }
                        button {
                            onclick: move |_| app.screen.set(Screen::Bank { subject, class_no, goal: *goal, mode: Mode::Practice }),
                            Icon { name: "list-checks" } "{content::phrase(\"practice\", lang)}"
                        }
                        button {
                            onclick: move |_| app.screen.set(Screen::Exam { subject, class_no, goal: *goal, nonce: 0 }),
                            Icon { name: "clipboard-check" } "{content::phrase(\"exam\", lang)}"
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
    let lang = if subject == 2 {
        Lang::from_store(&app.store.read().math_lang)
    } else {
        Lang::Hi
    };
    let sections = content::bank(class_no, goal, lang);
    let heading = content::phrase(
        match mode {
            Mode::Learn => "learn",
            Mode::Practice => "practice",
        },
        lang,
    );
    let title = content::goal_title_in(goal, lang);
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Goals { subject, class_no }),
                    Icon { name: "arrow-left" } "{content::phrase(\"back\", lang)}"
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
