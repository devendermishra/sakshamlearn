use crate::score;
use dioxus::prelude::*;
use super::icon::Icon;
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn WritingBoard(
    dots: Vec<(f32, f32)>,
    fade: u8,
    rules: bool,
    pattern: bool,
    traveler: bool,
    ink: bool,
    finger_ok: bool,
    round: u8,
    on_next: EventHandler<bool>,
) -> Element {
    let mut strokes = use_signal(Vec::<Vec<(f32, f32)>>::new);
    let mut missed = use_signal(Vec::<usize>::new);
    let mut drawing = use_signal(|| false);
    let mut active_pointer = use_signal(|| None::<i32>);
    let mut size = use_signal(|| (1.0_f64, 1.0_f64));
    let mut tick = use_signal(|| 0usize);
    let mut traveler_gen = use_signal(|| 0u32);

    use_effect(move || {
        let _ = round;
        strokes.set(Vec::new());
        missed.set(Vec::new());
        drawing.set(false);
    });

    use_effect(move || {
        spawn(async move {
            TimeoutFuture::new(60).await;
            if let Some(next) = slate_size().await {
                size.set(next);
            }
        });
    });

    let dots_for_draw = dots.clone();
    let dots_for_up = dots.clone();
    let dots_for_leave = dots.clone();
    let dots_for_next = dots.clone();
    let travel_dots = dots.clone();
    use_effect(move || {
        if !traveler || travel_dots.is_empty() {
            return;
        }
        traveler_gen += 1;
        let generation = traveler_gen();
        let total = travel_dots.len();
        tick.set(0);
        spawn(async move {
            for step in 0..total {
                TimeoutFuture::new(70).await;
                if traveler_gen() != generation {
                    break;
                }
                tick.set(step);
            }
        });
    });

    let shown: Vec<(usize, (f32, f32))> = dots_for_draw
        .iter()
        .copied()
        .enumerate()
        .filter(|(index, _)| fade < 2 || index % 2 == 0)
        .collect();
    let missed_now = missed();
    let stroke_now = strokes();
    let marks = dots.clone();
    let travel_at = tick().min(marks.len().saturating_sub(1));

    let allow = move |kind: String| -> bool {
        kind.is_empty() || kind == "pen" || kind == "mouse" || (finger_ok && kind == "touch")
    };

    rsx! {
        div { class: "slate-wrap",
            svg {
                id: "slate",
                class: if pattern { "slate pattern" } else if rules { "slate rules" } else { "slate" },
                view_box: "0 0 100 100",
                preserve_aspect_ratio: "none",
                onpointerdown: move |event| {
                    if !ink || !allow(event.pointer_type()) {
                        return;
                    }
                    let point = event.element_coordinates();
                    let (x, y) = map_point(point.x, point.y, size());
                    active_pointer.set(Some(event.pointer_id()));
                    drawing.set(true);
                    strokes.write().push(vec![(x, y)]);
                },
                onpointermove: move |event| {
                    if !drawing() || active_pointer() != Some(event.pointer_id()) {
                        return;
                    }
                    let point = event.element_coordinates();
                    let next = map_point(point.x, point.y, size());
                    let mut all = strokes.write();
                    if let Some(stroke) = all.last_mut() {
                        if stroke.last().map(|prev| score_dist(*prev, next) > 0.6).unwrap_or(true) {
                            stroke.push(next);
                        }
                    }
                },
                onpointerup: move |event| {
                    if active_pointer() != Some(event.pointer_id()) {
                        return;
                    }
                    drawing.set(false);
                    active_pointer.set(None);
                    let result = score::coverage(&dots_for_up, &strokes());
                    missed.set(result.missed);
                },
                onpointerleave: move |_| {
                    if drawing() {
                        drawing.set(false);
                        let result = score::coverage(&dots_for_leave, &strokes());
                        missed.set(result.missed);
                    }
                },
                rect { x: "0", y: "0", width: "100", height: "100", fill: "transparent" }
                for (index, (x, y)) in shown {
                    circle {
                        key: "d{index}",
                        class: if missed_now.contains(&index) { "dot miss" } else if fade == 0 { "dot" } else if fade == 1 { "dot fade1" } else { "dot fade2" },
                        cx: "{x}",
                        cy: "{y}",
                        r: "1.5",
                        pointer_events: "none",
                    }
                }
                if traveler && !marks.is_empty() {
                    circle {
                        class: "traveler",
                        cx: "{marks[travel_at].0}",
                        cy: "{marks[travel_at].1}",
                        r: "2.2",
                        pointer_events: "none",
                    }
                }
                for (index, stroke) in stroke_now.iter().enumerate() {
                    polyline {
                        key: "s{index}",
                        class: "ink",
                        points: stroke_points(stroke),
                        pointer_events: "none",
                    }
                }
            }
            div { class: "actions",
                button {
                    class: "quiet",
                    onclick: move |_| {
                        strokes.set(Vec::new());
                        missed.set(Vec::new());
                    },
                    Icon { name: "eraser" } "मिटाओ"
                }
                button {
                    onclick: move |_| {
                        let result = score::coverage(&dots_for_next, &strokes());
                        missed.set(result.missed.clone());
                        on_next.call(result.pass);
                    },
                    Icon { name: "arrow-right" } "आगे"
                }
            }
        }
    }
}

fn map_point(x: f64, y: f64, size: (f64, f64)) -> (f32, f32) {
    let (w, h) = size;
    let nx = if w > 1.0 { (x / w * 100.0).clamp(0.0, 100.0) } else { x.clamp(0.0, 100.0) };
    let ny = if h > 1.0 { (y / h * 100.0).clamp(0.0, 100.0) } else { y.clamp(0.0, 100.0) };
    (nx as f32, ny as f32)
}

fn score_dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

fn stroke_points(stroke: &[(f32, f32)]) -> String {
    stroke
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ")
}

async fn slate_size() -> Option<(f64, f64)> {
    let value = document::eval(
        r#"const el = document.getElementById("slate");
if (!el) return [1, 1];
const r = el.getBoundingClientRect();
return [r.width, r.height];"#,
    )
    .await
    .ok()?;
    let pair: Vec<f64> = serde_json::from_value(value).ok()?;
    Some((*pair.first()?, *pair.get(1)?))
}
