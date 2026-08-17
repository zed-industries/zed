// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::ToOwned;
use std::collections::hash_map::{Entry, HashMap};

use tendril::StrTendril;

fn index_words_string(input: &String) -> HashMap<char, Vec<String>> {
    let mut index = HashMap::new();
    for word in input.split(|c| c == ' ') {
        if word.len() == 0 {
            continue;
        }
        let word = word.to_owned();
        match index.entry(word.chars().next().unwrap()) {
            Entry::Occupied(mut e) => {
                let x: &mut Vec<String> = e.get_mut();
                x.push(word);
            }
            Entry::Vacant(e) => {
                e.insert(vec![word]);
            }
        }
    }
    index
}

fn index_words_tendril(input: &StrTendril) -> HashMap<char, Vec<StrTendril>> {
    let mut index = HashMap::new();
    let mut t = input.clone();
    loop {
        match t.pop_front_char_run(|c| c != ' ') {
            None => return index,
            Some((_, false)) => (),
            Some((word, true)) => match index.entry(word.chars().next().unwrap()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(word);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![word]);
                }
            },
        }
    }
}

static EN_1: &'static str = "Days turn to nights turn to paper into rocks into plastic";

static EN_2: &'static str =
    "Here the notes in my laboratory journal cease. I was able to write the last \
       words only with great effort. By now it was already clear to me that LSD had \
       been the cause of the remarkable experience of the previous Friday, for the \
       altered perceptions were of the same type as before, only much more intense. I \
       had to struggle to speak intelligibly. I asked my laboratory assistant, who was \
       informed of the self-experiment, to escort me home. We went by bicycle, no \
       automobile being available because of wartime restrictions on their use. On the \
       way home, my condition began to assume threatening forms. Everything in my \
       field of vision wavered and was distorted as if seen in a curved mirror. I also \
       had the sensation of being unable to move from the spot. Nevertheless, my \
       assistant later told me that we had traveled very rapidly. Finally, we arrived \
       at home safe and sound, and I was just barely capable of asking my companion to \
       summon our family doctor and request milk from the neighbors.\n\n\
       In spite of my delirious, bewildered condition, I had brief periods of clear \
       and effective thinking—and chose milk as a nonspecific antidote for poisoning.";

static KR_1: &'static str =
    "러스트(Rust)는 모질라(mozilla.org)에서 개발하고 있는, 메모리-안전하고 병렬 \
       프로그래밍이 쉬운 차세대 프로그래밍 언어입니다. 아직 \
       개발 단계이며 많은 기능이 구현 중으로, MIT/Apache2 라이선스로 배포됩니다.";

static HTML_KR_1: &'static str =
    "<p>러스트(<a href=\"http://rust-lang.org\">Rust</a>)는 모질라(<a href=\"\
       https://www.mozilla.org/\">mozilla.org</a>)에서 개발하고 있는, \
       메모리-안전하고 병렬 프로그래밍이 쉬운 차세대 프로그래밍 언어입니다. \
       아직 개발 단계이며 많은 기능이 구현 중으로, MIT/Apache2 라이선스로 배포됩니다.</p>";

mod index_words {
    macro_rules! bench {
        ($txt:ident) => {
            #[allow(non_snake_case)]
            mod $txt {
                const SMALL_SIZE: usize = 65536;
                const LARGE_SIZE: usize = (1 << 20);

                #[bench]
                fn index_words_string(b: &mut ::test::Bencher) {
                    let mut s = String::new();
                    while s.len() < SMALL_SIZE {
                        s.push_str(::tendril::bench::$txt);
                    }
                    b.iter(|| ::tendril::bench::index_words_string(&s));
                }

                #[bench]
                fn index_words_tendril(b: &mut ::test::Bencher) {
                    let mut t = ::tendril::StrTendril::new();
                    while t.len() < SMALL_SIZE {
                        t.push_slice(::tendril::bench::$txt);
                    }
                    b.iter(|| ::tendril::bench::index_words_tendril(&t));
                }

                #[bench]
                fn index_words_big_string(b: &mut ::test::Bencher) {
                    let mut s = String::new();
                    while s.len() < LARGE_SIZE {
                        s.push_str(::tendril::bench::$txt);
                    }
                    b.iter(|| ::tendril::bench::index_words_string(&s));
                }

                #[bench]
                fn index_words_big_tendril(b: &mut ::test::Bencher) {
                    let mut t = ::tendril::StrTendril::new();
                    while t.len() < LARGE_SIZE {
                        t.push_slice(::tendril::bench::$txt);
                    }
                    b.iter(|| ::tendril::bench::index_words_tendril(&t));
                }

                #[test]
                fn correctness() {
                    use std::borrow::ToOwned;
                    use tendril::bench::{index_words_string, index_words_tendril};
                    use tendril::SliceExt;

                    let txt = ::tendril::bench::$txt;
                    let input_string = txt.to_owned();
                    let count_s = index_words_string(&input_string);
                    let mut keys: Vec<char> = count_s.keys().cloned().collect();
                    keys.sort();

                    let input_tendril = txt.to_tendril();
                    let count_t = index_words_tendril(&input_tendril);
                    let mut keys_t: Vec<char> = count_t.keys().cloned().collect();
                    keys_t.sort();

                    assert_eq!(keys, keys_t);

                    for k in &keys {
                        let vs = &count_s[k];
                        let vt = &count_t[k];
                        assert_eq!(vs.len(), vt.len());
                        assert!(vs.iter().zip(vt.iter()).all(|(s, t)| **s == **t));
                    }
                }
            }
        };
    }

    bench!(EN_1);
    bench!(EN_2);
    bench!(KR_1);
    bench!(HTML_KR_1);
}
