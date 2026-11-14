use crate::content::{self, Kind, Lang, Mode};
use crate::points;
use crate::speech::speak_as;
use crate::state::{use_app, Screen};
use crate::ui::icon::Icon;
use crate::ui::quiz::Quiz;
use crate::ui::rewards::StickerChoice;
use crate::ui::trace::sample_glyph;
use crate::ui::writing_board::WritingBoard;
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
pub fn SessionScreen(id: String, mode: Mode, nonce: u32) -> Element {
    let app = use_app();
    let math = id.starts_with("math:");
    let lang = if math {
        Lang::from_store(&app.store.read().math_lang)
    } else {
        Lang::Hi
    };
    let voice = lang.code();
    let practice_seed = use_signal(|| points::now_ms() ^ ((nonce as u64 + 1) << 17));
    let seed = if math && mode == Mode::Practice {
        practice_seed()
    } else {
        0
    };
    let found = content::find_for(&id, lang, seed);
    let initial = found
        .as_ref()
        .map(|item| start_phase(mode, &item.kind))
        .unwrap_or(Phase::Done {
            gained: 0,
            passed: false,
        });
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
    let band = if math { 11 } else { store_sig.read().band_mm };

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
        let voice = voice;
        use_effect(move || {
            if !intro.is_empty() {
                speak_as(&intro, intro_sound, voice);
            }
        });
    }

    {
        let spoken = questions.clone();
        let phase_voice = phase;
        let quiz_sound = store_sig.read().sound_on;
        let voice = voice;
        use_effect(move || {
            if let Phase::Quiz { index, .. } = phase_voice() {
                if let Some(question) = spoken.get(index as usize) {
                    speak_as(&question.prompt, quiz_sound, voice);
                }
            }
        });
    }

    let Some(item) = found else {
        return rsx! {
            div { class: "stage",
                button {
                    onclick: move |_| screen.set(Screen::Home),
                    Icon { name: "home" } "{content::phrase(\"home\", lang)}"
                }
            }
        };
    };

    let pattern_dots = match &item.kind {
        Kind::Pattern { dots } => dots.clone(),
        _ => Vec::new(),
    };
    let active_dots = if pattern_dots.is_empty() {
        dots()
    } else {
        pattern_dots.clone()
    };
    let (step, total) = step_count(mode, &item.kind, phase());
    let title = item.title.clone();
    let voice_line = item.speak.clone();
    let short_trace = matches!(
        item.kind,
        Kind::Number {
            handwriting: false,
            trace: Some(_),
            ..
        }
    );
    let session_id = id.clone();
    let following = content::next_id(&id);

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
                    phase.set(Phase::Done {
                        gained: outcome.gained,
                        passed: true,
                    });
                } else {
                    phase.set(Phase::Sticker {
                        options: outcome.offer,
                        gained: outcome.gained,
                    });
                }
                speak_as(content::phrase("done", lang), sound_on, voice);
            } else {
                store_sig.write().mark_retry(&session_id, now);
                store_sig.write().save();
                phase.set(Phase::Fail);
            }
        }
    };

    let mode_icon = if mode == Mode::Learn {
        "sprout"
    } else {
        "list-checks"
    };
    let mode_label = content::phrase(
        if mode == Mode::Learn {
            "learn"
        } else {
            "practice"
        },
        lang,
    );
    rsx! {
        div {
            class: "stage",
            style: "--band: {band}mm",
            div { class: "session-head",
                h1 { Icon { name: mode_icon } "{mode_label} · {title}" }
                button {
                    class: "ghost",
                    onclick: move |_| paused.set(true),
                    Icon { name: "pause" } "{content::phrase(\"pause\", lang)}"
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
                        Model { text: title.clone(), voice: voice_line.clone(), formation: item.formation, sound: sound, lang: lang }
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
                                lang: lang,
                                on_next: move |_| phase.set(Phase::Trace { fade: 0, blank: false }),
                            }
                        } else {
                            div {
                                p { class: "hint", "{content::phrase(\"dots_next\", lang)}" }
                                div { class: "actions",
                                    button {
                                        onclick: move |_| phase.set(Phase::Trace { fade: 0, blank: false }),
                                        Icon { name: "arrow-right" } "{content::phrase(\"next\", lang)}"
                                    }
                                }
                            }
                        }
                    }
                },
                Phase::Listen => {
                    let (lines, picture) = match &item.kind {
                        Kind::Reading { lines, .. } => (lines.clone(), false),
                        Kind::Number { picture, .. } => (picture.split('\n').map(|line| line.to_string()).collect(), true),
                        _ => (vec![voice_line.clone()], false),
                    };
                    let spoken = if picture { voice_line.clone() } else { lines.join(" ") };
                    let line_class = if picture { "picture" } else { "listen-line" };
                    rsx! {
                        div { class: "listen",
                            for (index, line) in lines.into_iter().enumerate() {
                                p { class: line_class, key: "{index}", "{line}" }
                            }
                            div { class: "actions",
                                button {
                                    class: "quiet",
                                    onclick: move |_| speak_as(&spoken, sound, voice),
                                    Icon { name: "volume-2" } "{content::phrase(\"listen\", lang)}"
                                }
                                button {
                                    onclick: move |_| {
                                        if mode == Mode::Learn {
                                            if short_trace {
                                                phase.set(Phase::Trace { fade: 0, blank: false });
                                            } else {
                                                phase.set(Phase::Done { gained: 0, passed: false });
                                            }
                                        } else if short_trace {
                                            phase.set(Phase::Trace { fade: 1, blank: false });
                                        } else {
                                            phase.set(Phase::Quiz { index: 0, correct: 0 });
                                        }
                                    },
                                    Icon { name: "arrow-right" } "{content::phrase(\"next\", lang)}"
                                }
                            }
                        }
                    }
                },
                Phase::Trace { fade, blank } => {
                    let writing = match &item.kind {
                        Kind::Number { trace: Some(text), .. } => text.clone(),
                        _ => title.clone(),
                    };
                    let fact_picture = match &item.kind {
                        Kind::Number { picture, handwriting: false, .. } => Some(picture.clone()),
                        _ => None,
                    };
                    let model = if blank && !reveal() { None } else { Some(writing) };
                    let trace_key = if blank { "blank".to_string() } else { fade.to_string() };
                    rsx! {
                        if let Some(picture) = fact_picture {
                            div { class: "listen",
                                for (index, line) in picture.split('\n').enumerate() {
                                    p { class: "picture", key: "pic-{index}", "{line}" }
                                }
                            }
                        }
                        div { class: "work",
                            if let Some(text) = model {
                                Model { text: text, voice: voice_line.clone(), formation: item.formation, sound: sound, lang: lang }
                            } else {
                                div { class: "model",
                                    button {
                                        class: "quiet",
                                        onclick: move |_| reveal.set(true),
                                        Icon { name: "eye" } "{content::phrase(\"show\", lang)}"
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
                                lang: lang,
                                on_next: move |_| {
                                    if mode == Mode::Learn {
                                        phase.set(Phase::Done { gained: 0, passed: false });
                                    } else if short_trace || blank {
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
                        Model { text: title.clone(), voice: voice_line.clone(), formation: false, sound: sound, lang: lang }
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
                            lang: lang,
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
                        h1 { "{content::phrase(\"once\", lang)}" }
                        div { class: "actions",
                            button {
                                onclick: move |_| {
                                    phase.set(match &item.kind {
                                        Kind::Pattern { .. } => Phase::PatternTry { index: 0, passed: 0 },
                                        _ => Phase::Quiz { index: 0, correct: 0 },
                                    });
                                },
                                Icon { name: "rotate-ccw" } "{content::phrase(\"retry\", lang)}"
                            }
                            button {
                                class: "quiet",
                                onclick: move |_| screen.set(Screen::Home),
                                Icon { name: "home" } "{content::phrase(\"home\", lang)}"
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
                        h1 { "{content::phrase(\"done\", lang)}" }
                        if passed && gained > 0 {
                            p { class: "plus", "+{gained}" }
                            p { "{content::phrase(\"total\", lang)} ★ {store_sig.read().points}" }
                        }
                        div { class: "actions",
                            if let Some(following_id) = following.clone() {
                                button {
                                    onclick: move |_| {
                                        screen.set(Screen::Session {
                                            id: following_id.clone(),
                                            mode,
                                            nonce: 0,
                                        });
                                    },
                                    Icon { name: "arrow-right" } "{content::phrase(\"next\", lang)}"
                                }
                            }
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
                                Icon { name: "rotate-ccw" } "{content::phrase(\"again\", lang)}"
                            }
                            button {
                                class: "quiet",
                                onclick: move |_| screen.set(Screen::Home),
                                Icon { name: "home" } "{content::phrase(\"home\", lang)}"
                            }
                        }
                    }
                },
            }
            if paused() {
                div { class: "pause",
                    p { "{content::phrase(\"paused\", lang)}" }
                    button {
                        onclick: move |_| paused.set(false),
                        Icon { name: "play" } "{content::phrase(\"go\", lang)}"
                    }
                }
            }
        }
    }
}

#[component]
fn Model(text: String, voice: String, formation: bool, sound: bool, lang: Lang) -> Element {
    let spoken = voice.clone();
    let code = lang.code();
    rsx! {
        div { class: "model",
            p { class: "glyph", "{text}" }
            if formation {
                p { class: "hint", "{content::phrase(\"form\", lang)}" }
            }
            button {
                class: "quiet",
                onclick: move |_| speak_as(&spoken, sound, code),
                Icon { name: "volume-2" } "{content::phrase(\"hear\", lang)}"
            }
        }
    }
}

fn start_phase(mode: Mode, kind: &Kind) -> Phase {
    match (mode, kind) {
        (Mode::Learn, Kind::Reading { .. }) => Phase::Listen,
        (
            Mode::Learn,
            Kind::Number {
                handwriting: true, ..
            },
        ) => Phase::Demo,
        (Mode::Learn, Kind::Number { .. }) => Phase::Listen,
        (Mode::Learn, _) => Phase::Demo,
        (Mode::Practice, Kind::Pattern { .. }) => Phase::PatternTry {
            index: 0,
            passed: 0,
        },
        (
            Mode::Practice,
            Kind::Number {
                handwriting: true, ..
            },
        ) => Phase::Trace {
            fade: 0,
            blank: false,
        },
        (Mode::Practice, Kind::Reading { .. } | Kind::Number { .. }) => Phase::Listen,
        (Mode::Practice, _) => Phase::Trace {
            fade: 0,
            blank: false,
        },
    }
}

fn glyph_text(kind: &Kind) -> String {
    match kind {
        Kind::Glyph { text } => text.clone(),
        Kind::Number { trace, .. } => trace.clone().unwrap_or_default(),
        _ => String::new(),
    }
}

fn step_count(mode: Mode, kind: &Kind, phase: Phase) -> (usize, usize) {
    let fact = matches!(
        kind,
        Kind::Number {
            handwriting: false,
            trace: Some(_),
            ..
        }
    );
    let total = match (mode, kind) {
        (Mode::Learn, Kind::Reading { .. }) => 1,
        (
            Mode::Learn,
            Kind::Number {
                handwriting: false,
                trace: None,
                ..
            },
        ) => 1,
        (Mode::Learn, _) => 2,
        (Mode::Practice, Kind::Pattern { .. }) => 3,
        (
            Mode::Practice,
            Kind::Number {
                handwriting: true, ..
            },
        ) => 7,
        (Mode::Practice, Kind::Number { trace: Some(_), .. }) => 5,
        (Mode::Practice, Kind::Number { .. } | Kind::Reading { .. }) => 4,
        (Mode::Practice, _) => 7,
    };
    let step = match phase {
        Phase::Demo | Phase::Listen => 1,
        Phase::Trace { fade, blank } => {
            if mode == Mode::Learn || fact {
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
                (
                    Mode::Practice,
                    Kind::Number {
                        handwriting: true, ..
                    }
                    | Kind::Glyph { .. },
                ) => 4,
                (Mode::Practice, Kind::Number { trace: Some(_), .. }) => 2,
                (Mode::Practice, Kind::Number { .. } | Kind::Reading { .. }) => 1,
                (Mode::Practice, _) => 2,
                _ => 1,
            };
            base + index as usize + 1
        }
        Phase::Fail | Phase::Sticker { .. } | Phase::Done { .. } => total,
    };
    (step.min(total), total)
}
