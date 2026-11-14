#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CivilDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

pub const WEEKDAYS: [&str; 7] = [
    "रविवार",
    "सोमवार",
    "मंगलवार",
    "बुधवार",
    "गुरुवार",
    "शुक्रवार",
    "शनिवार",
];

pub const WEEKDAYS_SHORT: [&str; 7] = ["रवि", "सोम", "मंगल", "बुध", "गुरु", "शुक्र", "शनि"];

pub const MONTHS: [&str; 12] = [
    "जनवरी",
    "फरवरी",
    "मार्च",
    "अप्रैल",
    "मई",
    "जून",
    "जुलाई",
    "अगस्त",
    "सितंबर",
    "अक्टूबर",
    "नवंबर",
    "दिसंबर",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DayCard {
    pub label: &'static str,
    pub weekday: &'static str,
    pub day: u8,
    pub speak: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalQuestion {
    pub prompt: &'static str,
    pub choices: Vec<String>,
    pub answer: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarLesson {
    pub cards: Vec<DayCard>,
    pub month: &'static str,
    pub month_speak: String,
    pub questions: Vec<CalQuestion>,
}

pub struct PlanItem {
    pub id: &'static str,
    pub title: &'static str,
    pub steps: &'static [&'static str],
}

pub struct PlanGroup {
    pub title: &'static str,
    pub items: &'static [PlanItem],
}

pub fn today() -> CivilDate {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        CivilDate {
            year: date.get_full_year() as i32,
            month: date.get_month() as u8 + 1,
            day: date.get_date() as u8,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use chrono::Datelike;
        let local = chrono::Local::now();
        CivilDate {
            year: local.year(),
            month: local.month() as u8,
            day: local.day() as u8,
        }
    }
}

impl CivilDate {
    pub fn add_days(self, delta: i32) -> Self {
        civil_from_days(days_from_civil(self.year, self.month, self.day) + delta)
    }

    pub fn weekday(self) -> u8 {
        let mut year = self.year;
        let month = self.month as usize;
        if month < 3 {
            year -= 1;
        }
        let offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let sum = year + year / 4 - year / 100 + year / 400 + offsets[month - 1] + self.day as i32;
        sum.rem_euclid(7) as u8
    }

    pub fn key(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn line(self) -> String {
        format!(
            "{} {}, {}",
            self.day,
            month_name(self.month),
            WEEKDAYS[self.weekday() as usize]
        )
    }
}

pub fn weekday_short(weekday: u8) -> &'static str {
    WEEKDAYS_SHORT[weekday as usize % 7]
}

pub fn month_name(month: u8) -> &'static str {
    MONTHS[(month as usize).saturating_sub(1) % 12]
}

pub fn calendar_lesson(today: CivilDate) -> CalendarLesson {
    let yesterday = today.add_days(-1);
    let tomorrow = today.add_days(1);
    let month = month_name(today.month);
    CalendarLesson {
        cards: vec![
            day_card("बीता कल", "था", yesterday),
            day_card("आज", "है", today),
            day_card("आने वाला कल", "होगा", tomorrow),
        ],
        month,
        month_speak: format!("आज का महीना {month} है।"),
        questions: vec![
            weekday_question("बीते कल कौन-सा दिन था?", yesterday, today.day),
            weekday_question("आज कौन-सा दिन है?", today, today.day.wrapping_add(1)),
            weekday_question(
                "आने वाले कल कौन-सा दिन होगा?",
                tomorrow,
                today.day.wrapping_add(2),
            ),
            named_question(
                "आज कौन-सा महीना है?",
                month,
                month_name(shift_month(today.month, -1)),
                month_name(shift_month(today.month, 1)),
                today.day.wrapping_add(3),
            ),
            date_question("बीते कल तारीख क्या थी?", yesterday, today.day.wrapping_add(4)),
            date_question("आज तारीख क्या है?", today, today.day.wrapping_add(5)),
            date_question(
                "आने वाले कल तारीख क्या होगी?",
                tomorrow,
                today.day.wrapping_add(6),
            ),
        ],
    }
}

pub fn plan_groups() -> &'static [PlanGroup] {
    &PLAN
}

fn day_card(label: &'static str, verb: &str, date: CivilDate) -> DayCard {
    let weekday = WEEKDAYS[date.weekday() as usize];
    DayCard {
        label,
        weekday,
        day: date.day,
        speak: format!("{label} {weekday} {verb}। तारीख {}।", date.day),
    }
}

fn weekday_question(prompt: &'static str, date: CivilDate, salt: u8) -> CalQuestion {
    let weekday = date.weekday();
    named_question(
        prompt,
        WEEKDAYS[weekday as usize],
        WEEKDAYS[shift_weekday(weekday, -1) as usize],
        WEEKDAYS[shift_weekday(weekday, 1) as usize],
        salt,
    )
}

fn date_question(prompt: &'static str, date: CivilDate, salt: u8) -> CalQuestion {
    named_question(
        prompt,
        &date.day.to_string(),
        &date.add_days(-1).day.to_string(),
        &date.add_days(1).day.to_string(),
        salt,
    )
}

fn named_question(
    prompt: &'static str,
    correct: &str,
    before: &str,
    after: &str,
    salt: u8,
) -> CalQuestion {
    let (choices, answer) = place(correct, before, after, salt);
    CalQuestion {
        prompt,
        choices,
        answer,
    }
}

fn place(correct: &str, before: &str, after: &str, salt: u8) -> (Vec<String>, usize) {
    let slot = (salt % 3) as usize;
    let mut choices = vec![String::new(), String::new(), String::new()];
    choices[slot] = correct.to_string();
    let mut extras = [before, after].into_iter();
    for (index, choice) in choices.iter_mut().enumerate() {
        if index == slot {
            continue;
        }
        *choice = extras.next().unwrap_or(correct).to_string();
    }
    (choices, slot)
}

fn shift_month(month: u8, delta: i32) -> u8 {
    (month as i32 - 1 + delta).rem_euclid(12) as u8 + 1
}

fn shift_weekday(weekday: u8, delta: i32) -> u8 {
    (weekday as i32 + delta).rem_euclid(7) as u8
}

fn days_from_civil(mut year: i32, month: u8, day: u8) -> i32 {
    let month = month as i32;
    if month <= 2 {
        year -= 1;
    }
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = (year - era * 400) as u32;
    let day_of_year =
        (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day as i32 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year as u32;
    era * 146097 + day_of_era as i32 - 719468
}

fn civil_from_days(mut z: i32) -> CivilDate {
    z += 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let day_of_era = (z - era * 146097) as u32;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146096) / 365;
    let year = year_of_era as i32 + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_part + 2) / 5 + 1;
    let month = if month_part < 10 {
        month_part + 3
    } else {
        month_part - 9
    };
    CivilDate {
        year: year + i32::from(month <= 2),
        month: month as u8,
        day: day as u8,
    }
}

const PLAN: [PlanGroup; 2] = [
    PlanGroup {
        title: "नवंबर की बोलने की योजना",
        items: &[
            PlanItem {
                id: "nov:cheek",
                title: "गाल की मालिश",
                steps: &[
                    "OPM: गाल की मालिश, अंदर और बाहर।",
                    "Inner और outer, दोनों गाल।",
                ],
            },
            PlanItem {
                id: "nov:blow",
                title: "फूँकना",
                steps: &["Spirometer से फूँकने का अभ्यास।"],
            },
            PlanItem {
                id: "nov:r",
                title: "आवाज़ R",
                steps: &[
                    "शुरू, बीच, और अंत में R।",
                    "थेरेपी वाले शब्द बोलें। शब्द इस पन्ने पर नहीं लिखे हैं।",
                ],
            },
            PlanItem {
                id: "nov:l",
                title: "आवाज़ L",
                steps: &[
                    "शुरू, बीच, और अंत में L।",
                    "थेरेपी वाले शब्द बोलें। शब्द इस पन्ने पर नहीं लिखे हैं।",
                ],
            },
            PlanItem {
                id: "nov:s",
                title: "आवाज़ S",
                steps: &[
                    "शुरू, बीच, और अंत में S।",
                    "थेरेपी वाले शब्द बोलें। शब्द इस पन्ने पर नहीं लिखे हैं।",
                ],
            },
            PlanItem {
                id: "nov:sentence",
                title: "वाक्य बनाना",
                steps: &[
                    "क्रिया: चलना, खाना, खेलना।",
                    "इनका use करके पूरा sentence बनाने की practice करवाएँ।",
                ],
            },
            PlanItem {
                id: "nov:social",
                title: "सामाजिक सवाल",
                steps: &[
                    "नवंबर की योजना में यह शीर्षक है। सवाल लिखे नहीं हैं।",
                    "जो सवाल थेरेपी में आए हों, वही पूछें।",
                    "Hi, Bye और आप कैसे हो अगले भाग में लिखे हैं।",
                ],
            },
            PlanItem {
                id: "nov:picture",
                title: "चित्र बताओ",
                steps: &[
                    "किसी भी book या picture को दिखाएँ।",
                    "उसमें क्या हो रहा है, अपने शब्दों में describe करने को कहें।",
                ],
            },
            PlanItem {
                id: "nov:where",
                title: "कहाँ वाले प्रश्न",
                steps: &[
                    "खाना कहाँ बनाते हैं? — किचन में बनाते हैं।",
                    "Bathing कहाँ करते हैं? — bathroom में करते हैं।",
                    "पैसे कहाँ रखते हैं? — bank में रखते हैं।",
                    "कूड़ा कहाँ डालते हैं? — dustbin में डालते हैं।",
                ],
            },
            PlanItem {
                id: "nov:object",
                title: "वस्तु बताओ",
                steps: &[
                    "What is it?",
                    "What does it look like?",
                    "Where is it used?",
                    "बच्चे को encourage करें कि वह उस object के बारे में 2–3 simple sentences खुद बोले।",
                ],
            },
        ],
    },
    PlanGroup {
        title: "ABA घर की चिकित्सा",
        items: &[
            PlanItem {
                id: "aba:eye",
                title: "आँख से आँख",
                steps: &[
                    "पसंदीदा चीज़ आँख के पास रखें, या आँख की ऊँचाई पर बैठें।",
                    "शिवेंद्र का नाम लें, या बोलें: Look at me!",
                    "आँख मिले तो चुपचाप 10 सेकंड तक गिनें। बात जारी रखें।",
                    "तुरंत तारीफ: Great eye contact! या चीज़ दे दें।",
                ],
            },
            PlanItem {
                id: "aba:wait",
                title: "इंतज़ार",
                steps: &[
                    "पसंदीदा चीज़ दिखाकर बोलें: Wait।",
                    "Visual timer, या 1 मिनट की गिनती।",
                    "जल्दी हाथ बढ़ाए तो हाथ रोकें।",
                    "1 मिनट पूरा होते ही चीज़ दें: Awesome waiting!",
                ],
            },
            PlanItem {
                id: "aba:fruit",
                title: "फल, तीन शब्द",
                steps: &[
                    "फल का card दिखाएँ।",
                    "पूछें: ये क्या है?",
                    "3–5 सेकंड रुकें। ज़रूरत हो तो prompt दें।",
                    "सही कोशिश पर reinforce करें। जवाब जैसे: ये banana है।",
                ],
            },
            PlanItem {
                id: "aba:animal",
                title: "जानवर, तीन शब्द",
                steps: &[
                    "जानवर का card दिखाएँ।",
                    "पूछें: ये क्या है?",
                    "खुद बोले, या prompt के साथ।",
                    "तुरंत तारीफ। जवाब जैसे: ये dog है।",
                ],
            },
            PlanItem {
                id: "aba:action",
                title: "क्रिया, तीन शब्द",
                steps: &[
                    "Action का card दिखाएँ।",
                    "पूछें: boy/girl क्या कर रहा/रही है?",
                    "जवाब जैसे: boy/girl jumping कर रहा/रही है।",
                    "तारीफ करके अगली क्रिया पर जाएँ।",
                ],
            },
            PlanItem {
                id: "aba:gender",
                title: "Boy और Girl",
                steps: &[
                    "चित्र दिखाएँ, या घर के किसी व्यक्ति की ओर इशारा करें।",
                    "पूछें: Is this a boy or a girl?",
                    "जवाब: Boy या Girl। उलझन हो तो prompt दें।",
                    "सही पहचान पर तारीफ।",
                ],
            },
            PlanItem {
                id: "aba:sort",
                title: "Transport और Bird",
                steps: &[
                    "दो जगह लिखें: Transports और Birds।",
                    "Card दें और बुलवाएँ: ये car है, ये transport है। या: ये pigeon है, ये bird है।",
                    "पूछें: Where does this go? नाम बोलकर सही ढेर में रखें।",
                ],
            },
            PlanItem {
                id: "aba:mand",
                title: "माँग",
                steps: &[
                    "पसंदीदा चीज़ दिखाएँ, हाथ से दूर रखें। 3–5 सेकंड रुकें।",
                    "न माँगे तो prompt: Ball, आधा शब्द, या इशारा।",
                    "गायब चीज़: drawing book दें, pencil दूर रखें।",
                    "पूछ सकते हैं: What do you need? जवाब: Pencil, या I need a pencil।",
                    "तुरंत चीज़ दें। Prompt धीरे-धीरे हटाएँ।",
                ],
            },
            PlanItem {
                id: "aba:pattern",
                title: "पैटर्न",
                steps: &[
                    "क्रम रखें, जैसे Red-Blue-Red-Blue।",
                    "अगला टुकड़ा दें: Complete the pattern।",
                    "गलत जगह हो तो मदद करें। पूरा होने पर तारीफ।",
                ],
            },
            PlanItem {
                id: "aba:puzzle",
                title: "मौसम का puzzle",
                steps: &[
                    "Summer, Rainy, या Winter का puzzle।",
                    "टुकड़े एक-एक करके दें: Put it in the right place।",
                    "इशारे या हाथ से टुकड़ा सही मोड़ पर लगाएँ।",
                ],
            },
            PlanItem {
                id: "aba:job",
                title: "पेशे",
                steps: &[
                    "Professional का card या figurine दिखाएँ।",
                    "पूछें: ये कौन है?",
                    "जवाब जैसे: ये doctor है। तुरंत reinforce करें।",
                ],
            },
            PlanItem {
                id: "aba:name",
                title: "नाम और Yes",
                steps: &[
                    "थोड़ी दूर से पुकारें: Shivendra!",
                    "मुड़े, आँख मिलाए, और बोले: Yes।",
                    "देखे पर चुप रहे तो बोलें: Say Yes।",
                    "पूरा जवाब मिले तो जोर से तारीफ।",
                ],
            },
            PlanItem {
                id: "aba:give",
                title: "जाओ और दे दो",
                steps: &[
                    "चीज़ दें, जैसे ball।",
                    "बोलें: Go and give the ball to Papa।",
                    "चलकर देने में इशारे से मदद करें।",
                    "चीज़ पहुँच जाए तो reinforce करें।",
                ],
            },
            PlanItem {
                id: "aba:hi",
                title: "Hi, Bye, आप कैसे हो",
                steps: &[
                    "कमरे में आते या जाते समय Hi या Bye, हाथ हिलाकर।",
                    "मॉडल करें: Ask, आप कैसे हो?",
                    "दोहराए या खुद बोले। गर्मजोशी से जवाब दें।",
                ],
            },
            PlanItem {
                id: "aba:what",
                title: "What के सवाल",
                steps: &[
                    "Card दिखाएँ। पूछें: What is this? नाम बोले।",
                    "Card छिपाएँ। पूछें: What does a dog do? या What do we eat with?",
                    "उदाहरण: Cup, फिर Drink milk। सही जवाब पर तारीफ।",
                ],
            },
            PlanItem {
                id: "aba:yesno",
                title: "Yes और No",
                steps: &[
                    "जान-पहचान की चीज़ रखें: ball, car, pencil, book, spoon, cup।",
                    "पहले Yes: Is this a ball? ज़रूरत हो तो Yes बोलकर दिखाएँ।",
                    "फिर No: वही ball दिखाकर Is this a car? जवाब No।",
                    "सही और गलत लेबल बदल-बदल कर पूछें। Prompt हटाते जाएँ।",
                ],
            },
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_thursday_and_neighbours() {
        let today = CivilDate {
            year: 2026,
            month: 11,
            day: 12,
        };
        assert_eq!(today.weekday(), 4);
        assert_eq!(today.add_days(0), today);
        assert_eq!(
            today.add_days(-1),
            CivilDate {
                year: 2026,
                month: 11,
                day: 11
            }
        );
        assert_eq!(
            today.add_days(1),
            CivilDate {
                year: 2026,
                month: 11,
                day: 13
            }
        );
        assert_eq!(WEEKDAYS[today.weekday() as usize], "गुरुवार");
    }

    #[test]
    fn month_edges() {
        assert_eq!(
            CivilDate {
                year: 2026,
                month: 3,
                day: 1
            }
            .add_days(-1),
            CivilDate {
                year: 2026,
                month: 2,
                day: 28
            }
        );
        assert_eq!(
            CivilDate {
                year: 2024,
                month: 3,
                day: 1
            }
            .add_days(-1),
            CivilDate {
                year: 2024,
                month: 2,
                day: 29
            }
        );
        assert_eq!(
            CivilDate {
                year: 2026,
                month: 1,
                day: 1
            }
            .add_days(-1),
            CivilDate {
                year: 2025,
                month: 12,
                day: 31
            }
        );
        assert_eq!(
            CivilDate {
                year: 2026,
                month: 12,
                day: 31
            }
            .add_days(1),
            CivilDate {
                year: 2027,
                month: 1,
                day: 1
            }
        );
    }

    #[test]
    fn unix_epoch_is_thursday() {
        let epoch = CivilDate {
            year: 1970,
            month: 1,
            day: 1,
        };
        assert_eq!(epoch.weekday(), 4);
        assert_eq!(days_from_civil(1970, 1, 1), 0);
    }

    #[test]
    fn questions_follow_the_real_day() {
        let today = CivilDate {
            year: 2026,
            month: 11,
            day: 12,
        };
        let lesson = calendar_lesson(today);
        let expected = ["बुधवार", "गुरुवार", "शुक्रवार", "नवंबर", "11", "12", "13"];
        assert_eq!(lesson.questions.len(), 7);
        assert_eq!(lesson.cards.len(), 3);
        assert_eq!(lesson.cards[0].day, 11);
        assert_eq!(lesson.cards[1].weekday, "गुरुवार");
        assert_eq!(lesson.cards[2].day, 13);
        assert_eq!(lesson.month, "नवंबर");
        for (question, answer) in lesson.questions.iter().zip(expected) {
            assert_eq!(question.choices.len(), 3, "{}", question.prompt);
            assert_eq!(
                question.choices[question.answer], answer,
                "{}",
                question.prompt
            );
            let mut unique = question.choices.clone();
            unique.sort();
            unique.dedup();
            assert_eq!(unique.len(), 3, "{}", question.prompt);
        }
    }

    #[test]
    fn answer_slot_moves_with_the_date() {
        let early = calendar_lesson(CivilDate {
            year: 2026,
            month: 11,
            day: 12,
        });
        let later = calendar_lesson(CivilDate {
            year: 2026,
            month: 11,
            day: 13,
        });
        assert_ne!(early.questions[1].answer, later.questions[1].answer);
    }
}
