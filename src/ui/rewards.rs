use crate::points::{self, STICKERS};
use crate::state::use_app;
use dioxus::prelude::*;
use super::icon::Icon;

#[component]
pub fn RewardFace(id: String) -> Element {
    let ink = "#1f2933";
    let soft = "#7ba3c9";
    let leaf = "#2f6f4e";
    rsx! {
        svg {
            view_box: "0 0 120 90",
            match id.as_str() {
                "moon" => rsx! {
                    circle { cx: "58", cy: "45", r: "26", fill: "#f0d78c" }
                    circle { cx: "70", cy: "38", r: "20", fill: "#fffaf3" }
                },
                "flower" => rsx! {
                    circle { cx: "60", cy: "28", r: "12", fill: "#e7b3c3" }
                    circle { cx: "42", cy: "46", r: "12", fill: "#e7b3c3" }
                    circle { cx: "78", cy: "46", r: "12", fill: "#e7b3c3" }
                    circle { cx: "48", cy: "66", r: "12", fill: "#e7b3c3" }
                    circle { cx: "72", cy: "66", r: "12", fill: "#e7b3c3" }
                    circle { cx: "60", cy: "48", r: "10", fill: "#f0d78c" }
                },
                "bird" => rsx! {
                    path { d: "M20 58 Q60 20 100 50 Q70 48 60 58 Q78 62 96 54", fill: "none", stroke: "{soft}", stroke_width: "6", stroke_linecap: "round" }
                    circle { cx: "96", cy: "50", r: "4", fill: "{ink}" }
                },
                "fish" => rsx! {
                    ellipse { cx: "58", cy: "46", rx: "28", ry: "16", fill: "{soft}" }
                    polygon { points: "84,46 104,30 104,62", fill: "{soft}" }
                    circle { cx: "40", cy: "42", r: "2.5", fill: "{ink}" }
                },
                "kite" => rsx! {
                    polygon { points: "60,12 88,46 60,78 32,46", fill: "#e7b3c3", stroke: "{ink}", stroke_width: "2" }
                    line { x1: "60", y1: "78", x2: "70", y2: "88", stroke: "{ink}", stroke_width: "2" }
                },
                "tree" => rsx! {
                    rect { x: "54", y: "58", width: "12", height: "22", fill: "#8a6a45" }
                    circle { cx: "60", cy: "40", r: "22", fill: "{leaf}" }
                },
                "cloud" => rsx! {
                    circle { cx: "42", cy: "50", r: "14", fill: "{soft}" }
                    circle { cx: "60", cy: "42", r: "18", fill: "{soft}" }
                    circle { cx: "80", cy: "50", r: "14", fill: "{soft}" }
                },
                "leaf" => rsx! {
                    path { d: "M60 80 C20 50 30 16 60 14 C90 16 100 50 60 80 Z", fill: "{leaf}" }
                    line { x1: "60", y1: "20", x2: "60", y2: "74", stroke: "#fffaf3", stroke_width: "2" }
                },
                "boat" => rsx! {
                    path { d: "M18 52 H102 L88 70 H32 Z", fill: "{soft}" }
                    line { x1: "60", y1: "50", x2: "60", y2: "22", stroke: "{ink}", stroke_width: "3" }
                    path { d: "M60 24 L88 46 H60 Z", fill: "#f0d78c" }
                },
                _ => rsx! {
                    polygon { points: "60,10 70,40 102,40 76,58 86,90 60,70 34,90 44,58 18,40 50,40", fill: "#f0d78c", stroke: "{ink}", stroke_width: "2" }
                },
            }
        }
    }
}

#[component]
pub fn RewardBoard() -> Element {
    let mut app = use_app();
    let owned = app.store.read().stickers.clone();
    let points = app.store.read().points;
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(crate::state::Screen::Home),
                    Icon { name: "home" } "घर"
                }
                h1 { Icon { name: "star" } "इनाम" }
                span { class: "stars", "★ {points}" }
            }
            div { class: "rewards",
                for id in STICKERS {
                    if owned.iter().any(|kept| kept == id) {
                        div { key: "{id}",
                            RewardFace { id: id.to_string() }
                            p { class: "reward-name", "{points::sticker_label(id)}" }
                        }
                    } else {
                        div { key: "{id}", class: "empty-slot" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn StickerChoice(options: Vec<String>, on_pick: EventHandler<String>) -> Element {
    rsx! {
        div { class: "award",
            h1 { Icon { name: "star" } "एक इनाम चुनो" }
            div { class: "reward-row",
                for id in options {
                    button {
                        key: "{id}",
                        class: "reward-pick",
                        onclick: {
                            let id = id.clone();
                            move |_| on_pick.call(id.clone())
                        },
                        RewardFace { id: id.clone() }
                        p { class: "reward-name", "{points::sticker_label(&id)}" }
                    }
                }
            }
        }
    }
}
