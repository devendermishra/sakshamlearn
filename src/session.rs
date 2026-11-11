use crate::content::{self, Kind, Mode};
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
enum Phase {
    Demo,
    Listen,
    Trace { fade: u8, blank: bool },
    PatternTry { index: u8, passed: u8 },
    Quiz { index: u8, correct: u8 },
    Fail,
    Sticker { options: Vec<String>, gained: u32 },
    Done { gained: u32, passed: bool },
}

#[component]
pub fn SessionScreen(id: String, mode: Mode) -> Element {
    let app = use_app();
    let found = content::find(&id);
    let initial = found
        .as_ref()
        .map(|item| start_phase(mode, &item.kind))
        .unwrap_or(Phase::Done { gained: 0, passed: false });
    let mut phase = use_signal(|| initial);
    let mut paused = use_signal(|| false);
    let mut reveal = use_signal(|| false);
    let mut dots = use_signal(Vec::<(f32, f32)>::new);
    let glyph = found
        .as_ref()
        .map(|item| glyph_text(&item.kind))
        .unwrap_or_default();
    let questions = found
        .as_ref()
        .map(content::questions_for)
        .unwrap_or_default();
    let mut store_sig = app.store;
    let mut screen = app.screen;
    let sound = store_sig.read().sound_on;
    let finger_ok = store_sig.read().finger_ok;
    let band = store_sig.read().band_mm;

    use_effect({
        let glyph = glyph.clone();
        move || {
            let glyph = glyph.clone();
            if glyph.is_empty() {
                return;
            }
            spawn(async move {
                dots.set(sample_glyph(&glyph).await);
            });
        }
    });

    {
        let intro = found
            .as_ref()
            .map(|item| item.speak.clone())
            .unwrap_or_default();
        let intro_sound = store_sig.read().sound_on;
        use_effect(move || {
            if !intro.is_empty() {
                speak(&intro, intro_sound);
            }
        });
    }

    {
        let spoken = questions.clone();
        let phase_voice = phase;
        let quiz_sound = store_sig.read().sound_on;
        use_effect(move || {
            if let Phase::Quiz { index, .. } = phase_voice() {
                if let Some(question) = spoken.get(index as usize) {
                    speak(&question.prompt, quiz_sound);
                }
            }
        });
    }

    let Some(item) = found else {
        return rsx! {
            div { class: "stage",
                button {
                    onclick: move |_| screen.set(Screen::Home),
                    Icon { name: "home" } "घर"
                }
            }
        };
    };

    let pattern_dots = match &item.kind {
        Kind::Pattern { dots } => dots.clone(),
        _ => Vec::new(),
    };
    let active_dots = if pattern_dots.is_empty() { dots() } else { pattern_dots.clone() };
    let (step, total) = step_count(mode, &item.kind, phase());
    let title = item.title.clone();
    let session_id = id.clone();

    let mut finish = {
        let session_id = session_id.clone();
        let mut store_sig = store_sig;
        let mut phase = phase;
        move |correct: u8| {
            let now = points::now_ms();
            let sound_on = store_sig.read().sound_on;
            if correct >= 2 {
                let outcome = store_sig.write().award(&session_id, correct, now);
                store_sig.write().save();
                if outcome.offer.is_empty() {
                    phase.set(Phase::Done { gained: outcome.gained, passed: true });
                } else {
                    phase.set(Phase::Sticker { options: outcome.offer, gained: outcome.gained });
                }
                speak("हो गया", sound_on);
            } else {
                store_sig.write().mark_retry(&session_id, now);
                store_sig.write().save();
                phase.set(Phase::Fail);
            }
        }
    };

    let mode_icon = if mode == Mode::Learn { "sprout" } else { "list-checks" };
    rsx! {
        div {
            class: "stage",
            style: "--band: {band}mm",
            div { class: "session-head",
                h1 { Icon { name: mode_icon } "{head_title(mode)} · {title}" }
                button {
                    class: "ghost",
                    onclick: move |_| paused.set(true),
                    Icon { name: "pause" } "रोकें"
                }
            }
            div { class: "dots-row",
                for index in 0..total {
                    span { class: if index < step { "pip on" } else { "pip" }, key: "{index}" }
                }
            }
            match phase() {
                Phase::Demo => rsx! {
                    div { class: "work",
                        Model { text: title.clone(), formation: item.formation, sound: sound }
                        if !pattern_dots.is_empty() {
                            WritingBoard {
                                key: "{band}",
                                dots: active_dots.clone(),
                                fade: 0,
                                rules: false,
                                pattern: true,
                                traveler: true,
                                ink: false,
                                finger_ok: finger_ok,
                                round: 0,
                                on_next: move |_| phase.set(Phase::Trace { fade: 0, blank: false }),
                            }
                        } else {
                            div {
                                p { class: "hint", "आगे बिंदुओं पर लिखेंगे" }
                                div { class: "actions",
                                    button {
                                        onclick: move |_| phase.set(Phase::Trace { fade: 0, blank: false }),
                                        Icon { name: "arrow-right" } "आगे"
                                    }
                                }
                            }
                        }
                    }
                },
                Phase::Listen => {
                    let lines = match &item.kind {
                        Kind::Reading { lines, .. } => lines.clone(),
                        _ => vec![item.speak.clone()],
                    };
                    let spoken = lines.join(" ");
                    rsx! {
                        div { class: "listen",
                            for (index, line) in lines.into_iter().enumerate() {
                                p { class: "listen-line", key: "{index}", "{line}" }
                            }
                            div { class: "actions",
                                button {
                                    class: "quiet",
                                    onclick: move |_| speak(&spoken, sound),
                                    Icon { name: "volume-2" } "फिर सुनो"
                                }
                                button {
                                    onclick: move |_| {
                                        if mode == Mode::Learn {
                                            phase.set(Phase::Done { gained: 0, passed: false });
                                        } else {
                                            phase.set(Phase::Quiz { index: 0, correct: 0 });
                                        }
                                    },
                                    Icon { name: "arrow-right" } "आगे"
                                }
                            }
                        }
                    }
                },
                Phase::Trace { fade, blank } => {
                    let model = if blank && !reveal() { None } else { Some(title.clone()) };
                    let trace_key = if blank { "blank".to_string() } else { fade.to_string() };
                    rsx! {
                        div { class: "work",
                            if let Some(text) = model {
                                Model { text: text, formation: item.formation, sound: sound }
                            } else {
                                div { class: "model",
                                    button {
                                        class: "quiet",
                                        onclick: move |_| reveal.set(true),
                                        Icon { name: "eye" } "दिखाओ"
                                    }
                                }
                            }
                            WritingBoard {
                                key: "{trace_key}",
                                dots: if blank { Vec::new() } else { active_dots.clone() },
                                fade: fade,
                                rules: pattern_dots.is_empty(),
                                pattern: !pattern_dots.is_empty(),
                                traveler: false,
                                ink: true,
                                finger_ok: finger_ok,
                                round: if blank { 3 } else { fade },
                                on_next: move |_| {
                                    if mode == Mode::Learn {
                                        phase.set(Phase::Done { gained: 0, passed: false });
                                    } else if blank {
                                        phase.set(Phase::Quiz { index: 0, correct: 0 });
                                    } else if fade < 2 {
                                        phase.set(Phase::Trace { fade: fade + 1, blank: false });
                                    } else {
                                        reveal.set(false);
                                        phase.set(Phase::Trace { fade: 0, blank: true });
                                    }
                                },
                            }
                        }
                    }
                },
                Phase::PatternTry { index, passed } => rsx! {
                    div { class: "work",
                        Model { text: title.clone(), formation: false, sound: sound }
                        WritingBoard {
                            key: "try-{index}",
                            dots: active_dots.clone(),
                            fade: index,
                            rules: false,
                            pattern: true,
                            traveler: false,
                            ink: true,
                            finger_ok: finger_ok,
                            round: index,
                            on_next: move |ok| {
                                let passed = passed + u8::from(ok);
                                if index >= 2 {
                                    finish(passed);
                                } else {
                                    phase.set(Phase::PatternTry { index: index + 1, passed });
                                }
                            },
                        }
                    }
                },
                Phase::Quiz { index, correct } => {
                    let question = questions.get(index as usize).cloned();
                    rsx! {
                        if let Some(question) = question {
                            Quiz {
                                prompt: question.prompt.clone(),
                                choices: question.choices.clone(),
                                on_pick: move |pick| {
                                    let correct = correct + u8::from(pick == question.answer);
                                    let next = index + 1;
                                    if next >= 3 || next as usize >= questions.len() {
                                        finish(correct);
                                    } else {
                                        phase.set(Phase::Quiz { index: next, correct });
                                    }
                                },
                            }
                        }
                    }
                },
                Phase::Fail => rsx! {
                    div { class: "award",
                        h1 { "फिर एक बार" }
                        div { class: "actions",
                            button {
                                onclick: move |_| {
                                    phase.set(match &item.kind {
                                        Kind::Pattern { .. } => Phase::PatternTry { index: 0, passed: 0 },
                                        _ => Phase::Quiz { index: 0, correct: 0 },
                                    });
                                },
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
                Phase::Sticker { options, gained } => rsx! {
                    div {
                        p { class: "plus", "+{gained}" }
                        StickerChoice {
                            options: options.clone(),
                            on_pick: move |picked: String| {
                                store_sig.write().keep_sticker(&picked);
                                store_sig.write().save();
                                phase.set(Phase::Done { gained, passed: true });
                            },
                        }
                    }
                },
                Phase::Done { gained, passed } => rsx! {
                    div { class: "award",
                        h1 { "हो गया" }
                        if passed && gained > 0 {
                            p { class: "plus", "+{gained}" }
                            p { "कुल ★ {store_sig.read().points}" }
                        }
                        div { class: "actions",
                            button {
                                onclick: move |_| {
                                    let current = screen.read().clone();
                                    if let Screen::Session { nonce, .. } = current {
                                        screen.set(Screen::Session {
                                            id: id.clone(),
                                            mode,
                                            nonce: nonce + 1,
                                        });
                                    }
                                },
                                Icon { name: "rotate-ccw" } "फिर से"
                            }
                            button {
                                class: "quiet",
                                onclick: move |_| screen.set(Screen::Home),
                                Icon { name: "home" } "घर"
                            }
                        }
                    }
                },
            }
            if paused() {
                div { class: "pause",
                    p { "रुका हुआ" }
                    button {
                        onclick: move |_| paused.set(false),
                        Icon { name: "play" } "चलो"
                    }
                }
            }
        }
    }
}

#[component]
fn Model(text: String, formation: bool, sound: bool) -> Element {
    let spoken = text.clone();
    rsx! {
        div { class: "model",
            p { class: "glyph", "{text}" }
            if formation {
                p { class: "hint", "बनावट का अभ्यास" }
            }
            button {
                class: "quiet",
                onclick: move |_| speak(&spoken, sound),
                Icon { name: "volume-2" } "सुनो"
            }
        }
    }
}

fn start_phase(mode: Mode, kind: &Kind) -> Phase {
    match (mode, kind) {
        (Mode::Learn, Kind::Reading { .. }) => Phase::Listen,
        (Mode::Learn, _) => Phase::Demo,
        (Mode::Practice, Kind::Pattern { .. }) => Phase::PatternTry { index: 0, passed: 0 },
        (Mode::Practice, Kind::Reading { .. }) => Phase::Listen,
        (Mode::Practice, _) => Phase::Trace { fade: 0, blank: false },
    }
}

fn glyph_text(kind: &Kind) -> String {
    match kind {
        Kind::Glyph { text } => text.clone(),
        _ => String::new(),
    }
}

fn head_title(mode: Mode) -> &'static str {
    match mode {
        Mode::Learn => "सीखो",
        Mode::Practice => "अभ्यास",
    }
}

fn step_count(mode: Mode, kind: &Kind, phase: Phase) -> (usize, usize) {
    let total = match (mode, kind) {
        (Mode::Learn, Kind::Reading { .. }) => 1,
        (Mode::Learn, _) => 2,
        (Mode::Practice, Kind::Pattern { .. }) => 3,
        (Mode::Practice, Kind::Reading { .. }) => 4,
        (Mode::Practice, _) => 7,
    };
    let step = match phase {
        Phase::Demo | Phase::Listen => 1,
        Phase::Trace { fade, blank } => {
            if mode == Mode::Learn {
                2
            } else if blank {
                4
            } else {
                fade as usize + 1
            }
        }
        Phase::PatternTry { index, .. } => index as usize + 1,
        Phase::Quiz { index, .. } => {
            let base = match (mode, kind) {
                (Mode::Practice, Kind::Reading { .. }) => 1,
                (Mode::Practice, Kind::Glyph { .. }) => 4,
                (Mode::Practice, _) => 2,
                _ => 1,
            };
            base + index as usize + 1
        }
        Phase::Fail | Phase::Sticker { .. } | Phase::Done { .. } => total,
    };
    (step.min(total), total)
}
