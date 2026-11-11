# Saksham

Saksham (सक्षम) is a calm Hindi learning app for a tablet. A child in class 1 or class 2 opens one subject, picks a goal, and works through short listen, read, and write activities. Stars and stickers mark what is finished. A parent gate keeps sound, finger writing, and the screen lock out of the child’s reach.

The interface is in Hindi. English and गणित appear on the home screen and stay closed.

There is no account and no server. Progress stays in the browser that opened the app.

## What the child does

The home screen has three doors and a reward board.

| Door | State |
| --- | --- |
| हिन्दी | Open. Class 1 and class 2. |
| English | Closed. Labelled बाद में. |
| गणित | Closed. Labelled बाद में. |

हिन्दी leads to a class, then to five goals. Each goal offers **सीखो** (learn), **अभ्यास** (practice), and **परीक्षा** (a short test).

| Goal | Class 1 aim | Class 2 aim |
| --- | --- | --- |
| सुनना और बोलना | Hear a familiar line and tap an easy answer. | Hear a new sequence, put events in order, and pick a reason. |
| वर्ण और मात्राएँ | Match a letter sound and read words such as किताब and माला. | Read longer words and conjuncts such as बच्चा and बिल्ली. |
| प्रवाह से पढ़ना | Read one short sentence and stop at the danda. | Read three sentences and keep their order. |
| पढ़कर समझना | Say who is in the picture and what happened. | Order pictures and choose the meaning. |
| लिखना | Write a letter, a word, and one sentence. Guide dots start dark and fade. | Write a longer word and joined sentences. Dots fade more slowly. |

Class 1 is built around one sentence: रीना पौधे को पानी देती है. Class 2 uses three lines: रीना ने बीज बोया, पौधा निकला, रीना पौधे को पानी देती है.

A button labelled **आज का अभ्यास** sits under the top bar when there is something to review. It opens that item in practice mode.

The top bar shows the star total, whether the lock is on, and an आवाज़ / शांत switch.

## A lesson

Lessons are fixed content in `src/content/`. Nothing is fetched at runtime.

- **रेखा.** Stroke patterns: standing, sleeping, and slanted lines, plus the curve used for उ.
- **स्वर and व्यंजन.** The vowel and consonant rows. A few consonants are marked for formation practice.
- **मात्रा.** Barakhadi for क, प, म, and य in class 1. Class 2 adds conjuncts (क्ष, त्र, ज्ञ) and words such as बच्चा, बिल्ली, and क्षमा.
- **शब्द.** Short class 1 words (घर, जल, फल, माला, आम, पिता, किताब, पानी) and longer class 2 words (तालाब, चींटी, मूली, मेंढक, बैंगनी).
- **पढ़ो.** Short readings, plus the plant card for the class story.

Learn mode shows the model and speaks it, then a light trace. Practice mode traces with fading dots and then asks three questions. Reading items start by listening, then ask about the passage. A pattern item is three strokes instead of a quiz.

The exam for a goal is five questions. The writing exam also asks the child to copy the class sentence (class 1) or पौधा निकला (class 2). Two correct answers are enough to pass. Fewer than that ends on फिर एक बार, and the same questions can be tried again.

Speech uses the browser speech API in `hi-IN` at a slightly slow rate. The child can tap सुनो to hear a glyph again. Turning sound off on the home screen, or in the parent screen, stops new utterances.

## Writing

The slate is an 8 mm ruled band. Guide dots are sampled from the glyph. On later rounds only every other dot remains, so the line has to be remembered.

A stroke passes when all of these are true:

- at least six points were drawn
- at least 60% of the guide dots were touched (within 7 px)
- no more than half of the ink sits far from the guide (beyond 14 px)

Missed dots are marked so the next try can cover them. A moving dot can travel the guide before the child draws.

Pen and mouse are always accepted. A finger is accepted only after a parent turns on **उंगली से लिखना**. That default keeps an accidental palm from drawing.

## Stars and stickers

Points are awarded from how many answers were right:

| Correct | Stars |
| --- | --- |
| 3 | 5 |
| 2 | 3 |
| 0 or 1 | 0, and the item is due again immediately |

A passed item is scheduled again after 1 day, then 3 days, then 7 days. **आज का अभ्यास** picks the most overdue item, otherwise something never practiced, otherwise the item due soonest.

Crossing a multiple of 10 stars offers two stickers the child does not already have: तारा, चाँद, फूल, पक्षी, मछली, पतंग, पेड़, बादल, पत्ता, नाव. Collected pictures are on the इनाम board.

The last 50 awards are kept. Older ones are dropped.

## Parents

Press and hold the title सक्षम for a little more than half a second. That opens the parent gate. The child-facing buttons do not.

The first visit asks for a four-digit PIN twice. Later visits ask for that PIN once. A wrong code clears the pad and says फिर लिखो. The PIN is stored in the same browser record as the stars. It only keeps a child out of settings. It is not an account password.

After the gate opens, a parent can:

- turn आवाज़ on or off
- allow or block finger writing
- turn the ताला on or off

In a browser, the lock asks for fullscreen and hides the rest of the page. On Android, the same switch calls `startLockTask` / `stopLockTask`. Pinning only holds if App pinning is enabled in Android settings and “Ask for PIN before unpinning” is selected. The parent screen says this in Hindi.

## Run it

You need a recent Rust toolchain (edition 2021) and the Dioxus 0.7 CLI:

```bash
cargo install dioxus-cli
dx serve
```

`dx serve` follows `Dioxus.toml` and serves the web app. Open the URL it prints. Use a tablet-sized window, or a real tablet browser. Hindi speech needs a browser that supports `speechSynthesis` with an `hi-IN` voice.

Other platforms are Cargo features, not the default:

```bash
dx serve --platform desktop
dx serve --platform android
```

Desktop uses the `desktop` feature. Android uses the `mobile` feature and also needs the Android SDK. The Android build is what makes the screen-pinning lock call into the activity.

A release web build writes to `dist/`:

```bash
dx build --release
```

`target/` and `dist/` are gitignored.

## Progress

On the web build, the whole record is one JSON value in `localStorage` under `saksham.points.v1`. It holds stars, award history, stickers, the parent PIN, lock, sound, finger writing, and the review schedule. Closing the tab does not clear it. Clearing site data does.

Desktop and Android builds currently start from an empty record. Save and load run only in the wasm build.

## Layout

```
src/main.rs            app shell and screen switch
src/state.rs           which screen is open
src/content/           lessons: letters, words, readings, goals, exams
src/session.rs         learn and practice flow
src/exam.rs            the five-question paper for one goal
src/ui/                home, class menus, writing slate, rewards, parent gate
src/points.rs          stars, stickers, review schedule, browser storage
src/score.rs           stroke coverage
src/speech.rs          Hindi speech
src/lock.rs            fullscreen in the browser, screen pinning on Android
assets/app.css         paper, ink, and the 8 mm writing band
assets/fonts/          Noto Sans Devanagari, used as Saksham Devanagari
assets/icons/          Lucide icons (ISC)
```

`books/` and `practice_books/` may sit next to the project as local PDFs. The app does not open them, and they are not part of the git history.

`Cargo.lock` is gitignored, so a fresh clone resolves dependencies again.

## Add a lesson

Items are looked up by id in `src/content/mod.rs`:

| Prefix | Content |
| --- | --- |
| `pat:N` | Stroke pattern |
| `swar:N` | Vowel |
| `vyn:N` | Consonant |
| `bar:C:F` | Barakhadi form of consonant C |
| `word:N` | Word from the word list |
| `read:N` | Reading passage |
| `card:plant1`, `card:plant2` | Class story |
| `extra:…` | A word or sentence that is not in the word list |

Each item is a pattern (a list of dots), a glyph (one string to show, speak, and trace), or a reading (lines plus questions). `questions_for` builds three choices for glyphs and patterns from neighbouring items. Readings use the questions stored with the passage.

Class banks and exam papers are assembled in `src/content/catalog.rs` (`bank`, `paper`, `story`). A new class 1 or class 2 activity belongs there, with a stable id so the review schedule can find it again.
