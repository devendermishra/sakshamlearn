use crate::content::{self, Group, Mode};
use crate::points;
use crate::state::{use_app, Screen};
use dioxus::prelude::*;
use crate::ui::icon::Icon;

#[component]
pub fn Home() -> Element {
    let app = use_app();
    let store = app.store.read().clone();
    let suggestion = {
        let ids = content::all_practice_ids();
        points::suggest(&store, &ids)
    };
    let suggestion_title = suggestion
        .as_ref()
        .and_then(|id| content::find(id))
        .map(|item| item.title);
    let mut pressed = use_signal(|| None::<u64>);
    let mut screen = app.screen;
    let mut store_sig = app.store;
    let lock_icon = if store.lock_on { "lock" } else { "lock-open" };

    rsx! {
        div { class: "stage",
            div { class: "topbar",
                button {
                    class: "title",
                    onpointerdown: move |_| pressed.set(Some(points::now_ms())),
                    onpointerup: move |_| {
                        if pressed().map(|start| points::now_ms().saturating_sub(start) > 650).unwrap_or(false) {
                            screen.set(Screen::Parent);
                        }
                        pressed.set(None);
                    },
                    onpointerleave: move |_| pressed.set(None),
                    "सक्षम"
                }
                span { class: "stars", Icon { name: "star" } "{store.points}" }
                span { class: if store.lock_on { "lock on" } else { "lock" }, Icon { name: lock_icon } "ताला" }
                button {
                    class: "ghost",
                    onclick: move |_| {
                        let on = !store_sig.read().sound_on;
                        store_sig.write().sound_on = on;
                        store_sig.write().save();
                    },
                    if store.sound_on { Icon { name: "volume-2" } "आवाज़" } else { Icon { name: "volume-x" } "शांत" }
                }
            }
            if let Some(title) = suggestion_title.clone() {
                button {
                    class: "suggest",
                    onclick: {
                        let suggestion = suggestion.clone();
                        let mut screen = screen;
                        move |_| {
                            if let Some(id) = suggestion.clone() {
                                screen.set(Screen::Session { id, mode: Mode::Practice, nonce: 0 });
                            }
                        }
                    },
                    Icon { name: "list-checks" } "आज का अभ्यास: {title}"
                }
            }
            div { class: "doors",
                for subject in content::subjects() {
                    button {
                        key: "{subject.index}",
                        class: "door",
                        disabled: !subject.open,
                        onclick: move |_| {
                            if subject.open {
                                screen.set(Screen::Classes { subject: subject.index });
                            }
                        },
                        Icon { name: crate::ui::icon::subject_icon(subject.index) } h2 { "{subject.title}" }
                        if subject.open {
                            p { "कक्षा 1 और कक्षा 2" }
                        } else {
                            p { "बाद में" }
                        }
                    }
                }
            }
            div { class: "tiles",
                button {
                    class: "tile",
                    onclick: move |_| screen.set(Screen::Rewards),
                    Icon { name: "star" } small { "इनाम" }
                    "तस्वीरें"
                }
            }
        }
    }
}

#[component]
fn Tile(label: &'static str, hint: &'static str, group: Group) -> Element {
    let mut app = use_app();
    rsx! {
        button {
            class: "tile",
            onclick: move |_| app.screen.set(Screen::Group { group, mode: Mode::Learn }),
            small { "{hint}" }
            "{label}"
        }
    }
}

#[component]
pub fn SkillMenu(mode: Mode) -> Element {
    let mut app = use_app();
    let heading = match mode {
        Mode::Learn => "सीखो",
        Mode::Practice => "अभ्यास",
    };
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button { class: "back", onclick: move |_| app.screen.set(Screen::Home), Icon { name: "home" } "घर" }
                h1 { "{heading}" }
            }
            div { class: "tiles",
                for (label, hint, group) in [
                    ("रेखा", "लेखन", Group::Pattern),
                    ("अ आ", "अक्षर", Group::Akshar),
                    ("का कि", "मात्रा", Group::Matra),
                    ("शब्द", "शब्द", Group::Word),
                    ("पढ़ो", "सारंगी", Group::Reading),
                ] {
                    button {
                        key: "{hint}-{label}",
                        class: "tile",
                        onclick: move |_| app.screen.set(Screen::Group { group, mode }),
                        small { "{hint}" }
                        "{label}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn GroupPage(group: Group, mode: Mode) -> Element {
    let mut app = use_app();
    let mut current = use_signal(|| mode);
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button { class: "back", onclick: move |_| app.screen.set(Screen::Home), Icon { name: "home" } "घर" }
                h1 { "{group_name(group)}" }
            }
            div { class: "modes",
                button {
                    class: if current() == Mode::Learn { "on" } else { "" },
                    onclick: move |_| current.set(Mode::Learn),
                    "सीखो"
                }
                button {
                    class: if current() == Mode::Practice { "on" } else { "" },
                    onclick: move |_| current.set(Mode::Practice),
                    "अभ्यास"
                }
            }
            match group {
                Group::Pattern => rsx! { ItemButtons { items: content::pattern_items(), mode: current() } },
                Group::Akshar => rsx! {
                    p { class: "section-title", "स्वर" }
                    ItemButtons { items: content::swar_items(), mode: current() }
                    p { class: "section-title", "व्यंजन" }
                    ItemButtons { items: content::vyanjan_items(), mode: current() }
                },
                Group::Matra => rsx! {
                    div { class: "grid",
                        for index in 0..content::vyanjan_len() {
                            button {
                                key: "{index}",
                                class: "cell",
                                onclick: move |_| {
                                    app.screen.set(Screen::Matra { consonant: index, mode: current() });
                                },
                                "{content::vyanjan_label(index).unwrap_or(\"\")}"
                            }
                        }
                    }
                },
                Group::Word => rsx! {
                    for (name, items) in content::word_sections() {
                        p { class: "section-title", key: "{name}", "{name}" }
                        ItemButtons { items: items, mode: current() }
                    }
                },
                Group::Reading => rsx! { ItemButtons { items: content::reading_items(), mode: current() } },
            }
        }
    }
}

#[component]
pub fn MatraPage(consonant: usize, mode: Mode) -> Element {
    let mut app = use_app();
    let mut current = use_signal(|| mode);
    let label = content::vyanjan_label(consonant).unwrap_or("");
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Group { group: Group::Matra, mode: current() }),
                    "मात्रा"
                }
                h1 { "{label}" }
            }
            div { class: "modes",
                button {
                    class: if current() == Mode::Learn { "on" } else { "" },
                    onclick: move |_| current.set(Mode::Learn),
                    "सीखो"
                }
                button {
                    class: if current() == Mode::Practice { "on" } else { "" },
                    onclick: move |_| current.set(Mode::Practice),
                    "अभ्यास"
                }
            }
            ItemButtons { items: content::barakhadi_items(consonant), mode: current() }
        }
    }
}

#[component]
fn ItemButtons(items: Vec<content::Item>, mode: Mode) -> Element {
    let mut app = use_app();
    rsx! {
        div { class: "grid",
            for item in items {
                button {
                    key: "{item.id}",
                    class: "cell",
                    onclick: {
                        let id = item.id.clone();
                        move |_| {
                            app.screen.set(Screen::Session { id: id.clone(), mode, nonce: 0 });
                        }
                    },
                    "{item.title}"
                }
            }
        }
    }
}

fn group_name(group: Group) -> &'static str {
    match group {
        Group::Pattern => "रेखा",
        Group::Akshar => "अक्षर",
        Group::Matra => "मात्रा",
        Group::Word => "शब्द",
        Group::Reading => "पढ़ो",
    }
}
