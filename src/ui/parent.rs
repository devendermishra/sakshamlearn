use crate::lock;
use crate::state::{use_app, Screen};
use dioxus::prelude::*;
use super::icon::Icon;

#[component]
pub fn ParentGate() -> Element {
    let mut app = use_app();
    let mut entered = use_signal(String::new);
    let draft = use_signal(|| None::<String>);
    let unlocked = use_signal(|| false);
    let message = use_signal(|| {
        if app.store.read().pin.is_none() {
            "चार-अंकों का नया ताला बनाओ".to_string()
        } else {
            "चार-अंकों का ताला डालो".to_string()
        }
    });
    let mut store_sig = app.store;

    let dots = "◝".repeat(entered().len()) + &"○".repeat(4 - entered().len());
    let store = store_sig.read().clone();

    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Home),
                    Icon { name: "home" } "घर"
                }
                h1 { "अभिभावक" }
            }
            if !unlocked() {
                div { class: "parent",
                    p { class: "prompt", "{message}" }
                    p { class: "pin-dots", "{dots}" }
                    div { class: "keys",
                        for digit in 1..=9 {
                            button {
                                key: "{digit}",
                                class: "key",
                                onclick: move |_| push_digit(digit, entered, draft, unlocked, message, store_sig),
                                "{digit}"
                            }
                        }
                        button {
                            class: "key quiet",
                            onclick: move |_| entered.set(String::new()),
                            Icon { name: "eraser" } "मिटाओ"
                        }
                        button {
                            class: "key",
                            onclick: move |_| push_digit(0, entered, draft, unlocked, message, store_sig),
                            "0"
                        }
                    }
                }
            } else {
                div { class: "parent",
                    div { class: "toggle-line",
                        span { Icon { name: "volume-2" } "आवाज़" }
                        button {
                            class: "quiet",
                            onclick: move |_| {
                                let on = !store_sig.read().sound_on;
                                store_sig.write().sound_on = on;
                                store_sig.write().save();
                            },
                            if store.sound_on { "चालू" } else { "बंद" }
                        }
                    }
                    div { class: "toggle-line",
                        span { Icon { name: "hand" } "उंगली से लिखना" }
                        button {
                            class: "quiet",
                            onclick: move |_| {
                                let on = !store_sig.read().finger_ok;
                                store_sig.write().finger_ok = on;
                                store_sig.write().save();
                            },
                            if store.finger_ok { "चालू" } else { "बंद" }
                        }
                    }
                    div { class: "toggle-line",
                        span { Icon { name: "lock" } "ताला" }
                        button {
                            class: "quiet",
                            onclick: move |_| {
                                let on = !store_sig.read().lock_on;
                                store_sig.write().lock_on = on;
                                store_sig.write().save();
                                lock::apply(on);
                            },
                            if store.lock_on { "लगा है" } else { "खुला" }
                        }
                    }
                    p { class: "note",
                        "ब्राउज़र में यह ताला पूरी स्क्रीन छिपाता है और फ़ुलस्क्रीन माँगता है। टैबलेट पर लगा हुआ ऐप अन्य ऐप नहीं खोलने देता। उसके लिए एक बार Android सेटिंग में App pinning चालू करो, और Ask for PIN before unpinning चुनो।"
                    }
                }
            }
        }
    }
}

fn push_digit(
    digit: u8,
    mut entered: Signal<String>,
    mut draft: Signal<Option<String>>,
    mut unlocked: Signal<bool>,
    mut message: Signal<String>,
    mut store_sig: Signal<crate::points::Store>,
) {
    if unlocked() || entered().len() >= 4 {
        return;
    }
    entered.write().push(char::from(b'0' + digit));
    if entered().len() < 4 {
        return;
    }
    let code = entered();
    if let Some(saved) = store_sig.read().pin.clone() {
        if saved == code {
            unlocked.set(true);
        } else {
            message.set("फिर लिखो".to_string());
            entered.set(String::new());
        }
        return;
    }
    if let Some(first) = draft() {
        if first == code {
            store_sig.write().pin = Some(code);
            store_sig.write().save();
            unlocked.set(true);
            message.set("तैयार".to_string());
        } else {
            message.set(
                "दोनों बार एक समान लिखो"
                    .to_string(),
            );
            draft.set(None);
        }
        entered.set(String::new());
    } else {
        draft.set(Some(code));
        entered.set(String::new());
        message.set(
            "वही लिखो एक बार और".to_string(),
        );
    }
}
