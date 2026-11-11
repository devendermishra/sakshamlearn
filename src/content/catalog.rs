use super::{barakhadi_items, pattern_items, swar_items, vyanjan_items, Item, Kind, QuizQ};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Goal {
    Listen,
    Letters,
    Fluency,
    Meaning,
    Writing,
}

pub const GOALS: [Goal; 5] = [
    Goal::Listen,
    Goal::Letters,
    Goal::Fluency,
    Goal::Meaning,
    Goal::Writing,
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Subject {
    pub index: u8,
    pub title: &'static str,
    pub open: bool,
}

pub struct Section {
    pub title: &'static str,
    pub items: Vec<Item>,
}

pub struct Paper {
    pub write: Option<String>,
    pub questions: Vec<QuizQ>,
}

pub fn subjects() -> [Subject; 3] {
    [
        Subject { index: 0, title: "हिन्दी", open: true },
        Subject { index: 1, title: "English", open: false },
        Subject { index: 2, title: "गणित", open: false },
    ]
}

pub fn goal_key(goal: Goal) -> &'static str {
    match goal {
        Goal::Listen => "listen",
        Goal::Letters => "letters",
        Goal::Fluency => "fluency",
        Goal::Meaning => "meaning",
        Goal::Writing => "writing",
    }
}

pub fn goal_title(goal: Goal) -> &'static str {
    match goal {
        Goal::Listen => "सुनना और बोलना",
        Goal::Letters => "वर्ण और मात्राएँ",
        Goal::Fluency => "प्रवाह से पढ़ना",
        Goal::Meaning => "पढ़कर समझना",
        Goal::Writing => "लिखना",
    }
}

pub fn goal_aim(class_no: u8, goal: Goal) -> &'static str {
    match (class_no, goal) {
        (1, Goal::Listen) => "जानी हुई बात सुनो और आसान सवाल का जवाब छुओ।",
        (_, Goal::Listen) => "नई बात सुनो, क्रम बताओ, और एक कारण चुनो।",
        (1, Goal::Letters) => "अक्षर की ध्वनि पहचानो और किताब, माला जैसे शब्द पढ़ो।",
        (_, Goal::Letters) => "बच्चा, बिल्ली जैसे लंबे और संयुक्त शब्द पढ़ो।",
        (1, Goal::Fluency) => "एक छोटा वाक्य सही गति से पढ़ो और । पर रुको।",
        (_, Goal::Fluency) => "तीन वाक्य रुककर पढ़ो और क्रम बनाए रखो।",
        (1, Goal::Meaning) => "बताओ कौन है और चित्र में क्या हुआ।",
        (_, Goal::Meaning) => "तस्वीरों का क्रम बताओ और अर्थ चुनो।",
        (1, Goal::Writing) => "अक्षर, शब्द और एक वाक्य लिखो। बिंदु पहले गहरे हैं, फिर हल्के।",
        (_, Goal::Writing) => "लंबा शब्द और जुड़े वाक्य लिखो। बिंदु धीरे-धीरे मिटते हैं।",
    }
}

pub fn story(class_no: u8) -> Vec<String> {
    if class_no == 1 {
        vec!["रीना पौधे को पानी देती है।".to_string()]
    } else {
        vec![
            "रीना ने बीज बोया।".to_string(),
            "पौधा निकला।".to_string(),
            "रीना पौधे को पानी देती है।".to_string(),
        ]
    }
}

pub fn bank(class_no: u8, goal: Goal) -> Vec<Section> {
    match goal {
        Goal::Listen => vec![Section { title: "कहानी", items: listen_items(class_no) }],
        Goal::Letters => letter_sections(class_no),
        Goal::Fluency => vec![Section { title: "पढ़ो", items: fluency_items(class_no) }],
        Goal::Meaning => vec![Section { title: "अर्थ", items: meaning_items(class_no) }],
        Goal::Writing => writing_sections(class_no),
    }
}

pub fn paper(class_no: u8, goal: Goal) -> Paper {
    let class_one = class_no == 1;
    let questions = match goal {
        Goal::Listen if class_one => vec![
            ask("कहानी में कौन है?", &["रीना", "दादी", "राम"], 0),
            ask("रीना क्या करती है?", &["पानी देती है", "सोती है", "दौड़ती है"], 0),
            ask("रीना किसे पानी देती है?", &["पौधे को", "घर को", "पुस्तक को"], 0),
            ask("यह किसकी बात है?", &["पौधे की", "चाँद की", "बस की"], 0),
            ask("पानी कौन देती है?", &["रीना", "माँ", "पिता"], 0),
        ],
        Goal::Listen => vec![
            ask("पहले क्या हुआ?", &["रीना ने बीज बोया", "पौधा निकला", "रीना ने पानी दिया"], 0),
            ask("बीज के बाद क्या हुआ?", &["पौधा निकला", "रीना सो गई", "बारिश रुकी"], 0),
            ask("आखिर में रीना क्या करती है?", &["पौधे को पानी देती है", "बीज खाती है", "घर बनाती है"], 0),
            ask("पानी क्यों?", &["पौधा हरा रहे", "पौधा सोए", "खेल के लिए"], 0),
            ask("कितनी बातेँ हुईं?", &["तीन", "एक", "पाँच"], 0),
        ],
        Goal::Letters if class_one => vec![
            ask("किताब कहाँ है?", &["किताब", "माला", "घर"], 0),
            ask("माला कहाँ है?", &["माला", "जल", "फल"], 0),
            ask("आ की मात्रा किसमें है?", &["माला", "दिन", "फल"], 0),
            ask("इ की मात्रा किसमें है?", &["किताब", "आम", "बस"], 0),
            ask("क कहाँ है?", &["क", "म", "प"], 0),
        ],
        Goal::Letters => vec![
            ask("बच्चा कहाँ है?", &["बच्चा", "बचा", "बाल"], 0),
            ask("बिल्ली कहाँ है?", &["बिल्ली", "बिल", "बली"], 0),
            ask("च्च किस शब्द में है?", &["बच्चा", "माला", "घर"], 0),
            ask("ल्ल किस शब्द में है?", &["बिल्ली", "पानी", "फल"], 0),
            ask("क्षमा कहाँ है?", &["क्षमा", "क्ष", "मा"], 0),
        ],
        Goal::Fluency if class_one => vec![
            ask("इस वाक्य में कितने शब्द हैं?", &["छह", "दो", "दस"], 0),
            ask("कहाँ रुकना है?", &["। के बाद", "हर अक्षर पर", "कहीं नहीं"], 0),
            ask("पहला शब्द कौन सा है?", &["रीना", "पानी", "है"], 0),
            ask("आखिरी शब्द कौन सा है?", &["है", "रीना", "पौधे"], 0),
            ask("यह कैसा वाक्य है?", &["छोटा वाक्य", "लंबी कविता", "तीन कहानियाँ"], 0),
        ],
        Goal::Fluency => vec![
            ask("कितने वाक्य हैं?", &["तीन", "एक", "छह"], 0),
            ask("पहले वाक्य में क्या हुआ?", &["बीज बोया", "पानी दिया", "पौधा निकला"], 0),
            ask("बीच वाले वाक्य में क्या निकला?", &["पौधा", "चाँद", "बस"], 0),
            ask("। पर क्या करें?", &["रुकें", "दौड़ें", "लेख मिटाएँ"], 0),
            ask("आखिरी वाक्य में रीना क्या देती है?", &["पानी", "फल", "पुस्तक"], 0),
        ],
        Goal::Meaning if class_one => vec![
            ask("कौन है?", &["रीना", "दादी", "राम"], 0),
            ask("वह क्या कर रही है?", &["पौधे को पानी दे रही है", "सो रही है", "खा रही है"], 0),
            ask("चित्र में मुख्य बात क्या है?", &["पानी देना", "चाँद देखना", "होली खेलना"], 0),
            ask("पौधा किसके पास है?", &["रीना के पास", "दादी के घर में", "तालाब में"], 0),
            ask("यह किस बारे में है?", &["पौधे को पानी", "चाँद", "होली"], 0),
        ],
        Goal::Meaning => vec![
            ask("सही क्रम कौन सा है?", &["बीज, फिर पौधा, फिर पानी", "पानी, फिर बीज, फिर पौधा", "पौधा, फिर पानी, फिर बीज"], 0),
            ask("बीच में क्या हुआ?", &["पौधा निकला", "रीना सो गई", "बीज उड़ गया"], 0),
            ask("आखिर में क्या हुआ?", &["रीना ने पानी दिया", "रीना ने बीज खाया", "पौधा गिर गया"], 0),
            ask("पानी से क्या होता है?", &["पौधा हरा रहता है", "पौधा सो जाता है", "बीज उड़ जाता है"], 0),
            ask("यह क्रम क्या बताता है?", &["बीज से पौधा बनता है", "चाँद निकलता है", "बारिश रुकती है"], 0),
        ],
        Goal::Writing if class_one => vec![
            ask("सही शब्द कौन सा है?", &["पानी", "पनी", "पानीी"], 0),
            ask("कौन सा वाक्य चित्र के बारे में है?", &["रीना पौधे को पानी देती है।", "राम बस में बैठा।", "दादी पुस्तक पढ़ती है।"], 0),
            ask("पौधे को क्या चाहिए?", &["पानी", "जूता", "घर"], 0),
            ask("घर कहाँ है?", &["घर", "घार", "गहर"], 0),
            ask("रीना क्या देती है?", &["पानी", "फल", "रंग"], 0),
        ],
        Goal::Writing => vec![
            ask("सही शब्द कौन सा है?", &["बिल्ली", "बली", "बिली"], 0),
            ask("बच्चा में क्या है?", &["च्च", "ल्ल", "क्ष"], 0),
            ask("सही क्रम कौन सा है?", &["बीज बोया। पौधा निकला। पानी दिया।", "पानी दिया। बीज बोया। पौधा निकला।", "पौधा निकला। पानी दिया। बीज बोया।"], 0),
            ask("पानी क्यों लिखें?", &["पौधा हरा रहे", "पौधा सोए", "बीज उड़े"], 0),
            ask("क्षमा कहाँ है?", &["क्षमा", "क्ष", "मा"], 0),
        ],
    };
    let write = match goal {
        Goal::Writing if class_one => Some("रीना पौधे को पानी देती है।".to_string()),
        Goal::Writing => Some("पौधा निकला।".to_string()),
        _ => None,
    };
    Paper { write, questions }
}

pub fn lookup(id: &str) -> Option<Item> {
    if id == "card:plant1" {
        return Some(plant_card(1));
    }
    if id == "card:plant2" {
        return Some(plant_card(2));
    }
    id.strip_prefix("extra:").map(|text| super::glyph_item(id.to_string(), text, false))
}

pub fn extra_ids() -> Vec<String> {
    vec!["card:plant1".to_string(), "card:plant2".to_string()]
}

fn listen_items(class_no: u8) -> Vec<Item> {
    let mut items = vec![plant_card(class_no)];
    items.extend(reads(if class_no == 1 { &[0, 1] } else { &[4, 6] }));
    items
}

fn fluency_items(class_no: u8) -> Vec<Item> {
    let mut items = vec![plant_card(class_no)];
    items.extend(reads(if class_no == 1 { &[0, 1] } else { &[3, 5] }));
    items
}

fn meaning_items(class_no: u8) -> Vec<Item> {
    let mut items = vec![plant_card(class_no)];
    items.extend(reads(if class_no == 1 { &[0] } else { &[2, 7] }));
    items
}

fn letter_sections(class_no: u8) -> Vec<Section> {
    if class_no == 1 {
        vec![
            Section { title: "स्वर", items: swar_items() },
            Section {
                title: "व्यंजन",
                items: vyanjan_items().into_iter().take(33).collect(),
            },
            Section {
                title: "मात्रा",
                items: [0usize, 20, 24, 26].into_iter().flat_map(barakhadi_items).collect(),
            },
            Section {
                title: "शब्द",
                items: words(&["घर", "जल", "फल", "माला", "आम", "पिता", "किताब", "पानी"]),
            },
        ]
    } else {
        let mut conjunct: Vec<Item> = vyanjan_items().into_iter().skip(33).collect();
        conjunct.extend(words(&["बच्चा", "बिल्ली", "क्षमा"]));
        vec![
            Section { title: "संयुक्त", items: conjunct },
            Section {
                title: "लंबे शब्द",
                items: words(&["तालाब", "चींटी", "मूली", "मेंढक", "बैंगनी"]),
            },
        ]
    }
}

fn writing_sections(class_no: u8) -> Vec<Section> {
    let sentence = if class_no == 1 {
        "रीना पौधे को पानी देती है।"
    } else {
        "पौधा निकला।"
    };
    let words_now = if class_no == 1 {
        words(&["घर", "माला", "किताब", "पानी", sentence])
    } else {
        words(&["बच्चा", "बिल्ली", "चींटी", "मेंढक", "क्षमा", sentence])
    };
    vec![
        Section { title: "रेखा", items: pattern_items() },
        Section { title: "लिखो", items: words_now },
    ]
}

fn words(list: &[&str]) -> Vec<Item> {
    list.iter().copied().map(word_named).collect()
}

fn word_named(text: &str) -> Item {
    if let Some(index) = super::hindi::WORDS.iter().position(|(word, _)| *word == text) {
        super::glyph_item(format!("word:{index}"), text, false)
    } else {
        super::glyph_item(format!("extra:{text}"), text, false)
    }
}

fn reads(indexes: &[usize]) -> Vec<Item> {
    indexes.iter().filter_map(|index| super::reading_item(*index)).collect()
}

fn plant_card(class_no: u8) -> Item {
    let lines = story(class_no);
    let questions = if class_no == 1 {
        vec![
            ask("कौन पानी देती है?", &["रीना", "दादी", "राम"], 0),
            ask("रीना किसे पानी देती है?", &["पौधे को", "घर को", "पुस्तक को"], 0),
            ask("रीना क्या करती है?", &["पानी देती है", "सोती है", "दौड़ती है"], 0),
        ]
    } else {
        vec![
            ask("पहले क्या हुआ?", &["रीना ने बीज बोया", "पौधा निकला", "रीना ने पानी दिया"], 0),
            ask("बीज के बाद क्या हुआ?", &["पौधा निकला", "रीना सो गई", "बारिश रुकी"], 0),
            ask("पानी क्यों?", &["पौधा हरा रहे", "पौधा सोए", "खेल के लिए"], 0),
        ]
    };
    Item {
        id: format!("card:plant{class_no}"),
        title: if class_no == 1 { "पौधा".to_string() } else { "बीज से पौधा".to_string() },
        speak: lines.join(" "),
        formation: false,
        kind: Kind::Reading { lines, questions },
    }
}

fn ask(prompt: &str, choices: &[&str], answer: usize) -> QuizQ {
    QuizQ {
        prompt: prompt.to_string(),
        choices: choices.iter().map(|choice| (*choice).to_string()).collect(),
        answer,
    }
}
