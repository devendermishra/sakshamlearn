#[derive(Clone, Debug)]
pub struct Coverage {
    pub pass: bool,
    pub missed: Vec<usize>,
}

pub fn coverage(dots: &[(f32, f32)], strokes: &[Vec<(f32, f32)>]) -> Coverage {
    if dots.is_empty() {
        return Coverage {
            pass: false,
            missed: Vec::new(),
        };
    }
    let hit_r = 7.0;
    let stray_r = 14.0;
    let mut missed = Vec::new();
    let mut hits = 0usize;
    for (index, dot) in dots.iter().enumerate() {
        let near = strokes
            .iter()
            .flatten()
            .any(|point| dist(*point, *dot) <= hit_r);
        if near {
            hits += 1;
        } else {
            missed.push(index);
        }
    }
    let mut total = 0usize;
    let mut stray = 0usize;
    for point in strokes.iter().flatten() {
        total += 1;
        let near = dots.iter().any(|dot| dist(*point, *dot) <= stray_r);
        if !near {
            stray += 1;
        }
    }
    let covered = hits as f32 / dots.len() as f32;
    let stray_ratio = if total == 0 {
        1.0
    } else {
        stray as f32 / total as f32
    };
    let pass = total >= 6 && covered >= 0.6 && stray_ratio <= 0.5;
    Coverage { pass, missed }
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}
