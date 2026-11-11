use dioxus::prelude::*;

#[component]
pub fn Quiz(prompt: String, choices: Vec<String>, on_pick: EventHandler<usize>) -> Element {
    rsx! {
        div {
            p { class: "prompt", "{prompt}" }
            div { class: "choices",
                for (index, choice) in choices.iter().cloned().enumerate() {
                    button {
                        key: "{index}-{choice}",
                        class: if choice.chars().count() > 6 { "choice long" } else { "choice" },
                        onclick: move |_| on_pick.call(index),
                        "{choice}"
                    }
                }
            }
        }
    }
}
