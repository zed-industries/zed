use elasticlunr::*;
use serde_json::json;
use std::fs::{self, File};
use std::path::Path;

fn create_index(lang: Box<dyn Language>, docs: &'static [[&'static str; 2]]) -> serde_json::Value {
    let mut index = Index::with_language(lang, &["title", "body"]);
    for (i, doc) in docs.iter().enumerate() {
        index.add_doc(&(i + 1).to_string(), doc);
    }
    json!(index)
}

fn generate_fixture(
    lang: Box<dyn Language>,
    docs: &'static [[&'static str; 2]],
) -> serde_json::Value {
    let code = lang.code();
    let src = create_index(lang, docs);
    let dest = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/searchindex_fixture_{}.json", code));
    let dest = File::create(&dest).unwrap();
    serde_json::to_writer_pretty(dest, &src).unwrap();
    src
}

fn read_fixture(lang: &dyn Language) -> serde_json::Value {
    let src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/searchindex_fixture_{}.json", lang.code()));
    let json = fs::read_to_string(src).unwrap();
    serde_json::from_str(&json).expect("Unable to deserialize the fixture")
}

const GENERATE_FIXTURE: bool = false;

fn check_index<L: Language + Clone + 'static>(lang: L, docs: &'static [[&'static str; 2]]) {
    let new_index = create_index(Box::new(lang.clone()), docs);
    let name = lang.name();
    let fixture_index = if GENERATE_FIXTURE {
        generate_fixture(Box::new(lang), docs)
    } else {
        read_fixture(&lang)
    };
    if new_index != fixture_index {
        panic!("The {} search index has changed from the fixture", name);
    }
}

#[test]
fn en_search_index_hasnt_changed_accidentally() {
    check_index(lang::English::new(), DOCS_EN);
}

#[cfg(feature = "ja")]
#[test]
fn ja_search_index_hasnt_changed_accidentally() {
    check_index(lang::Japanese::new(), DOCS_JA);
}

const DOCS_EN: &[[&str; 2]] = &[
    [
        "Chapter 1",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
    ],
    [
        "Chapter 2",
        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad",
    ],
    [
        "Chapter 3",
        "minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex",
    ],
    [
        "Chapter 4",
        "ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate",
    ],
    [
        "Chapter 5",
        "velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat",
    ],
    ["Chapter 6", "Spatiëring shouldn’t cause a panic."],
];

#[cfg(feature = "ja")]
const DOCS_JA: &'static [[&'static str; 2]] = &[
    [
        "第1章",
        "吾輩は猫である。名前はまだ無い。",
    ],
    [
        "第2章",
        "どこで生れたかとんと見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。",
    ],
    [
        "第3章",
        "吾輩はここで始めて人間というものを見た。しかもあとで聞くとそれは書生という人間中で一番獰悪な種族であったそうだ。この書生というのは時々我々を捕えて煮て食うという話である。しかしその当時は何という考もなかったから別段恐しいとも思わなかった。ただ彼の掌に載せられてスーと持ち上げられた時何だかフワフワした感じがあったばかりである。掌の上で少し落ちついて書生の顔を見たのがいわゆる人間というものの見始であろう。この時妙なものだと思った感じが今でも残っている。",
    ],
    [
        "第4章",
        "第一毛をもって装飾されべきはずの顔がつるつるしてまるで薬缶だ。その後猫にもだいぶ逢ったがこんな片輪には一度も出会わした事がない。のみならず顔の真中があまりに突起している。",
    ],
];
