use crate::content::{self, Goal};
use crate::points;
use crate::speech::speak;
use crate::state::{use_app, Screen};
use crate::ui::quiz::Quiz;
use crate::ui::rewards::StickerChoice;
use crate::ui::trace::sample_glyph;
use crate::ui::writing_board::WritingBoard;
use crate::ui::icon::Icon;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum Step {
    Write { fade: u8, blank: bool },
    Ask { index: usize, correct: u8 },
    Fail,
    Sticker { options: Vec<String>, gained: u32 },
    Done { gained: u32 },
}

#[component]
pub fn ExamScreen(subject: u8, class_no: u8, goal: Goal, nonce: u32) -> Element {
    let paper = content::paper(class_no, goal);
    let questions = paper.questions.clone();
    let write_text = paper.write.clone();
    let lines = content::story(class_no);
    let opening = write_text.clone().unwrap_or_default();
    let passage = lines.join(" ");
    let prompts: Vec<String> = questions.iter().map(|q| q.prompt.clone()).collect();
    let mut step = use_signal(|| {
        if write_text.is_some() {
            Step::Write { fade: 0, blank: false }
        } else {
            Step::Ask { index: 0, correct: 0 }
        }
    });
    let mut dots = use_signal(Vec::<(f32, f32)>::new);
    let mut reveal = use_signal(|| false);
    let app = use_app();
    let sound = app.store.read().sound_on;
    let finger_ok = app.store.read().finger_ok;
    let mut store_sig = app.store;
    let mut screen = app.screen;
    let exam_id = format!("exam:{class_no}:{}", content::goal_key(goal));
    let show_story = goal != Goal::Letters;

    {
        let opening = opening.clone();
        let passage = passage.clone();
        let prompts = prompts.clone();
        use_effect(move || {
            let spoken = match step() {
                Step::Write { fade: 0, blank: false } => opening.clone(),
                Step::Ask { index, .. } => {
                    let mut text = String::new();
                    if index == 0 && !passage.is_empty() && show_story {
                        text.push_str(&passage);
                        text.push(' ');
                    }
                    if let Some(prompt) = prompts.get(index) {
                        text.push_str(prompt);
                    }
                    text
                }
                _ => String::new(),
            };
            speak(&spoken, sound);
        });
    }
    {
        let opening = opening.clone();
        use_effect(move || {
            let opening = opening.clone();
            if opening.is_empty() {
                return;
            }
            spawn(async move {
                dots.set(sample_glyph(&opening).await);
            });
        });
    }

    let mut finish = {
        let exam_id = exam_id.clone();
        move |correct: u8| {
            let now = points::now_ms();
            let mapped = if correct >= 4 { 3 } else if correct >= 3 { 2 } else { 0 };
            if mapped == 0 {
                store_sig.write().mark_retry(&exam_id, now);
                store_sig.write().save();
                step.set(Step::Fail);
            } else {
                let outcome = store_sig.write().award(&exam_id, mapped, now);
                store_sig.write().save();
                if outcome.offer.is_empty() {
                    step.set(Step::Done { gained: outcome.gained });
                } else {
                    step.set(Step::Sticker { options: outcome.offer, gained: outcome.gained });
                }
            }
        }
    };

    let title = content::goal_title(goal);
    let _nonce = nonce;
    rsx! {
        div { class: "stage",
            div { class: "subhead",
                button {
                    class: "back",
                    onclick: move |_| screen.set(Screen::Goals { subject, class_no }),
                    Icon { name: "arrow-left" } "वापस"
                }
                h1 { Icon { name: "clipboard-check" } "परीक्षा · {title}" }
            }
            match step() {
                Step::Write { fade, blank } => {
                    let show = !blank || reveal();
                    let label = opening.clone();
                    rsx! {
                        if show_story { Frames { class_no } }
                        if show {
                            p { class: "listen-line", "{label}" }
                        } else {
                            div { class: "actions",
                                button {
                                    class: "quiet",
                                    onclick: move |_| reveal.set(true),
                                    Icon { name: "eye" } "दिखाओ"
                                }
                            }
                        }
                        WritingBoard {
                            dots: if blank { Vec::new() } else { dots() },
                            fade: fade,
                            rules: true,
                            pattern: false,
                            traveler: false,
                            ink: true,
                            finger_ok: finger_ok,
                            round: if blank { 3 } else { fade },
                            on_next: move |_| {
                                if blank {
                                    step.set(Step::Ask { index: 0, correct: 0 });
                                } else if fade < 2 {
                                    step.set(Step::Write { fade: fade + 1, blank: false });
                                } else {
                                    reveal.set(false);
                                    step.set(Step::Write { fade: 0, blank: true });
                                }
                            },
                        }
                    }
                }
                Step::Ask { index, correct } => {
                    let question = questions.get(index).cloned();
                    rsx! {
                        if show_story { Frames { class_no } }
                        if show_story {
                            div { class: "listen",
                                for (line_i, line) in lines.iter().enumerate() {
                                    p { key: "line-{line_i}", class: "listen-line", "{line}" }
                                }
                            }
                        }
                        if let Some(question) = question {
                            p { class: "hint", "{index + 1} / {questions.len()}" }
                            Quiz {
                                prompt: question.prompt.clone(),
                                choices: question.choices.clone(),
                                on_pick: move |pick| {
                                    let correct = correct + u8::from(pick == question.answer);
                                    if index + 1 >= questions.len() {
                                        finish(correct);
                                    } else {
                                        step.set(Step::Ask { index: index + 1, correct });
                                    }
                                },
                            }
                        }
                    }
                }
                Step::Fail => rsx! {
                    div { class: "award",
                        h1 { "फिर एक बार" }
                        div { class: "actions",
                            button {
                                onclick: move |_| step.set(Step::Ask { index: 0, correct: 0 }),
                                Icon { name: "rotate-ccw" } "प्रश्न फिर"
                            }
                            button {
                                class: "quiet",
                                onclick: move |_| screen.set(Screen::Home),
                                Icon { name: "home" } "घर"
                            }
                        }
                    }
                },
                Step::Sticker { options, gained } => rsx! {
                    div {
                        p { class: "plus", "+{gained}" }
                        StickerChoice {
                            options: options.clone(),
                            on_pick: move |picked: String| {
                                store_sig.write().keep_sticker(&picked);
                                store_sig.write().save();
                                step.set(Step::Done { gained });
                            },
                        }
                    }
                },
                Step::Done { gained } => rsx! {
                    div { class: "award",
                        h1 { "हो गया" }
                        if gained > 0 {
                            p { class: "plus", "+{gained}" }
                            p { "कुल ★ {store_sig.read().points}" }
                        }
                        div { class: "actions",
                            button {
                                onclick: move |_| {
                                    screen.set(Screen::Exam { subject, class_no, goal, nonce: nonce + 1 });
                                },
                                Icon { name: "rotate-ccw" } "फिर से"
                            }
                            button {
                                class: "quiet",
                                onclick: move |_| screen.set(Screen::Goals { subject, class_no }),
                                Icon { name: "arrow-left" } "वापस"
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn Frames(class_no: u8) -> Element {
    rsx! {
        div { class: "seq",
            if class_no != 1 {
                div { class: "frame",
                    svg { view_box: "0 0 120 90",
                        rect { x: "0", y: "58", width: "120", height: "32", fill: "#d7c4a3" }
                        ellipse { cx: "60", cy: "58", rx: "10", ry: "14", fill: "#8a6a45" }
                    }
                    p { "बीज" }
                }
                div { class: "frame",
                    svg { view_box: "0 0 120 90",
                        rect { x: "0", y: "64", width: "120", height: "26", fill: "#d7c4a3" }
                        path { d: "M60 66 C58 40 70 36 66 22", fill: "none", stroke: "#2f6f4e", stroke_width: "4", stroke_linecap: "round" }
                        ellipse { cx: "74", cy: "28", rx: "10", ry: "6", fill: "#7ba3c9" }
                        ellipse { cx: "52", cy: "34", rx: "10", ry: "6", fill: "#2f6f4e" }
                    }
                    p { "पौधा" }
                }
            }
            div { class: "frame",
                svg { view_box: "0 0 120 90",
                    rect { x: "0", y: "70", width: "120", height: "20", fill: "#d7c4a3" }
                    path { d: "M78 72 C76 40 88 30 82 16", fill: "none", stroke: "#2f6f4e", stroke_width: "4", stroke_linecap: "round" }
                    ellipse { cx: "96", cy: "22", rx: "12", ry: "7", fill: "#7ba3c9" }
                    circle { cx: "36", cy: "28", r: "10", fill: "#f0d78c" }
                    path { d: "M36 38 L36 62", stroke: "#1f2933", stroke_width: "3", stroke_linecap: "round" }
                    path { d: "M36 46 L58 40", stroke: "#1f2933", stroke_width: "3", stroke_linecap: "round" }
                    circle { cx: "66", cy: "34", r: "3", fill: "#7ba3c9" }
                    circle { cx: "74", cy: "42", r: "3", fill: "#7ba3c9" }
                }
                p { "पानी" }
            }
        }
    }
}
