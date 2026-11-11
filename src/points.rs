use serde::{Deserialize, Serialize};

pub const KEY: &str = "saksham.points.v1";
const DAY_MS: u64 = 86_400_000;

pub const STICKERS: &[&str] = &[
    "star", "moon", "flower", "bird", "fish", "kite", "tree", "cloud", "leaf", "boat",
];

#[derive(Clone, Serialize, Deserialize)]
pub struct Award {
    pub session_id: String,
    pub at_ms: u64,
    pub points: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Store {
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub history: Vec<Award>,
    #[serde(default)]
    pub stickers: Vec<String>,
    #[serde(default)]
    pub pin: Option<String>,
    #[serde(default)]
    pub lock_on: bool,
    #[serde(default = "default_sound")]
    pub sound_on: bool,
    #[serde(default)]
    pub finger_ok: bool,
    #[serde(default = "default_band")]
    pub band_mm: u8,
    #[serde(default)]
    pub next_due: std::collections::BTreeMap<String, u64>,
    #[serde(default)]
    pub streak: std::collections::BTreeMap<String, u32>,
}

fn default_sound() -> bool {
    true
}

fn default_band() -> u8 {
    8
}

impl Default for Store {
    fn default() -> Self {
        Self {
            points: 0,
            history: Vec::new(),
            stickers: Vec::new(),
            pin: None,
            lock_on: false,
            sound_on: true,
            finger_ok: false,
            band_mm: 8,
            next_due: std::collections::BTreeMap::new(),
            streak: std::collections::BTreeMap::new(),
        }
    }
}

pub struct AwardOutcome {
    pub gained: u32,
    pub total: u32,
    pub offer: Vec<String>,
}

impl Store {
    pub fn load() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(raw)) = storage.get_item(KEY) {
                        if let Ok(mut store) = serde_json::from_str::<Store>(&raw) {
                            store.band_mm = 8;
                            return store;
                        }
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Ok(raw) = serde_json::to_string(self) else {
                return;
            };
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(KEY, &raw);
                }
            }
        }
    }

    pub fn award(&mut self, session_id: &str, correct: u8, now: u64) -> AwardOutcome {
        let gained = match correct {
            3 => 5,
            2 => 3,
            _ => 0,
        };
        if gained == 0 {
            self.streak.insert(session_id.to_string(), 0);
            self.next_due.insert(session_id.to_string(), now);
            return AwardOutcome {
                gained: 0,
                total: self.points,
                offer: Vec::new(),
            };
        }
        let old = self.points;
        self.points = self.points.saturating_add(gained);
        self.history.push(Award {
            session_id: session_id.to_string(),
            at_ms: now,
            points: gained,
        });
        if self.history.len() > 50 {
            let extra = self.history.len() - 50;
            self.history.drain(0..extra);
        }
        let streak = self.streak.get(session_id).copied().unwrap_or(0) + 1;
        self.streak.insert(session_id.to_string(), streak);
        let days = match streak {
            1 => 1,
            2 => 3,
            _ => 7,
        };
        self.next_due
            .insert(session_id.to_string(), now.saturating_add(days * DAY_MS));
        let offer = if old / 10 != self.points / 10 {
            self.remaining_stickers(2)
        } else {
            Vec::new()
        };
        AwardOutcome {
            gained,
            total: self.points,
            offer,
        }
    }

    pub fn mark_retry(&mut self, session_id: &str, now: u64) {
        self.streak.insert(session_id.to_string(), 0);
        self.next_due.insert(session_id.to_string(), now);
    }

    pub fn keep_sticker(&mut self, id: &str) {
        if STICKERS.contains(&id) && !self.stickers.iter().any(|kept| kept == id) {
            self.stickers.push(id.to_string());
        }
    }

    fn remaining_stickers(&self, n: usize) -> Vec<String> {
        STICKERS
            .iter()
            .filter(|id| !self.stickers.iter().any(|kept| kept == *id))
            .take(n)
            .map(|id| (*id).to_string())
            .collect()
    }
}

pub fn suggest(store: &Store, ids: &[String]) -> Option<String> {
    let now = now_ms();
    let mut overdue: Option<(u64, &str)> = None;
    let mut soonest: Option<(u64, &str)> = None;
    let mut fresh: Option<&str> = None;
    for id in ids {
        match store.next_due.get(id) {
            None => {
                if fresh.is_none() {
                    fresh = Some(id);
                }
            }
            Some(&due) => {
                if soonest.map(|(at, _)| due < at).unwrap_or(true) {
                    soonest = Some((due, id));
                }
                if due <= now && overdue.map(|(at, _)| due < at).unwrap_or(true) {
                    overdue = Some((due, id));
                }
            }
        }
    }
    overdue
        .map(|(_, id)| id.to_string())
        .or_else(|| fresh.map(|id| id.to_string()))
        .or_else(|| soonest.map(|(_, id)| id.to_string()))
}

pub fn now_ms() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}

pub fn sticker_label(id: &str) -> &'static str {
    match id {
        "star" => "तारा",
        "moon" => "चाँद",
        "flower" => "फूल",
        "bird" => "पक्षी",
        "fish" => "मछली",
        "kite" => "पतंग",
        "tree" => "पेड़",
        "cloud" => "बादल",
        "leaf" => "पत्ता",
        "boat" => "नाव",
        _ => "इनाम",
    }
}
