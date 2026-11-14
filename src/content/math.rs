use super::catalog::{Goal, Paper, Section};
use super::{Item, Kind, Lang, QuizQ};

struct Ask {
    prompt_hi: String,
    prompt_en: String,
    choices_hi: Vec<String>,
    choices_en: Vec<String>,
    answer: usize,
}

struct Card {
    title_hi: String,
    title_en: String,
    speak_hi: String,
    speak_en: String,
    picture: String,
    trace: Option<String>,
    handwriting: bool,
    questions: Vec<Ask>,
}

struct Group {
    title_hi: &'static str,
    title_en: &'static str,
    cards: Vec<Card>,
}

pub fn lookup(id: &str, lang: Lang, seed: u64) -> Option<Item> {
    let rest = id.strip_prefix("math:")?;
    let mut parts = rest.split(':');
    let class_no: u8 = parts.next()?.parse().ok()?;
    let code = parts.next()?;
    let index: usize = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let card = flatten(class_no, code, seed).into_iter().nth(index)?;
    Some(to_item(id, card, lang))
}

pub fn next_id(id: &str) -> Option<String> {
    let rest = id.strip_prefix("math:")?;
    let mut parts = rest.split(':');
    let class_no: u8 = parts.next()?.parse().ok()?;
    let code = parts.next()?;
    let index: usize = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let next = index + 1;
    if next < flatten(class_no, code, 0).len() {
        Some(format!("math:{class_no}:{code}:{next}"))
    } else {
        None
    }
}

pub fn practice_ids() -> Vec<String> {
    let mut ids = Vec::new();
    for class_no in [1, 2] {
        for code in ["count", "add", "times", "share"] {
            for index in 0..flatten(class_no, code, 0).len() {
                ids.push(format!("math:{class_no}:{code}:{index}"));
            }
        }
    }
    ids
}

pub fn bank(class_no: u8, goal: Goal, lang: Lang) -> Vec<Section> {
    let Some(code) = code_of(goal) else {
        return Vec::new();
    };
    let mut sections = Vec::new();
    let mut index = 0usize;
    for group in grouped(class_no, code, 0) {
        let mut items = Vec::new();
        for card in group.cards {
            let id = format!("math:{class_no}:{code}:{index}");
            index += 1;
            items.push(to_item(&id, card, lang));
        }
        sections.push(Section {
            title: if lang == Lang::En {
                group.title_en
            } else {
                group.title_hi
            },
            items,
        });
    }
    sections
}

pub fn paper(class_no: u8, goal: Goal, lang: Lang, seed: u64) -> Paper {
    let salt = match goal {
        Goal::Count => 1,
        Goal::AddSub => 2,
        Goal::Tables => 3,
        Goal::Divide => 4,
        _ => 0,
    };
    let mut rng = Rng::new(seed ^ ((class_no as u64) << 16) ^ salt);
    let (mut asks, write_digits) = match goal {
        Goal::Count => (count_paper(class_no, &mut rng), true),
        Goal::AddSub => (add_paper(class_no, &mut rng), true),
        Goal::Tables => (times_paper(class_no, &mut rng), false),
        Goal::Divide => (share_paper(class_no, &mut rng), false),
        _ => (Vec::new(), false),
    };
    rng.shuffle(&mut asks);
    asks = asks
        .into_iter()
        .map(|ask| shuffle_ask(ask, &mut rng))
        .collect();
    let write = if write_digits {
        Some(write_target(class_no, &asks, &mut rng))
    } else {
        None
    };
    Paper {
        write,
        questions: asks.into_iter().map(|ask| to_quiz(ask, lang)).collect(),
    }
}

fn code_of(goal: Goal) -> Option<&'static str> {
    Some(match goal {
        Goal::Count => "count",
        Goal::AddSub => "add",
        Goal::Tables => "times",
        Goal::Divide => "share",
        _ => return None,
    })
}

fn grouped(class_no: u8, code: &str, seed: u64) -> Vec<Group> {
    match (class_no, code) {
        (1, "count") => vec![
            group("अंक लिखो", "Write", digits()),
            group("आगे", "Forward", (1..=10).map(forward_card).collect()),
            group("दस बनाओ", "Make 10", make_ten()),
            group(
                "बीस तक",
                "Up to 20",
                vec![read_teen(11), read_teen(15), read_teen(20)],
            ),
            group(
                "पीछे",
                "Backward",
                vec![
                    back_card(&[5, 4, 3]),
                    back_card(&[10, 9, 8]),
                    back_card(&[3, 2, 1]),
                    back_card(&[20, 19, 18]),
                ],
            ),
            group(
                "छोड़कर",
                "Skip",
                vec![
                    skip_card(&[2, 4, 6], 2),
                    skip_card(&[4, 6, 8], 2),
                    skip_card(&[8, 10, 12], 2),
                    skip_card(&[5, 10, 15], 5),
                    skip_card(&[0, 10], 10),
                ],
            ),
            group(
                "पहले, बाद, बीच",
                "Before, after, between",
                vec![
                    after_card(7),
                    before_card(12),
                    between_card(8, 10),
                    after_card(4),
                    before_card(15),
                    between_card(6, 8),
                ],
            ),
            group(
                "क्रम",
                "Order",
                vec![
                    order_card(&[4, 9, 2], true),
                    order_card(&[1, 8, 3], true),
                    order_card(&[9, 4, 2], false),
                    order_card(&[7, 2, 5], false),
                ],
            ),
        ],
        (_, "count") => vec![
            group(
                "संख्या",
                "Numbers",
                vec![
                    place_card(23, false),
                    place_card(45, false),
                    place_card(25, true),
                    place_card(100, true),
                    place_card(200, false),
                    place_card(305, false),
                ],
            ),
            group(
                "पीछे",
                "Backward",
                vec![
                    back_card(&[50, 49, 48]),
                    back_card(&[100, 99, 98]),
                    back_card(&[47, 46, 45]),
                ],
            ),
            group(
                "छोड़कर",
                "Skip",
                vec![
                    skip_card(&[12, 14, 16], 2),
                    skip_card(&[3, 6, 9], 3),
                    skip_card(&[20, 25, 30], 5),
                    skip_card(&[40, 50, 60], 10),
                ],
            ),
            group(
                "पहले, बाद, बीच",
                "Before, after, between",
                vec![
                    after_card(29),
                    before_card(30),
                    between_card(29, 31),
                    after_card(99),
                    before_card(100),
                    between_card(249, 251),
                ],
            ),
            group(
                "क्रम",
                "Order",
                vec![
                    order_card(&[14, 41, 4], true),
                    order_card(&[18, 80, 8], false),
                    order_card(&[23, 32, 12, 21], true),
                    order_card(&[120, 210, 102], true),
                ],
            ),
        ],
        (1, "add") => vec![group(
            "जोड़ और घटाव",
            "Add and subtract",
            vec![
                add_card(3, 2, "3 आम और 2 आम।", "3 mangoes and 2 mangoes."),
                add_card(2, 2, "2 गेंद और 2 गेंद।", "2 balls and 2 balls."),
                add_card(4, 1, "4 बच्चे और 1 बच्चा।", "4 children and 1 child."),
                add_card(6, 3, "6 फूल और 3 फूल।", "6 flowers and 3 flowers."),
                add_card(5, 4, "5 किताब और 4 किताब।", "5 books and 4 books."),
                add_card(7, 2, "7 पक्षी और 2 पक्षी।", "7 birds and 2 birds."),
                add_card(1, 6, "1 सेब और 6 सेब।", "1 apple and 6 apples."),
                add_card(5, 5, "5 पत्ता और 5 पत्ता।", "5 leaves and 5 leaves."),
                sub_card(5, 2, "5 आम में से 2 आम गए।", "5 mangoes, 2 are eaten."),
                sub_card(6, 1, "6 गेंद में से 1 गई।", "6 balls, 1 rolls away."),
                sub_card(8, 3, "8 फूल में से 3 गए।", "8 flowers, 3 are picked."),
                sub_card(9, 2, "9 पक्षी में से 2 उड़ गए।", "9 birds, 2 fly away."),
                sub_card(10, 4, "10 में से 4 गए।", "10, take away 4."),
                sub_card(7, 3, "7 में से 3 गए।", "7, take away 3."),
            ],
        )],
        (_, "add") => vec![group(
            "जोड़ और घटाव",
            "Add and subtract",
            vec![
                plain_add(23, 14),
                plain_add(40, 25),
                plain_add(32, 16),
                plain_add(51, 8),
                plain_add(28, 15),
                plain_add(47, 36),
                plain_add(56, 19),
                plain_sub(47, 23),
                plain_sub(68, 15),
                plain_sub(90, 40),
                plain_sub(40, 16),
                plain_sub(52, 27),
                plain_sub(70, 38),
            ],
        )],
        (1, "times") => {
            let mut rng = Rng::new(seed ^ 0x71);
            vec![group(
                "समूह",
                "Groups",
                vec![
                    times_tile(2, 20, &mut rng, seed == 0, true),
                    times_tile(5, 20, &mut rng, seed == 0, true),
                    times_tile(10, 20, &mut rng, seed == 0, true),
                ],
            )]
        }
        (_, "times") => {
            let mut rng = Rng::new(seed ^ 0x72);
            vec![group(
                "पहाड़े",
                "Tables",
                (2..=10)
                    .map(|n| times_tile(n, 100, &mut rng, seed == 0, false))
                    .collect(),
            )]
        }
        (1, "share") => vec![group(
            "घटाकर बाँटो",
            "Take away",
            vec![
                share_card(4, 2),
                share_card(6, 2),
                share_card(8, 2),
                share_card(10, 5),
                share_card(10, 2),
            ],
        )],
        (_, "share") => vec![group(
            "घटाकर बाँटो",
            "Take away",
            vec![
                share_card(8, 2),
                share_card(6, 2),
                share_card(10, 2),
                share_card(9, 3),
                share_card(12, 3),
                share_card(8, 4),
                share_card(12, 4),
                share_card(10, 5),
                share_card(15, 5),
                share_card(20, 5),
                share_card(12, 6),
                share_card(14, 7),
                share_card(16, 8),
                share_card(18, 9),
                share_card(20, 10),
            ],
        )],
        _ => Vec::new(),
    }
}

fn flatten(class_no: u8, code: &str, seed: u64) -> Vec<Card> {
    grouped(class_no, code, seed)
        .into_iter()
        .flat_map(|group| group.cards)
        .enumerate()
        .map(|(index, card)| {
            let mut rng = Rng::new(seed ^ ((index as u64 + 1) << 32) ^ 0xC0FFEE);
            shuffle_card(card, &mut rng)
        })
        .collect()
}

fn group(title_hi: &'static str, title_en: &'static str, cards: Vec<Card>) -> Group {
    Group {
        title_hi,
        title_en,
        cards,
    }
}

fn digits() -> Vec<Card> {
    (0..10).map(digit_card).collect()
}

fn digit_card(n: u8) -> Card {
    let label = n.to_string();
    let speak_hi = hi_word(n as u16).to_string();
    let speak_en = en_word(n as u16).to_string();
    let questions = if n == 0 {
        vec![
            nums("0 के बाद क्या आएगा?", "What comes after 0?", "1", "2", "3", 0),
            nums(
                "खाली का अंक कौन सा है?",
                "Which digit means none?",
                "0",
                "4",
                "5",
                1,
            ),
            nums("1 के बाद क्या आएगा?", "What comes after 1?", "2", "0", "3", 2),
        ]
    } else if n == 9 {
        vec![
            nums(
                "9 से पहले क्या आएगा?",
                "What comes before 9?",
                "8",
                "7",
                "6",
                0,
            ),
            nums(
                "8 के बाद क्या आएगा?",
                "What comes after 8?",
                "9",
                "7",
                "10",
                1,
            ),
            nums(
                "9 के बाद क्या आएगा?",
                "What comes after 9?",
                "10",
                "8",
                "11",
                2,
            ),
        ]
    } else {
        let after = (n + 1).to_string();
        let before = (n - 1).to_string();
        let (aw1, aw2) = two_wrongs(n as i32 + 1, 0, 12);
        let (bw1, bw2) = two_wrongs(n as i32 - 1, 0, 12);
        let (mw1, mw2) = two_wrongs(n as i32, 0, 12);
        vec![
            nums(
                &format!("{n} के बाद क्या आएगा?"),
                &format!("What comes after {n}?"),
                &after,
                &aw1,
                &aw2,
                0,
            ),
            nums(
                &format!("{n} से पहले क्या आएगा?"),
                &format!("What comes before {n}?"),
                &before,
                &bw1,
                &bw2,
                1,
            ),
            nums(
                &format!("{} और {} के बीच क्या है?", n - 1, n + 1),
                &format!("What is between {} and {}?", n - 1, n + 1),
                &label,
                &mw1,
                &mw2,
                2,
            ),
        ]
    };
    num_card(
        &label, &speak_hi, &speak_en, &label, &label, true, questions,
    )
}

fn forward_card(n: u8) -> Card {
    let picture = dots(n);
    let label = n.to_string();
    let (cw1, cw2) = two_wrongs(n as i32, 0, 12);
    let next = n + 1;
    let prev = n - 1;
    let (aw1, aw2) = two_wrongs(next as i32, 0, 12);
    let (bw1, bw2) = two_wrongs(prev as i32, 0, 12);
    let after_prompt_n = n;
    let after_answer = next;
    num_card(
        &label,
        &format!("ये {}", hi_word(n as u16)),
        &format!("These are {}", en_word(n as u16)),
        &picture,
        &label,
        false,
        vec![
            nums(
                &format!("{picture} कितने हैं?"),
                &format!("How many? {picture}"),
                &label,
                &cw1,
                &cw2,
                0,
            ),
            nums(
                &format!("{after_prompt_n} के बाद क्या आएगा?"),
                &format!("What comes after {after_prompt_n}?"),
                &after_answer.to_string(),
                &aw1,
                &aw2,
                1,
            ),
            nums(
                &format!("{n} से पहले क्या आएगा?"),
                &format!("What comes before {n}?"),
                &prev.to_string(),
                &bw1,
                &bw2,
                2,
            ),
        ],
    )
}

fn make_ten() -> Vec<Card> {
    [(1, 9), (2, 8), (3, 7), (4, 6), (5, 5)]
        .into_iter()
        .map(|(a, b)| {
            let picture = format!("{a} + {b}");
            let (w1, w2) = two_wrongs(b, 0, 10);
            let (third, t1, t2) = if a == b {
                ("11".to_string(), "10".to_string(), "9".to_string())
            } else {
                let (x, y) = two_wrongs(a, 0, 10);
                (a.to_string(), x, y)
            };
            let third_hi = if a == b {
                "10 के बाद क्या आएगा?".to_string()
            } else {
                format!("{a} कौन सी संख्या है?")
            };
            let third_en = if a == b {
                "What comes after 10?".to_string()
            } else {
                format!("Which number is {a}?")
            };
            num_card(
                &picture,
                &format!("{} और {} मिलकर दस", hi_word(a as u16), hi_word(b as u16)),
                &format!("{} and {} make ten", en_word(a as u16), en_word(b as u16)),
                &picture,
                "10",
                false,
                vec![
                    nums(
                        &format!("{picture} = ?"),
                        &format!("{picture} = ?"),
                        "10",
                        "9",
                        "8",
                        0,
                    ),
                    nums(
                        &format!("{a} के साथ क्या जोड़ें कि 10 बने?"),
                        &format!("What joins {a} to make 10?"),
                        &b.to_string(),
                        &w1,
                        &w2,
                        1,
                    ),
                    nums(&third_hi, &third_en, &third, &t1, &t2, 2),
                ],
            )
        })
        .collect()
}

fn read_teen(n: u16) -> Card {
    let label = n.to_string();
    let (w1, w2) = two_wrongs(n as i32, 0, 21);
    let (a1, a2) = two_wrongs(n as i32 + 1, 0, 22);
    let (b1, b2) = two_wrongs(n as i32 - 1, 0, 21);
    num_card(
        &label,
        &format!("यह {}", hi_word(n)),
        &format!("This is {}", en_word(n)),
        &label,
        &label,
        false,
        vec![
            nums(
                &format!("{label} कौन सी संख्या है?"),
                &format!("Which number is {label}?"),
                &label,
                &w1,
                &w2,
                0,
            ),
            nums(
                &format!("{label} के बाद क्या आएगा?"),
                &format!("What comes after {label}?"),
                &(n + 1).to_string(),
                &a1,
                &a2,
                1,
            ),
            nums(
                &format!("{label} से पहले क्या आएगा?"),
                &format!("What comes before {label}?"),
                &(n - 1).to_string(),
                &b1,
                &b2,
                2,
            ),
        ],
    )
}

fn back_card(seq: &[u16]) -> Card {
    let next = seq[seq.len() - 1] - 1;
    let shown = join(seq);
    let picture = format!("{shown}, __");
    let (w1, w2) = two_wrongs(next as i32, 0, next as i32 + 4);
    num_card(
        &shown,
        &format!("पीछे। आगे {}", hi_word(next)),
        &format!("Backward. Next is {}", en_word(next)),
        &picture,
        &next.to_string(),
        false,
        vec![
            nums(
                &format!("{picture} आगे क्या आएगा?"),
                &format!("{picture} What comes next?"),
                &next.to_string(),
                &w1,
                &w2,
                0,
            ),
            words(
                "हम किस ओर गिन रहे हैं?",
                "Which way are we counting?",
                ["आगे", "पीछे", "छोड़कर"],
                ["Forward", "Backward", "Skip"],
                1,
            ),
            nums(
                "गिनती कहाँ से शुरू हुई?",
                "Where did this count start?",
                &seq[0].to_string(),
                &seq[1].to_string(),
                &next.to_string(),
                2,
            ),
        ],
    )
}

fn skip_card(seq: &[u16], step: u16) -> Card {
    let next = seq[seq.len() - 1] + step;
    let shown = join(seq);
    let picture = format!("{shown}, __");
    let (w1, w2) = two_wrongs(next as i32, 0, next as i32 + 6);
    let (s1, s2) = two_wrongs(step as i32, 1, 10);
    let first = seq[0];
    let last = seq[seq.len() - 1];
    let third = if first == step { last } else { first };
    let third_wrong = if first != third { first } else { seq[1] };
    num_card(
        &shown,
        &format!("{} छोड़कर। आगे {}", hi_word(step), hi_word(next)),
        &format!("Skip {}. Next is {}", en_word(step), en_word(next)),
        &picture,
        &next.to_string(),
        false,
        vec![
            nums(
                &format!("{picture} आगे क्या आएगा?"),
                &format!("{picture} What comes next?"),
                &next.to_string(),
                &w1,
                &w2,
                0,
            ),
            nums(
                "कितने छोड़कर गिन रहे हैं?",
                "We skip by how many?",
                &step.to_string(),
                &s1,
                &s2,
                1,
            ),
            nums(
                "इस पंक्ति की पहली संख्या कौन सी है?",
                "What is the first number in this row?",
                &third.to_string(),
                &third_wrong.to_string(),
                &next.to_string(),
                2,
            ),
        ],
    )
}

fn after_card(n: u16) -> Card {
    let answer = n + 1;
    let (w1, w2) = two_wrongs(answer as i32, 0, answer as i32 + 3);
    neighbour("बाद", "After", n, answer, &format!("{n}, __"), &w1, &w2)
}

fn before_card(n: u16) -> Card {
    let answer = n - 1;
    let (w1, w2) = two_wrongs(answer as i32, 0, n as i32 + 2);
    neighbour("पहले", "Before", n, answer, &format!("__, {n}"), &w1, &w2)
}

fn between_card(left: u16, right: u16) -> Card {
    let answer = left + 1;
    let (w1, w2) = two_wrongs(answer as i32, 0, right as i32 + 2);
    let picture = format!("{left} __ {right}");
    num_card(
        &format!("{left}, {right}"),
        &format!(
            "{} और {} के बीच {}",
            hi_word(left),
            hi_word(right),
            hi_word(answer)
        ),
        &format!(
            "Between {} and {} is {}",
            en_word(left),
            en_word(right),
            en_word(answer)
        ),
        &picture,
        &answer.to_string(),
        false,
        vec![
            nums(
                &format!("{left} और {right} के बीच क्या है?"),
                &format!("What is between {left} and {right}?"),
                &answer.to_string(),
                &w1,
                &w2,
                0,
            ),
            nums(
                "बाएँ कौन सी संख्या है?",
                "Which number is on the left?",
                &left.to_string(),
                &answer.to_string(),
                &right.to_string(),
                1,
            ),
            nums(
                "दाएँ कौन सी संख्या है?",
                "Which number is on the right?",
                &right.to_string(),
                &left.to_string(),
                &answer.to_string(),
                2,
            ),
        ],
    )
}

fn neighbour(
    hi_kind: &str,
    en_kind: &str,
    n: u16,
    answer: u16,
    picture: &str,
    w1: &str,
    w2: &str,
) -> Card {
    let hi_prompt = if hi_kind == "बाद" {
        format!("{n} के बाद क्या आएगा?")
    } else {
        format!("{n} से पहले क्या आएगा?")
    };
    let en_prompt = format!("What comes {} {n}?", en_kind.to_lowercase());
    let third = if hi_kind == "बाद" {
        answer + 1
    } else {
        n.saturating_sub(2)
    };
    fact(
        &format!("{hi_kind} {n}"),
        &format!("{en_kind} {n}"),
        &format!("{hi_prompt} {}", hi_word(answer)),
        &format!("{en_prompt} {}", en_word(answer)),
        picture,
        &answer.to_string(),
        false,
        vec![
            nums(&hi_prompt, &en_prompt, &answer.to_string(), w1, w2, 0),
            nums(
                &format!("{answer} के पास कौन सी है?"),
                &format!("Which is next to {answer}?"),
                &n.to_string(),
                &answer.to_string(),
                &third.to_string(),
                1,
            ),
            nums(
                &format!("{third} कौन सी संख्या है?"),
                &format!("Which number is {third}?"),
                &third.to_string(),
                &answer.to_string(),
                &n.to_string(),
                2,
            ),
        ],
    )
}

fn order_card(values: &[u16], ascending: bool) -> Card {
    let shown = join(values);
    let (correct, wrong1, wrong2) = order_choices(values, ascending);
    let mut sorted = values.to_vec();
    if ascending {
        sorted.sort_unstable();
    } else {
        sorted.sort_unstable_by(|a, b| b.cmp(a));
    }
    let first = sorted[0];
    let last = sorted[sorted.len() - 1];
    let (f1, f2) = pair_from(values, first);
    let (l1, l2) = pair_from(values, last);
    let (title_hi, title_en, speak_hi, speak_en, q1_hi, q1_en, q2_hi, q2_en, q3_hi, q3_en) =
        if ascending {
            (
                format!("बढ़ता {shown}"),
                format!("Up {shown}"),
                format!("{shown}। छोटे से बड़ा। पहले {}", hi_word(first)),
                format!("{shown}. Smallest to largest. First is {}", en_word(first)),
                format!("{shown} का बढ़ता क्रम क्या है?"),
                format!("Smallest to largest for {shown}?"),
                format!("{shown} में सबसे छोटी कौन सी है?"),
                format!("Which is smallest in {shown}?"),
                format!("{shown} में सबसे बड़ी कौन सी है?"),
                format!("Which is largest in {shown}?"),
            )
        } else {
            (
                format!("घटता {shown}"),
                format!("Down {shown}"),
                format!("{shown}। बड़े से छोटा। पहले {}", hi_word(first)),
                format!("{shown}. Largest to smallest. First is {}", en_word(first)),
                format!("{shown} का घटता क्रम क्या है?"),
                format!("Largest to smallest for {shown}?"),
                format!("{shown} में सबसे बड़ी कौन सी है?"),
                format!("Which is largest in {shown}?"),
                format!("{shown} में सबसे छोटी कौन सी है?"),
                format!("Which is smallest in {shown}?"),
            )
        };
    fact(
        &title_hi,
        &title_en,
        &speak_hi,
        &speak_en,
        &shown,
        &first.to_string(),
        false,
        vec![
            nums(&q1_hi, &q1_en, &correct, &wrong1, &wrong2, 0),
            nums(&q2_hi, &q2_en, &first.to_string(), &f1, &f2, 1),
            nums(&q3_hi, &q3_en, &last.to_string(), &l1, &l2, 2),
        ],
    )
}

fn place_card(n: u16, handwriting: bool) -> Card {
    let hundreds = n / 100;
    let tens = (n / 10) % 10;
    let ones = n % 10;
    let label = n.to_string();
    let (nw1, nw2) = two_wrongs(n as i32, 0, n as i32 + 20);
    let speak_hi = if n >= 100 {
        format!(
            "{}। {} सैकड़ा, {} दहाई, {} इकाई।",
            hi_word(n),
            hi_word(hundreds),
            hi_word(tens),
            hi_word(ones)
        )
    } else {
        format!(
            "{}। {} दहाई, {} इकाई।",
            hi_word(n),
            hi_word(tens),
            hi_word(ones)
        )
    };
    let speak_en = if n >= 100 {
        format!(
            "{}. {} hundreds, {} tens, {} ones.",
            en_word(n),
            en_word(hundreds),
            en_word(tens),
            en_word(ones)
        )
    } else {
        format!(
            "{}. {} tens, {} ones.",
            en_word(n),
            en_word(tens),
            en_word(ones)
        )
    };
    let questions = if n >= 100 {
        let (h1, h2) = two_wrongs(hundreds as i32, 0, 9);
        let (t1, t2) = two_wrongs(tens as i32, 0, 9);
        let (o1, o2) = two_wrongs(ones as i32, 0, 9);
        vec![
            nums(
                &format!("{label} में कितने सैकड़े हैं?"),
                &format!("How many hundreds in {label}?"),
                &hundreds.to_string(),
                &h1,
                &h2,
                0,
            ),
            nums(
                &format!("{label} में कितनी दहाई हैं?"),
                &format!("How many tens in {label}?"),
                &tens.to_string(),
                &t1,
                &t2,
                1,
            ),
            if tens == ones {
                nums(
                    &format!("{label} कौन सी संख्या है?"),
                    &format!("Which number is {label}?"),
                    &label,
                    &nw1,
                    &nw2,
                    2,
                )
            } else {
                nums(
                    &format!("{label} में कितनी इकाई हैं?"),
                    &format!("How many ones in {label}?"),
                    &ones.to_string(),
                    &o1,
                    &o2,
                    2,
                )
            },
        ]
    } else {
        let (t1, t2) = two_wrongs(tens as i32, 0, 9);
        let (o1, o2) = two_wrongs(ones as i32, 0, 9);
        vec![
            nums(
                &format!("{label} में कितनी दहाई हैं?"),
                &format!("How many tens in {label}?"),
                &tens.to_string(),
                &t1,
                &t2,
                0,
            ),
            nums(
                &format!("{label} में कितनी इकाई हैं?"),
                &format!("How many ones in {label}?"),
                &ones.to_string(),
                &o1,
                &o2,
                1,
            ),
            nums(
                &format!("{label} कौन सी संख्या है?"),
                &format!("Which number is {label}?"),
                &label,
                &nw1,
                &nw2,
                2,
            ),
        ]
    };
    num_card(
        &label,
        &speak_hi,
        &speak_en,
        &label,
        &label,
        handwriting,
        questions,
    )
}

fn add_card(a: u16, b: u16, story_hi: &str, story_en: &str) -> Card {
    let c = a + b;
    let picture = format!("{a} + {b}");
    let (sh, se) = (
        format!("{} जमा {} बराबर {}", hi_word(a), hi_word(b), hi_word(c)),
        format!("{} plus {} is {}", en_word(a), en_word(b), en_word(c)),
    );
    let (w1, w2) = two_wrongs(c as i32, 0, c as i32 + 8);
    let (mw1, mw2) = two_wrongs(b as i32, 0, c as i32 + 2);
    num_card(
        &picture,
        &sh,
        &se,
        &picture,
        &c.to_string(),
        false,
        vec![
            nums(
                &format!("{story_hi} कितने?"),
                &format!("{story_en} How many?"),
                &c.to_string(),
                &w1,
                &w2,
                0,
            ),
            nums(
                &format!("{a} + ? = {c}"),
                &format!("{a} + ? = {c}"),
                &b.to_string(),
                &mw1,
                &mw2,
                1,
            ),
            if a == b {
                nums(
                    &format!("{c} के बाद क्या आएगा?"),
                    &format!("What comes after {c}?"),
                    &(c + 1).to_string(),
                    &c.to_string(),
                    &a.to_string(),
                    2,
                )
            } else {
                nums(
                    &format!("{c} - {b} = ?"),
                    &format!("{c} - {b} = ?"),
                    &a.to_string(),
                    &b.to_string(),
                    &c.to_string(),
                    2,
                )
            },
        ],
    )
}

fn sub_card(a: u16, b: u16, story_hi: &str, story_en: &str) -> Card {
    let c = a - b;
    let picture = format!("{a} - {b}");
    let (w1, w2) = two_wrongs(c as i32, 0, a as i32);
    let (t1, t2) = two_wrongs(b as i32, 0, a as i32);
    num_card(
        &picture,
        &format!("{} में से {}। बचा {}", hi_word(a), hi_word(b), hi_word(c)),
        &format!(
            "{} take away {} leaves {}",
            en_word(a),
            en_word(b),
            en_word(c)
        ),
        &picture,
        &c.to_string(),
        false,
        vec![
            nums(
                &format!("{story_hi} कितने बचे?"),
                &format!("{story_en} How many are left?"),
                &c.to_string(),
                &w1,
                &w2,
                0,
            ),
            nums(
                "शुरू में कितने थे?",
                "How many were there at the start?",
                &a.to_string(),
                &(a + 1).to_string(),
                &c.to_string(),
                1,
            ),
            if b == c {
                nums(
                    &format!("{a} के बाद क्या आएगा?"),
                    &format!("What comes after {a}?"),
                    &(a + 1).to_string(),
                    &a.to_string(),
                    &c.to_string(),
                    2,
                )
            } else {
                nums(
                    "कितने घटाए?",
                    "How many did we take away?",
                    &b.to_string(),
                    &t1,
                    &t2,
                    2,
                )
            },
        ],
    )
}

fn plain_add(a: u16, b: u16) -> Card {
    add_card(a, b, &format!("{a} + {b}।"), &format!("{a} + {b}."))
}

fn plain_sub(a: u16, b: u16) -> Card {
    sub_card(a, b, &format!("{a} - {b}।"), &format!("{a} - {b}."))
}

fn times_tile(n: u16, limit: u16, rng: &mut Rng, learn: bool, groups_only: bool) -> Card {
    let max_k = (limit / n).clamp(1, 10);
    let mut ks = if learn {
        [2u16, 3, 4]
            .into_iter()
            .filter(|k| *k <= max_k)
            .collect::<Vec<_>>()
    } else {
        let mut all: Vec<u16> = (1..=max_k).collect();
        rng.shuffle(&mut all);
        all
    };
    if ks.is_empty() {
        ks.push(1.min(max_k));
    }
    let mut questions = Vec::new();
    let mut used = std::collections::BTreeSet::new();
    let keep =
        |ask: Ask, questions: &mut Vec<Ask>, used: &mut std::collections::BTreeSet<String>| {
            if questions.len() == 3 {
                return;
            }
            let answer = ask.choices_hi[ask.answer].clone();
            if used.insert(answer) {
                questions.push(ask);
            }
        };
    for k in ks.iter().copied() {
        let product = n * k;
        let (w1, w2) = two_wrongs(product as i32, 0, product as i32 + 8);
        keep(
            nums(
                &format!("{n} × {k} = ?"),
                &format!("{n} × {k} = ?"),
                &product.to_string(),
                &w1,
                &w2,
                0,
            ),
            &mut questions,
            &mut used,
        );
    }
    for k in ks.iter().copied() {
        let (w1, w2) = two_wrongs(k as i32, 1, 12);
        keep(
            nums(
                "कितनी बार लिया?",
                "How many times?",
                &k.to_string(),
                &w1,
                &w2,
                0,
            ),
            &mut questions,
            &mut used,
        );
    }
    let (w1, w2) = two_wrongs(n as i32, 1, 12);
    keep(
        nums(
            "हर समूह में कितने हैं?",
            "How many are in each group?",
            &n.to_string(),
            &w1,
            &w2,
            0,
        ),
        &mut questions,
        &mut used,
    );
    let mut extra = n.saturating_add(1);
    while questions.len() < 3 {
        let (ew1, ew2) = two_wrongs(extra as i32, 0, extra as i32 + 4);
        let ask = nums(
            &format!("{extra} के बाद क्या आएगा?"),
            &format!("What comes after {extra}?"),
            &(extra + 1).to_string(),
            &ew1,
            &ew2,
            0,
        );
        keep(ask, &mut questions, &mut used);
        extra += 1;
    }
    let k0 = ks[0];
    let product0 = n * k0;
    let picture = if groups_only && learn {
        if k0 == 1 {
            n.to_string()
        } else {
            (0..k0)
                .map(|_| n.to_string())
                .collect::<Vec<_>>()
                .join(" + ")
        }
    } else {
        ks.iter()
            .take(3)
            .map(|k| format!("{n} × {k}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let (title_hi, title_en) = if groups_only {
        (format!("{n} के समूह"), format!("Groups of {n}"))
    } else {
        (format!("{n} का पहाड़ा"), format!("Table of {n}"))
    };
    let (speak_hi, speak_en) = if learn {
        (
            format!(
                "{title_hi}। {} बार {}। {}",
                hi_word(n),
                hi_word(k0),
                hi_word(product0)
            ),
            format!(
                "{title_en}. {} times {} is {}.",
                en_word(n),
                en_word(k0),
                en_word(product0)
            ),
        )
    } else if groups_only {
        (
            format!("{} के समूह", hi_word(n)),
            format!("Groups of {}", en_word(n)),
        )
    } else {
        (
            format!("{} का पहाड़ा", hi_word(n)),
            format!("Table of {}", en_word(n)),
        )
    };
    fact(
        &title_hi,
        &title_en,
        &speak_hi,
        &speak_en,
        &picture,
        &product0.to_string(),
        false,
        questions,
    )
}

fn share_card(total: u16, step: u16) -> Card {
    let times = total / step;
    let picture = std::iter::once(total.to_string())
        .chain((0..times).map(|_| step.to_string()))
        .collect::<Vec<_>>()
        .join(" - ");
    let (w1, w2) = two_wrongs(times as i32, 1, 10);
    let (s1, s2) = two_wrongs(step as i32, 1, 10);
    num_card(
        &format!("{total} - {step}"),
        &format!(
            "{} में से {} बार-बार। {} बार।",
            hi_word(total),
            hi_word(step),
            hi_word(times)
        ),
        &format!(
            "Take {} from {} again and again. {} times.",
            en_word(step),
            en_word(total),
            en_word(times)
        ),
        &picture,
        &times.to_string(),
        false,
        vec![
            nums(
                &format!("{picture}। कितनी बार?"),
                &format!("{picture}. How many times?"),
                &times.to_string(),
                &w1,
                &w2,
                0,
            ),
            if step == times {
                nums(
                    "शुरू में कितने थे?",
                    "How many were there at the start?",
                    &total.to_string(),
                    &step.to_string(),
                    "0",
                    1,
                )
            } else {
                nums(
                    "हर बार कितने घटाए?",
                    "How many did we take away each time?",
                    &step.to_string(),
                    &s1,
                    &s2,
                    1,
                )
            },
            nums(
                "अंत में क्या बचा?",
                "What is left at the end?",
                "0",
                "1",
                &total.to_string(),
                2,
            ),
        ],
    )
}

fn count_paper(class_no: u8, rng: &mut Rng) -> Vec<Ask> {
    let wide = class_no != 1;
    let after_n = rng.gen_range(if wide { 10 } else { 0 }, if wide { 98 } else { 18 });
    let before_n = rng.gen_range(if wide { 12 } else { 2 }, if wide { 100 } else { 20 });
    let left = if wide && rng.next_u32() % 4 == 0 {
        rng.gen_range(100, 250)
    } else {
        rng.gen_range(if wide { 10 } else { 0 }, if wide { 90 } else { 18 })
    };
    let step = if wide {
        [2u16, 3, 5, 10][(rng.next_u32() as usize) % 4]
    } else {
        [2u16, 5, 10][(rng.next_u32() as usize) % 3]
    };
    let cap: u16 = if wide { 100 } else { 20 };
    let max_start = cap.saturating_sub(2 * step);
    let start = rng.gen_range(0, max_start);
    let next = start + 3 * step;
    let mut asks = vec![
        number_ask(
            &format!("{after_n} के बाद क्या आएगा?"),
            &format!("What comes after {after_n}?"),
            after_n as i32 + 1,
            0,
            cap as i32 + 2,
        ),
        number_ask(
            &format!("{before_n} से पहले क्या आएगा?"),
            &format!("What comes before {before_n}?"),
            before_n as i32 - 1,
            0,
            cap as i32,
        ),
        number_ask(
            &format!("{left} और {} के बीच क्या है?", left + 2),
            &format!("What is between {left} and {}?", left + 2),
            left as i32 + 1,
            0,
            left as i32 + 4,
        ),
        number_ask(
            &format!(
                "{start}, {}, {} के बाद क्या आएगा?",
                start + step,
                start + 2 * step
            ),
            &format!(
                "What comes after {start}, {}, {}?",
                start + step,
                start + 2 * step
            ),
            next as i32,
            0,
            next as i32 + 6,
        ),
        order_ask(rng, if wide { 99 } else { 20 }),
    ];
    if wide && rng.next_u32() % 2 == 0 {
        let place = rng.gen_range(10, 99);
        asks[4] = number_ask(
            &format!("{place} में कितनी दहाई हैं?"),
            &format!("How many tens in {place}?"),
            (place / 10) as i32,
            0,
            9,
        );
    }
    asks
}

fn add_paper(class_no: u8, rng: &mut Rng) -> Vec<Ask> {
    if class_no == 1 {
        let mut asks = Vec::new();
        for index in 0..5 {
            if index % 2 == 0 {
                let a = rng.gen_range(1, 8);
                let b = rng.gen_range(1, 10 - a);
                let c = a + b;
                asks.push(number_ask(
                    &format!("{a} + {b} = ?"),
                    &format!("{a} + {b} = ?"),
                    c as i32,
                    0,
                    12,
                ));
            } else {
                let a = rng.gen_range(3, 10);
                let b = rng.gen_range(1, a);
                asks.push(number_ask(
                    &format!("{a} - {b} = ?"),
                    &format!("{a} - {b} = ?"),
                    (a - b) as i32,
                    0,
                    a as i32,
                ));
            }
        }
        asks
    } else {
        let (a, b, c) = add_pair(rng, false);
        let (d, e, f) = add_pair(rng, true);
        let (g, h, i) = sub_pair(rng, false);
        let (j, k, l) = sub_pair(rng, true);
        let carry = rng.next_u32() % 2 == 0;
        let (m, n, o) = add_pair(rng, carry);
        vec![
            number_ask(
                &format!("{a} + {b} = ?"),
                &format!("{a} + {b} = ?"),
                c as i32,
                0,
                110,
            ),
            number_ask(
                &format!("{d} + {e} = ?"),
                &format!("{d} + {e} = ?"),
                f as i32,
                0,
                110,
            ),
            number_ask(
                &format!("{g} - {h} = ?"),
                &format!("{g} - {h} = ?"),
                i as i32,
                0,
                g as i32,
            ),
            number_ask(
                &format!("{j} - {k} = ?"),
                &format!("{j} - {k} = ?"),
                l as i32,
                0,
                j as i32,
            ),
            number_ask(
                &format!("{m} + {n} = ?"),
                &format!("{m} + {n} = ?"),
                o as i32,
                0,
                110,
            ),
        ]
    }
}

fn times_paper(class_no: u8, rng: &mut Rng) -> Vec<Ask> {
    let limit = if class_no == 1 { 20 } else { 100 };
    let tables: &[u16] = if class_no == 1 {
        &[2, 5, 10]
    } else {
        &[2, 3, 4, 5, 6, 7, 8, 9, 10]
    };
    let mut pairs = Vec::new();
    for n in tables {
        for k in 1..=10 {
            if n * k <= limit {
                pairs.push((*n, k));
            }
        }
    }
    rng.shuffle(&mut pairs);
    pairs.truncate(5);
    pairs
        .into_iter()
        .map(|(n, k)| {
            let product = n * k;
            number_ask(
                &format!("{n} × {k} = ?"),
                &format!("{n} × {k} = ?"),
                product as i32,
                0,
                product as i32 + 8,
            )
        })
        .collect()
}

fn share_paper(class_no: u8, rng: &mut Rng) -> Vec<Ask> {
    let cap = if class_no == 1 { 10 } else { 20 };
    let steps: Vec<u16> = if class_no == 1 {
        vec![2, 5]
    } else {
        (2..=10).collect()
    };
    let mut facts = Vec::new();
    for step in steps {
        let mut times = 2u16;
        while step.saturating_mul(times) <= cap {
            facts.push((step * times, step, times));
            times += 1;
        }
    }
    rng.shuffle(&mut facts);
    facts.truncate(5);
    facts
        .into_iter()
        .map(|(total, step, times)| {
            let picture = std::iter::once(total.to_string())
                .chain((0..times).map(|_| step.to_string()))
                .collect::<Vec<_>>()
                .join(" - ");
            number_ask(
                &format!("{picture}। कितनी बार?"),
                &format!("{picture}. How many times?"),
                times as i32,
                1,
                10,
            )
        })
        .collect()
}

fn number_ask(hi: &str, en: &str, correct: i32, min: i32, max: i32) -> Ask {
    let (w1, w2) = two_wrongs(correct, min, max);
    nums(hi, en, &correct.to_string(), &w1, &w2, 0)
}

fn order_ask(rng: &mut Rng, max: u16) -> Ask {
    let mut values = Vec::new();
    let mut guard = 0;
    while values.len() < 3 && guard < 40 {
        guard += 1;
        let n = rng.gen_range(0, max);
        if !values.contains(&n) {
            values.push(n);
        }
    }
    let ascending = rng.next_u32() % 2 == 0;
    let (correct, wrong1, wrong2) = order_choices(&values, ascending);
    let shown = join(&values);
    if ascending {
        nums(
            &format!("{shown} का बढ़ता क्रम क्या है?"),
            &format!("Smallest to largest for {shown}?"),
            &correct,
            &wrong1,
            &wrong2,
            0,
        )
    } else {
        nums(
            &format!("{shown} का घटता क्रम क्या है?"),
            &format!("Largest to smallest for {shown}?"),
            &correct,
            &wrong1,
            &wrong2,
            0,
        )
    }
}

fn add_pair(rng: &mut Rng, carry: bool) -> (u16, u16, u16) {
    for _ in 0..40 {
        let a = rng.gen_range(12, 70);
        let b = rng.gen_range(3, 29);
        if a + b >= 100 {
            continue;
        }
        if ((a % 10) + (b % 10) >= 10) == carry {
            return (a, b, a + b);
        }
    }
    if carry {
        (28, 15, 43)
    } else {
        (23, 14, 37)
    }
}

fn sub_pair(rng: &mut Rng, borrow: bool) -> (u16, u16, u16) {
    for _ in 0..40 {
        let a = rng.gen_range(30, 90);
        let b = rng.gen_range(11, a - 1);
        if ((a % 10) < (b % 10)) == borrow {
            return (a, b, a - b);
        }
    }
    if borrow {
        (40, 16, 24)
    } else {
        (68, 15, 53)
    }
}

fn write_target(class_no: u8, asks: &[Ask], rng: &mut Rng) -> String {
    let answers: Vec<String> = asks
        .iter()
        .map(|ask| ask.choices_hi[ask.answer].clone())
        .collect();
    let (lo, hi) = if class_no == 1 { (0, 9) } else { (12, 98) };
    for _ in 0..40 {
        let text = rng.gen_range(lo, hi).to_string();
        if !answers.iter().any(|answer| answer == &text) {
            return text;
        }
    }
    for fallback in if class_no == 1 {
        ["8", "3", "0"]
    } else {
        ["16", "27", "64"]
    } {
        if !answers.iter().any(|answer| answer == fallback) {
            return fallback.to_string();
        }
    }
    "8".to_string()
}

fn order_choices(nums: &[u16], ascending: bool) -> (String, String, String) {
    let mut sorted = nums.to_vec();
    if ascending {
        sorted.sort_unstable();
    } else {
        sorted.sort_unstable_by(|a, b| b.cmp(a));
    }
    let correct = join(&sorted);
    let mut wrong_a = nums.to_vec();
    wrong_a.reverse();
    if wrong_a == sorted {
        wrong_a.swap(0, 1);
    }
    let mut wrong_b = sorted.clone();
    let last = wrong_b.len() - 1;
    wrong_b.swap(0, last);
    if join(&wrong_b) == correct || join(&wrong_b) == join(&wrong_a) {
        wrong_b.swap(0, 1.min(last));
    }
    (correct, join(&wrong_a), join(&wrong_b))
}

fn pair_from(nums: &[u16], correct: u16) -> (String, String) {
    let mut rest: Vec<u16> = nums.iter().copied().filter(|n| *n != correct).collect();
    rest.sort_unstable();
    rest.dedup();
    (rest[0].to_string(), rest[1].to_string())
}

fn join(nums: &[u16]) -> String {
    nums.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn dots(n: u8) -> String {
    (0..n).map(|_| "●").collect::<Vec<_>>().join(" ")
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        let mut x = seed.wrapping_add(0x9E3779B97F4A7C15) | 1;
        x ^= x >> 30;
        x = x.wrapping_mul(0xBF58476D1CE4E5B9);
        x ^= x >> 27;
        x = x.wrapping_mul(0x94D049BB133111EB);
        x ^= x >> 31;
        Self(x | 1)
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545F4914F6CDD1D) >> 32) as u32
    }

    fn gen_range(&mut self, lo: u16, hi: u16) -> u16 {
        let span = (hi as u32 - lo as u32) + 1;
        lo + (self.next_u32() % span) as u16
    }

    fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = (self.next_u32() as usize) % (i + 1);
            items.swap(i, j);
        }
    }
}

fn shuffle_ask(ask: Ask, rng: &mut Rng) -> Ask {
    let correct = ask.choices_hi[ask.answer].clone();
    let mut pairs: Vec<(String, String)> = ask.choices_hi.into_iter().zip(ask.choices_en).collect();
    rng.shuffle(&mut pairs);
    let answer = pairs
        .iter()
        .position(|(text, _)| text == &correct)
        .unwrap_or(0);
    let (choices_hi, choices_en): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    Ask {
        choices_hi,
        choices_en,
        answer,
        ..ask
    }
}

fn shuffle_card(card: Card, rng: &mut Rng) -> Card {
    Card {
        questions: card
            .questions
            .into_iter()
            .map(|ask| shuffle_ask(ask, rng))
            .collect(),
        ..card
    }
}

fn two_wrongs(correct: i32, min: i32, max: i32) -> (String, String) {
    let mut out = Vec::new();
    for delta in 1..=8 {
        for value in [correct + delta, correct - delta] {
            if (min..=max).contains(&value) && value != correct && !out.contains(&value) {
                out.push(value);
            }
            if out.len() == 2 {
                return (out[0].to_string(), out[1].to_string());
            }
        }
    }
    let fallback_a = if correct == min { min + 1 } else { min };
    let fallback_b = if correct == max { max - 1 } else { max };
    (fallback_a.to_string(), fallback_b.to_string())
}

fn nums(hi: &str, en: &str, correct: &str, w1: &str, w2: &str, slot: usize) -> Ask {
    let mut choices = [w1, w2, correct];
    let spins = (2 + 3 - (slot % 3)) % 3;
    for _ in 0..spins {
        choices.rotate_left(1);
    }
    q(hi, en, choices, slot % 3)
}

fn words(hi: &str, en: &str, hi_c: [&str; 3], en_c: [&str; 3], answer: usize) -> Ask {
    Ask {
        prompt_hi: hi.to_string(),
        prompt_en: en.to_string(),
        choices_hi: hi_c.map(|choice| choice.to_string()).to_vec(),
        choices_en: en_c.map(|choice| choice.to_string()).to_vec(),
        answer,
    }
}

fn q(hi: &str, en: &str, choices: [&str; 3], answer: usize) -> Ask {
    Ask {
        prompt_hi: hi.to_string(),
        prompt_en: en.to_string(),
        choices_hi: choices.map(|choice| choice.to_string()).to_vec(),
        choices_en: choices.map(|choice| choice.to_string()).to_vec(),
        answer,
    }
}

fn num_card(
    title: &str,
    speak_hi: &str,
    speak_en: &str,
    picture: &str,
    trace: &str,
    handwriting: bool,
    questions: Vec<Ask>,
) -> Card {
    fact(
        title,
        title,
        speak_hi,
        speak_en,
        picture,
        trace,
        handwriting,
        questions,
    )
}

fn fact(
    title_hi: &str,
    title_en: &str,
    speak_hi: &str,
    speak_en: &str,
    picture: &str,
    trace: &str,
    handwriting: bool,
    questions: Vec<Ask>,
) -> Card {
    Card {
        title_hi: title_hi.to_string(),
        title_en: title_en.to_string(),
        speak_hi: speak_hi.to_string(),
        speak_en: speak_en.to_string(),
        picture: picture.to_string(),
        trace: Some(trace.to_string()),
        handwriting,
        questions,
    }
}

fn to_item(id: &str, card: Card, lang: Lang) -> Item {
    let en = lang == Lang::En;
    Item {
        id: id.to_string(),
        title: if en { card.title_en } else { card.title_hi },
        speak: if en { card.speak_en } else { card.speak_hi },
        formation: card.handwriting,
        kind: Kind::Number {
            picture: card.picture,
            trace: card.trace,
            handwriting: card.handwriting,
            questions: card
                .questions
                .into_iter()
                .map(|ask| to_quiz(ask, lang))
                .collect(),
        },
    }
}

fn to_quiz(ask: Ask, lang: Lang) -> QuizQ {
    if lang == Lang::En {
        QuizQ {
            prompt: ask.prompt_en,
            choices: ask.choices_en,
            answer: ask.answer,
        }
    } else {
        QuizQ {
            prompt: ask.prompt_hi,
            choices: ask.choices_hi,
            answer: ask.answer,
        }
    }
}

fn hi_word(n: u16) -> &'static str {
    match n {
        0 => "शून्य",
        1 => "एक",
        2 => "दो",
        3 => "तीन",
        4 => "चार",
        5 => "पाँच",
        6 => "छह",
        7 => "सात",
        8 => "आठ",
        9 => "नौ",
        10 => "दस",
        11 => "ग्यारह",
        12 => "बारह",
        14 => "चौदह",
        15 => "पंद्रह",
        16 => "सोलह",
        17 => "सत्रह",
        18 => "अठारह",
        19 => "उन्नीस",
        20 => "बीस",
        21 => "इक्कीस",
        23 => "तेईस",
        24 => "चौबीस",
        25 => "पच्चीस",
        27 => "सत्ताईस",
        28 => "अट्ठाईस",
        29 => "उनतीस",
        30 => "तीस",
        31 => "इकतीस",
        32 => "बत्तीस",
        35 => "पैंतीस",
        36 => "छत्तीस",
        37 => "सैंतीस",
        38 => "अड़तीस",
        40 => "चालीस",
        41 => "इकतालीस",
        42 => "बयालीस",
        43 => "तैंतालीस",
        44 => "चौवालीस",
        45 => "पैंतालीस",
        46 => "छियालीस",
        47 => "सैंतालीस",
        48 => "अड़तालीस",
        49 => "उनचास",
        50 => "पचास",
        51 => "इक्यावन",
        52 => "बावन",
        53 => "तिरपन",
        56 => "छप्पन",
        59 => "उनसठ",
        60 => "साठ",
        65 => "पैंसठ",
        68 => "अड़सठ",
        70 => "सत्तर",
        75 => "पचहत्तर",
        80 => "अस्सी",
        81 => "इक्यासी",
        83 => "तिरासी",
        90 => "नब्बे",
        97 => "सत्तानवे",
        98 => "अठानवे",
        99 => "निन्यानवे",
        100 => "सौ",
        102 => "एक सौ दो",
        120 => "एक सौ बीस",
        200 => "दो सौ",
        210 => "दो सौ दस",
        249 => "दो सौ उनचास",
        250 => "दो सौ पचास",
        251 => "दो सौ इक्यावन",
        305 => "तीन सौ पाँच",
        _ => "MISSING",
    }
}

fn en_word(n: u16) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        21 => "twenty-one",
        23 => "twenty-three",
        24 => "twenty-four",
        25 => "twenty-five",
        27 => "twenty-seven",
        28 => "twenty-eight",
        29 => "twenty-nine",
        30 => "thirty",
        31 => "thirty-one",
        32 => "thirty-two",
        35 => "thirty-five",
        36 => "thirty-six",
        37 => "thirty-seven",
        38 => "thirty-eight",
        40 => "forty",
        41 => "forty-one",
        42 => "forty-two",
        43 => "forty-three",
        44 => "forty-four",
        45 => "forty-five",
        46 => "forty-six",
        47 => "forty-seven",
        48 => "forty-eight",
        49 => "forty-nine",
        50 => "fifty",
        51 => "fifty-one",
        52 => "fifty-two",
        53 => "fifty-three",
        56 => "fifty-six",
        59 => "fifty-nine",
        60 => "sixty",
        65 => "sixty-five",
        68 => "sixty-eight",
        70 => "seventy",
        75 => "seventy-five",
        80 => "eighty",
        81 => "eighty-one",
        83 => "eighty-three",
        90 => "ninety",
        97 => "ninety-seven",
        98 => "ninety-eight",
        99 => "ninety-nine",
        100 => "one hundred",
        102 => "one hundred two",
        120 => "one hundred twenty",
        200 => "two hundred",
        210 => "two hundred ten",
        249 => "two hundred forty-nine",
        250 => "two hundred fifty",
        251 => "two hundred fifty-one",
        305 => "three hundred five",
        _ => "MISSING",
    }
}

#[cfg(test)]
mod tests {
    use super::super::Kind;
    use super::*;

    #[test]
    fn cards_and_papers_are_complete() {
        for class_no in [1, 2] {
            for code in ["count", "add", "times", "share"] {
                for seed in [0, 1, 2] {
                    let cards = flatten(class_no, code, seed);
                    assert!(!cards.is_empty(), "{class_no} {code}");
                    for (index, card) in cards.iter().enumerate() {
                        assert_eq!(
                            card.questions.len(),
                            3,
                            "{class_no} {code} {index} {}",
                            card.title_en
                        );
                        assert!(card.trace.is_some());
                        assert!(
                            !card.speak_hi.contains("MISSING"),
                            "{} {seed}",
                            card.title_en
                        );
                        assert!(
                            !card.speak_en.contains("MISSING"),
                            "{} {seed}",
                            card.title_en
                        );
                        let mut answers = Vec::new();
                        for ask in &card.questions {
                            assert_eq!(ask.choices_hi.len(), 3);
                            assert!(ask.answer < 3);
                            assert_ne!(ask.choices_hi[0], ask.choices_hi[1]);
                            assert_ne!(ask.choices_hi[0], ask.choices_hi[2]);
                            assert_ne!(ask.choices_hi[1], ask.choices_hi[2]);
                            answers.push(ask.choices_hi[ask.answer].clone());
                        }
                        answers.sort();
                        answers.dedup();
                        assert_eq!(
                            answers.len(),
                            3,
                            "{class_no} {code} {index} {} seed {seed}",
                            card.title_en
                        );
                    }
                }
            }
        }
        let add = lookup("math:1:add:0", Lang::En, 0).unwrap();
        assert_eq!(add.title, "3 + 2");
        let Kind::Number {
            trace, questions, ..
        } = add.kind
        else {
            panic!("add")
        };
        assert_eq!(trace.as_deref(), Some("5"));
        assert!(questions.iter().any(|ask| ask.choices[ask.answer] == "5"));

        let skip = lookup("math:1:count:32", Lang::Hi, 0).unwrap();
        assert_eq!(skip.title, "2, 4, 6");
        let Kind::Number {
            trace, questions, ..
        } = skip.kind
        else {
            panic!("skip")
        };
        assert_eq!(trace.as_deref(), Some("8"));
        assert!(questions.iter().any(|ask| ask.choices[ask.answer] == "8"));

        let share = lookup("math:2:share:0", Lang::En, 0).unwrap();
        let Kind::Number {
            trace, questions, ..
        } = share.kind
        else {
            panic!("share")
        };
        assert_eq!(trace.as_deref(), Some("4"));
        assert!(questions.iter().any(|ask| ask.choices[ask.answer] == "4"));

        let order = lookup("math:1:count:43", Lang::En, 0).unwrap();
        assert!(order.title.contains("4, 9, 2"));
        let Kind::Number { trace, .. } = order.kind else {
            panic!("order")
        };
        assert_eq!(trace.as_deref(), Some("2"));

        let table = lookup("math:2:times:0", Lang::Hi, 0).unwrap();
        assert!(table.speak.contains("दो बार दो"), "{}", table.speak);
        assert!(!table.speak.contains("चार बार"));
        let table_en = lookup("math:2:times:0", Lang::En, 0).unwrap();
        assert!(
            table_en.speak.contains("two times two"),
            "{}",
            table_en.speak
        );
        let again = lookup("math:2:times:0", Lang::En, 5).unwrap();
        let Kind::Number {
            questions: first_qs,
            ..
        } = table_en.kind
        else {
            panic!("table")
        };
        let Kind::Number {
            questions: next_qs, ..
        } = again.kind
        else {
            panic!("table again")
        };
        let first_prompts: Vec<_> = first_qs.iter().map(|ask| ask.prompt.clone()).collect();
        let next_prompts: Vec<_> = next_qs.iter().map(|ask| ask.prompt.clone()).collect();
        assert_ne!(first_prompts, next_prompts);

        assert_eq!(next_id("math:1:count:0").as_deref(), Some("math:1:count:1"));
        assert!(next_id("math:1:count:999").is_none());

        for class_no in [1, 2] {
            for goal in [Goal::Count, Goal::AddSub, Goal::Tables, Goal::Divide] {
                let mut seen = Vec::new();
                for seed in [1u64, 2, 3] {
                    for lang in [Lang::Hi, Lang::En] {
                        let paper = paper(class_no, goal, lang, seed);
                        assert_eq!(paper.questions.len(), 5);
                        for ask in &paper.questions {
                            assert!(ask.answer < ask.choices.len());
                            assert_ne!(ask.choices[0], ask.choices[1]);
                            assert_ne!(ask.choices[0], ask.choices[2]);
                            assert_ne!(ask.choices[1], ask.choices[2]);
                            if let Some(written) = &paper.write {
                                assert_ne!(&ask.choices[ask.answer], written);
                            }
                        }
                        if lang == Lang::En {
                            seen.push(
                                paper
                                    .questions
                                    .into_iter()
                                    .map(|ask| ask.prompt)
                                    .collect::<Vec<_>>()
                                    .join("|"),
                            );
                        }
                    }
                }
                assert!(
                    seen.iter().any(|prompts| prompts != &seen[0]),
                    "{class_no} {goal:?}"
                );
            }
        }
    }
}
