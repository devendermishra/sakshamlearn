use crate::content::aba::{self, CalQuestion};
use crate::points::{self, AwardOutcome};
use crate::speech;
use crate::state::{use_app, Screen};
use crate::ui::icon::Icon;
use crate::ui::quiz::Quiz;
use crate::ui::rewards::StickerChoice;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq)]
enum CalPhase {
    Teach,
    Quiz {
        index: u8,
        correct: u8,
    },
    Sticker {
        options: Vec<String>,
        gained: u32,
        correct: u8,
    },
    Done {
        gained: u32,
        correct: u8,
    },
}

#[component]
pub fn AbaMenu() -> Element {
    let mut app = use_app();
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Home),
                    Icon { name: "home" } "घर"
                }
                h1 { Icon { name: "calendar" } "ABA" }
            }
            p { class: "aim", "शिवेंद्र" }
            div { class: "doors",
                button {
                    class: "door",
                    onclick: move |_| app.screen.set(Screen::AbaCalendar),
                    Icon { name: "calendar" }
                    h2 { "कैलेंडर" }
                    p { "कल, आज, और आने वाला कल" }
                }
                button {
                    class: "door",
                    onclick: move |_| app.screen.set(Screen::AbaChecklist),
                    Icon { name: "list-checks" }
                    h2 { "घर की योजना" }
                    p { "आज के काम" }
                }
            }
        }
    }
}

#[component]
pub fn CalendarScreen() -> Element {
    let mut app = use_app();
    let lesson = use_signal(|| aba::calendar_lesson(aba::today()));
    let mut phase = use_signal(|| CalPhase::Teach);
    let cards = lesson.read().cards.clone();
    let month = lesson.read().month;
    let month_speak = lesson.read().month_speak.clone();
    let questions = lesson.read().questions.clone();
    let sound = app.store.read().sound_on;
    let mut store_sig = app.store;

    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Aba),
                    Icon { name: "arrow-left" } "वापस"
                }
                h1 { Icon { name: "calendar" } "कैलेंडर" }
            }
            match phase() {
                CalPhase::Teach => rsx! {
                    div { class: "day-cards",
                        for card in cards.iter().cloned() {
                            button {
                                key: "{card.label}",
                                class: if card.label == "आज" { "day-card today" } else { "day-card" },
                                onclick: move |_| speech::speak(&card.speak, sound),
                                p { class: "when", "{card.label}" }
                                p { class: "wday", "{card.weekday}" }
                                p { class: "dnum", "{card.day}" }
                                span { class: "hint", Icon { name: "volume-2" } "सुनो" }
                            }
                        }
                    }
                    p { class: "month-line", "आज का महीना: {month}" }
                    div { class: "actions",
                        button {
                            class: "quiet",
                            onclick: {
                                let month_speak = month_speak.clone();
                                move |_| speech::speak(&month_speak, sound)
                            },
                            Icon { name: "volume-2" } "सुनो"
                        }
                        button {
                            onclick: {
                                let questions = questions.clone();
                                let mut phase = phase;
                                move |_| {
                                    speak_at(&questions, 0, sound);
                                    phase.set(CalPhase::Quiz { index: 0, correct: 0 });
                                }
                            },
                            "अब पूछो"
                        }
                    }
                },
                CalPhase::Quiz { index, correct } => {
                    let question = questions.get(index as usize).cloned();
                    rsx! {
                        div { class: "dots-row",
                            for pip in 0..7u8 {
                                span { class: if pip <= index { "pip on" } else { "pip" } }
                            }
                        }
                        if let Some(question) = question {
                            button {
                                class: "quiet listen-again",
                                onclick: {
                                    let prompt = question.prompt;
                                    move |_| speech::speak(prompt, sound)
                                },
                                Icon { name: "volume-2" } "सुनो"
                            }
                            Quiz {
                                prompt: question.prompt.to_string(),
                                choices: question.choices.clone(),
                                on_pick: {
                                    let questions = questions.clone();
                                    move |pick| {
                                        let next_correct = correct + u8::from(pick == question.answer);
                                        let next = index + 1;
                                        if next as usize >= questions.len() {
                                            finish(next_correct, store_sig, phase);
                                        } else {
                                            speak_at(&questions, next as usize, sound);
                                            phase.set(CalPhase::Quiz { index: next, correct: next_correct });
                                        }
                                    }
                                },
                            }
                        }
                    }
                },
                CalPhase::Sticker { options, gained, correct } => rsx! {
                    StickerChoice {
                        options: options.clone(),
                        on_pick: move |picked: String| {
                            store_sig.write().keep_sticker(&picked);
                            store_sig.write().save();
                            phase.set(CalPhase::Done { gained, correct });
                        },
                    }
                },
                CalPhase::Done { gained, correct } => rsx! {
                    div { class: "award",
                        h1 { if gained > 0 { "हो गया" } else { "फिर एक बार" } }
                        p { class: "prompt", "सही {correct} / 7" }
                        if gained > 0 {
                            p { class: "plus", "+{gained}" }
                            p { "कुल ★ {store_sig.read().points}" }
                        }
                        div { class: "actions",
                            button {
                                onclick: move |_| phase.set(CalPhase::Teach),
                                Icon { name: "rotate-ccw" } "फिर से"
                            }
                            button {
                                class: "quiet",
                                onclick: move |_| app.screen.set(Screen::Aba),
                                Icon { name: "arrow-left" } "वापस"
                            }
                        }
                    }
                },
            }
        }
    }
}

fn speak_at(questions: &[CalQuestion], index: usize, sound: bool) {
    if let Some(question) = questions.get(index) {
        speech::speak(question.prompt, sound);
    }
}

fn finish(correct: u8, mut store_sig: Signal<points::Store>, phase: Signal<CalPhase>) {
    let bucket = match correct {
        6 | 7 => 3,
        4 | 5 => 2,
        _ => 0,
    };
    let outcome = {
        let mut store = store_sig.write();
        let outcome = store.award("aba:calendar", bucket, points::now_ms());
        store.save();
        outcome
    };
    show_outcome(outcome, correct, phase);
}

fn show_outcome(outcome: AwardOutcome, correct: u8, mut phase: Signal<CalPhase>) {
    if outcome.offer.is_empty() {
        phase.set(CalPhase::Done {
            gained: outcome.gained,
            correct,
        });
    } else {
        phase.set(CalPhase::Sticker {
            options: outcome.offer,
            gained: outcome.gained,
            correct,
        });
    }
}

#[component]
pub fn ChecklistScreen() -> Element {
    let mut app = use_app();
    let today = aba::today();
    let day_key = today.key();
    let today_line = today.line();
    let mut open = use_signal(|| None::<&'static str>);
    let store = app.store.read().clone();
    let mut store_sig = app.store;
    let groups = aba::plan_groups();
    let total: usize = groups.iter().map(|group| group.items.len()).sum();
    let done: usize = groups
        .iter()
        .flat_map(|group| group.items)
        .filter(|item| store.aba_has(&day_key, item.id))
        .count();
    let week: Vec<WeekDot> = (-6..=0)
        .map(|offset| {
            let date = today.add_days(offset);
            WeekDot {
                key: date.key(),
                day: date.day,
                name: aba::weekday_short(date.weekday()),
                hit: store.aba_any(&date.key()),
                current: offset == 0,
            }
        })
        .collect();

    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| app.screen.set(Screen::Aba),
                    Icon { name: "arrow-left" } "वापस"
                }
                h1 { Icon { name: "list-checks" } "घर की योजना" }
            }
            p { class: "aim", "{today_line}" }
            p { class: "section-title", "{done} / {total} आज हो गया" }
            div { class: "week-strip",
                for dot in week {
                    div {
                        key: "{dot.key}",
                        class: if dot.current { "week-day today" } else { "week-day" },
                        span { class: "wk", "{dot.name}" }
                        span { class: "dn", "{dot.day}" }
                        span { class: if dot.hit { "dot hit" } else { "dot" } }
                    }
                }
            }
            for group in groups {
                div { class: "plan-head",
                    h2 { "{group.title}" }
                    span {
                        class: "hint",
                        "{group.items.iter().filter(|item| store.aba_has(&day_key, item.id)).count()} / {group.items.len()}"
                    }
                }
                for item in group.items {
                    div {
                        key: "{item.id}",
                        class: "check-block",
                        div {
                            class: if store.aba_has(&day_key, item.id) { "check-row done" } else { "check-row" },
                            button {
                                class: "check-main",
                                onclick: {
                                    let id = item.id;
                                    move |_| {
                                        open.set(if open() == Some(id) { None } else { Some(id) });
                                    }
                                },
                                "{item.title}"
                            }
                            button {
                                class: "check-tick",
                                onclick: {
                                    let id = item.id;
                                    let day_key = day_key.clone();
                                    move |_| {
                                        store_sig.write().aba_toggle(&day_key, id);
                                        store_sig.write().save();
                                    }
                                },
                                if store.aba_has(&day_key, item.id) {
                                    Icon { name: "check" }
                                }
                                "हो गया"
                            }
                        }
                        if open() == Some(item.id) {
                            ol { class: "check-steps",
                                for (step_i, step) in item.steps.iter().enumerate() {
                                    li { key: "{item.id}-{step_i}", "{step}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct WeekDot {
    key: String,
    day: u8,
    name: &'static str,
    hit: bool,
    current: bool,
}
