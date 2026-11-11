// Icons are Lucide (https://lucide.dev), ISC license.
// Simple single-color outlines: the same stroke, the same size, and always beside the word.
use dioxus::prelude::*;

#[component]
pub fn Icon(name: &'static str) -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            {icon_body(name)}
        }
    }
}

pub fn subject_icon(index: u8) -> &'static str {
    match index {
        0 => "book-open-text",
        1 => "languages",
        _ => "calculator",
    }
}

pub fn goal_icon(goal: crate::content::Goal) -> &'static str {
    use crate::content::Goal;
    match goal {
        Goal::Listen => "ear",
        Goal::Letters => "a-large-small",
        Goal::Fluency => "book-open",
        Goal::Meaning => "book-open-check",
        Goal::Writing => "pencil",
    }
}

fn icon_body(name: &str) -> Element {
    match name {
        "a-large-small" => rsx! { path { d: "m15 16 2.536-7.328a1.02 1.02 1 0 1 1.928 0L22 16" } path { d: "M15.697 14h5.606" } path { d: "m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16" } path { d: "M3.304 13h6.392" } },
        "arrow-left" => rsx! { path { d: "m12 19-7-7 7-7" } path { d: "M19 12H5" } },
        "arrow-right" => rsx! { path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } },
        "book-heart" => rsx! { path { d: "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" } path { d: "M8.62 9.8A2.25 2.25 0 1 1 12 6.836a2.25 2.25 0 1 1 3.38 2.966l-2.626 2.856a.998.998 0 0 1-1.507 0z" } },
        "book-open-check" => rsx! { path { d: "M12 21V7" } path { d: "m16 12 2 2 4-4" } path { d: "M22 6V4a1 1 0 0 0-1-1h-5a4 4 0 0 0-4 4 4 4 0 0 0-4-4H3a1 1 0 0 0-1 1v13a1 1 0 0 0 1 1h6a3 3 0 0 1 3 3 3 3 0 0 1 3-3h6a1 1 0 0 0 1-1v-1.3" } },
        "book-open-text" => rsx! { path { d: "M12 5v16" } path { d: "M16 13h2" } path { d: "M16 9h2" } path { d: "M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z" } path { d: "M6 13h2" } path { d: "M6 9h2" } },
        "book-open" => rsx! { path { d: "M12 5v16" } path { d: "M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z" } },
        "calculator" => rsx! { rect { x: "4", y: "2", width: "16", height: "20", rx: "2" } line { x1: "8", x2: "16", y1: "6", y2: "6" } line { x1: "16", x2: "16", y1: "14", y2: "18" } path { d: "M16 10h.01" } path { d: "M12 10h.01" } path { d: "M8 10h.01" } path { d: "M12 14h.01" } path { d: "M8 14h.01" } path { d: "M12 18h.01" } path { d: "M8 18h.01" } },
        "clipboard-check" => rsx! { rect { x: "8", y: "2", width: "8", height: "4", rx: "1", ry: "1" } path { d: "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" } path { d: "m9 14 2 2 4-4" } },
        "ear" => rsx! { path { d: "M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0" } path { d: "M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4" } },
        "eraser" => rsx! { path { d: "M21 21H8a2 2 0 0 1-1.42-.587l-3.994-3.999a2 2 0 0 1 0-2.828l10-10a2 2 0 0 1 2.829 0l5.999 6a2 2 0 0 1 0 2.828L12.834 21" } path { d: "m5.082 11.09 8.828 8.828" } },
        "eye" => rsx! { path { d: "M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0" } circle { cx: "12", cy: "12", r: "3" } },
        "flower-2" => rsx! { path { d: "M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1" } circle { cx: "12", cy: "8", r: "2" } path { d: "M12 10v12" } path { d: "M12 22c4.2 0 7-1.667 7-5-4.2 0-7 1.667-7 5Z" } path { d: "M12 22c-4.2 0-7-1.667-7-5 4.2 0 7 1.667 7 5Z" } },
        "hand" => rsx! { path { d: "M18 11V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2" } path { d: "M14 10V4a2 2 0 0 0-2-2a2 2 0 0 0-2 2v2" } path { d: "M10 10.5V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2v8" } path { d: "M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" } },
        "home" => rsx! { path { d: "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8" } path { d: "M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" } },
        "image" => rsx! { rect { x: "3", y: "3", width: "18", height: "18", rx: "2", ry: "2" } circle { cx: "9", cy: "9", r: "2" } path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" } },
        "languages" => rsx! { path { d: "m5 8 6 6" } path { d: "m4 14 6-6 2-3" } path { d: "M2 5h12" } path { d: "M7 2h1" } path { d: "m22 22-5-10-5 10" } path { d: "M14 18h6" } },
        "list-checks" => rsx! { path { d: "M13 5h8" } path { d: "M13 12h8" } path { d: "M13 19h8" } path { d: "m3 17 2 2 4-4" } path { d: "m3 7 2 2 4-4" } },
        "lock-open" => rsx! { rect { x: "3", y: "11", width: "18", height: "11", rx: "2", ry: "2" } path { d: "M7 11V7a5 5 0 0 1 9.9-1" } },
        "lock" => rsx! { rect { x: "3", y: "11", width: "18", height: "11", rx: "2", ry: "2" } path { d: "M7 11V7a5 5 0 0 1 10 0v4" } },
        "notebook-pen" => rsx! { path { d: "M13.4 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-7.4" } path { d: "M2 6h4" } path { d: "M2 10h4" } path { d: "M2 14h4" } path { d: "M2 18h4" } path { d: "M21.378 5.626a1 1 0 1 0-3.004-3.004l-5.01 5.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z" } },
        "pause" => rsx! { rect { x: "14", y: "3", width: "5", height: "18", rx: "1" } rect { x: "5", y: "3", width: "5", height: "18", rx: "1" } },
        "pencil" => rsx! { path { d: "M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" } path { d: "m15 5 4 4" } },
        "play" => rsx! { path { d: "M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z" } },
        "rotate-ccw" => rsx! { path { d: "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" } path { d: "M3 3v5h5" } },
        "school" => rsx! { path { d: "M14 21v-3a2 2 0 0 0-4 0v3" } path { d: "M18 4.933V21" } path { d: "m4 6 7.106-3.79a2 2 0 0 1 1.788 0L20 6" } path { d: "m6 11-3.52 2.147a1 1 0 0 0-.48.854V19a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a1 1 0 0 0-.48-.853L18 11" } path { d: "M6 4.933V21" } circle { cx: "12", cy: "9", r: "2" } },
        "sprout" => rsx! { path { d: "M14 9.536V7a4 4 0 0 1 4-4h1.5a.5.5 0 0 1 .5.5V5a4 4 0 0 1-4 4 4 4 0 0 0-4 4c0 2 1 3 1 5a5 5 0 0 1-1 3" } path { d: "M4 9a5 5 0 0 1 8 4 5 5 0 0 1-8-4" } path { d: "M5 21h14" } },
        "star" => rsx! { path { d: "M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z" } },
        "volume-2" => rsx! { path { d: "M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z" } path { d: "M16 9a5 5 0 0 1 0 6" } path { d: "M19.364 18.364a9 9 0 0 0 0-12.728" } },
        "volume-x" => rsx! { path { d: "M11 4.702a.7.7 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.7.7 0 0 0 11 19.298z" } path { d: "m16.5 14.5 5-5" } path { d: "m16.5 9.5 5 5" } },
        _ => rsx! {},
    }
}
