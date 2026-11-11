mod catalog;
mod hindi;
mod patterns;

pub use catalog::{
    bank, extra_ids, goal_aim, goal_key, goal_title, lookup, paper, story, subjects, Goal, GOALS,
};

use hindi::{FORMS, FORMATION, READINGS, SWAR, VYANJAN, WORDS, WORD_GROUPS};
use patterns::PATTERNS;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Learn,
    Practice,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Group {
    Pattern,
    Akshar,
    Matra,
    Word,
    Reading,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuizQ {
    pub prompt: String,
    pub choices: Vec<String>,
    pub answer: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Pattern { dots: Vec<(f32, f32)> },
    Glyph { text: String },
    Reading { lines: Vec<String>, questions: Vec<QuizQ> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub speak: String,
    pub formation: bool,
    pub kind: Kind,
}

pub fn find(id: &str) -> Option<Item> {
    let mut parts = id.split(':');
    let kind = parts.next()?;
    match kind {
        "pat" => {
            let index: usize = parts.next()?.parse().ok()?;
            pattern_item(index)
        }
        "swar" => {
            let index: usize = parts.next()?.parse().ok()?;
            SWAR.get(index).map(|text| glyph_item(format!("swar:{index}"), text, false))
        }
        "vyn" => {
            let index: usize = parts.next()?.parse().ok()?;
            VYANJAN.get(index).map(|text| {
                glyph_item(format!("vyn:{index}"), text, FORMATION.contains(text))
            })
        }
        "bar" => {
            let consonant: usize = parts.next()?.parse().ok()?;
            let form: usize = parts.next()?.parse().ok()?;
            barakhadi_item(consonant, form)
        }
        "word" => {
            let index: usize = parts.next()?.parse().ok()?;
            WORDS.get(index).map(|(text, _)| glyph_item(format!("word:{index}"), text, false))
        }
        "read" => {
            let index: usize = parts.next()?.parse().ok()?;
            reading_item(index)
        }
        "card" | "extra" => lookup(id),
        _ => None,
    }
}

pub fn pattern_items() -> Vec<Item> {
    (0..PATTERNS.len()).filter_map(pattern_item).collect()
}

pub fn swar_items() -> Vec<Item> {
    SWAR.iter()
        .enumerate()
        .map(|(index, text)| glyph_item(format!("swar:{index}"), text, false))
        .collect()
}

pub fn vyanjan_items() -> Vec<Item> {
    VYANJAN
        .iter()
        .enumerate()
        .map(|(index, text)| glyph_item(format!("vyn:{index}"), text, FORMATION.contains(text)))
        .collect()
}

pub fn barakhadi_items(consonant: usize) -> Vec<Item> {
    (0..FORMS.len())
        .filter_map(|form| barakhadi_item(consonant, form))
        .collect()
}

pub fn word_sections() -> Vec<(&'static str, Vec<Item>)> {
    WORD_GROUPS
        .iter()
        .map(|group| {
            let items = WORDS
                .iter()
                .enumerate()
                .filter(|(_, (_, name))| name == group)
                .map(|(index, (text, _))| glyph_item(format!("word:{index}"), text, false))
                .collect();
            (*group, items)
        })
        .filter(|(_, items): &(_, Vec<_>)| !items.is_empty())
        .collect()
}

pub fn reading_items() -> Vec<Item> {
    (0..READINGS.len()).filter_map(reading_item).collect()
}

pub fn all_practice_ids() -> Vec<String> {
    let mut ids = Vec::new();
    ids.extend(pattern_items().into_iter().map(|item| item.id));
    ids.extend(swar_items().into_iter().map(|item| item.id));
    ids.extend(vyanjan_items().into_iter().map(|item| item.id));
    for consonant in 0..VYANJAN.len() {
        ids.extend(barakhadi_items(consonant).into_iter().map(|item| item.id));
    }
    ids.extend((0..WORDS.len()).map(|index| format!("word:{index}")));
    ids.extend((0..READINGS.len()).map(|index| format!("read:{index}")));
    ids.extend(extra_ids());
    ids
}

pub fn questions_for(item: &Item) -> Vec<QuizQ> {
    match &item.kind {
        Kind::Reading { questions, .. } => questions.clone(),
        Kind::Glyph { .. } | Kind::Pattern { .. } => {
            let pool = pool_for(item);
            (0..3)
                .map(|salt| glyph_question(&item.title, &pool, salt))
                .collect()
        }
    }
}

fn pool_for(item: &Item) -> Vec<String> {
    if item.id.starts_with("swar:") {
        SWAR.iter().map(|s| (*s).to_string()).collect()
    } else if item.id.starts_with("vyn:") {
        VYANJAN.iter().map(|s| (*s).to_string()).collect()
    } else if item.id.starts_with("bar:") {
        item.id
            .split(':')
            .nth(1)
            .and_then(|raw| raw.parse().ok())
            .map(|index: usize| {
                barakhadi_items(index)
                    .into_iter()
                    .map(|entry| entry.title)
                    .collect()
            })
            .unwrap_or_default()
    } else if item.id.starts_with("word:") {
        WORDS.iter().map(|(text, _)| (*text).to_string()).collect()
    } else if let Kind::Pattern { .. } = item.kind {
        PATTERNS.iter().map(|pat| pat.name.to_string()).collect()
    } else {
        vec![item.title.clone()]
    }
}

fn glyph_question(target: &str, pool: &[String], salt: usize) -> QuizQ {
    let others: Vec<&str> = pool.iter().map(String::as_str).filter(|c| *c != target).collect();
    let (first, second) = if others.len() >= 2 {
        let a = others[salt % others.len()];
        let mut b_index = (salt + 1) % others.len();
        if others[b_index] == a {
            b_index = (b_index + 1) % others.len();
        }
        (a.to_string(), others[b_index].to_string())
    } else {
        ("घर".to_string(), "जल".to_string())
    };
    let answer = salt % 3;
    let mut choices = vec![first, second, target.to_string()];
    while choices.iter().position(|choice| choice == target).unwrap_or(0) != answer {
        choices.rotate_left(1);
    }
    QuizQ {
        prompt: format!("{target} कहाँ है?"),
        choices,
        answer,
    }
}

fn pattern_item(index: usize) -> Option<Item> {
    let pattern = PATTERNS.get(index)?;
    Some(Item {
        id: format!("pat:{index}"),
        title: pattern.name.to_string(),
        speak: pattern.name.to_string(),
        formation: false,
        kind: Kind::Pattern {
            dots: pattern
                .dots
                .iter()
                .map(|(x, y)| (*x as f32, *y as f32))
                .collect(),
        },
    })
}

fn glyph_item(id: String, text: &str, formation: bool) -> Item {
    Item {
        id,
        title: text.to_string(),
        speak: text.to_string(),
        formation,
        kind: Kind::Glyph {
            text: text.to_string(),
        },
    }
}

fn barakhadi_item(consonant: usize, form: usize) -> Option<Item> {
    let base = *VYANJAN.get(consonant)?;
    let sign = *FORMS.get(form)?;
    let text = format!("{base}{sign}");
    Some(glyph_item(
        format!("bar:{consonant}:{form}"),
        &text,
        FORMATION.contains(&base),
    ))
}

fn reading_item(index: usize) -> Option<Item> {
    let card = READINGS.get(index)?;
    Some(Item {
        id: format!("read:{index}"),
        title: card.title.to_string(),
        speak: card.lines.join(" "),
        formation: false,
        kind: Kind::Reading {
            lines: card.lines.iter().map(|line| (*line).to_string()).collect(),
            questions: card
                .questions
                .iter()
                .map(|question| QuizQ {
                    prompt: question.prompt.to_string(),
                    choices: question.choices.iter().map(|c| (*c).to_string()).collect(),
                    answer: question.answer,
                })
                .collect(),
        },
    })
}

pub fn vyanjan_label(index: usize) -> Option<&'static str> {
    VYANJAN.get(index).copied()
}

pub fn vyanjan_len() -> usize {
    VYANJAN.len()
}
